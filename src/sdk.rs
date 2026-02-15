use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
    time::Duration,
};
use thiserror::Error;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{
        tcp::{OwnedReadHalf, OwnedWriteHalf},
        TcpStream,
    },
    sync::{
        mpsc::{self},
        oneshot::{self},
        Mutex,
    },
    task::JoinHandle,
};
use tracing::{debug, error, info, warn};

use crate::{
    crypto::Crypto,
    protocol::{
        auth::ChallengeResponse, errors::OriginError, Event, EventBody, Lsx, Message, Request,
        RequestBody, RequestResponse, Response, ResponseBody,
    },
};

/// Default port for the Origin SDK
pub const ORIGIN_SDK_PORT: u16 = 3216;

#[derive(Error, Debug)]
pub enum SdkError {
    #[error("{0:?}: {1}")]
    OriginError(OriginError, String),

    #[error("Network error: {0}")]
    Network(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] quick_xml::SeError),

    #[error("Deserialization error: {0}")]
    Deserialization(#[from] quick_xml::DeError),

    #[error("Crypto error: {0}")]
    Crypto(#[from] crate::crypto::CryptoError),

    #[error("{0}")]
    Other(String),
}

type SdkResult<T> = Result<T, SdkError>;

/// Configuration for the Origin SDK client
pub struct ClientConfig {
    /// Can be contentId, masterTitleId, or offerId
    pub content_id: String,
    pub language: String,
    pub multiplayer_id: String,
    pub title: String,
    pub version_override: Option<String>,
}

/// Shared state for tracking requests that are awaiting responses
///
/// Each request is associated with a unique numeric ID and a oneshot sender
/// through which the response will be delivered
type PendingRequests = Arc<Mutex<HashMap<u64, oneshot::Sender<Response>>>>;

/// The client for interacting with the Origin SDK protocol.
pub struct OriginSdk {
    /// Shared handle for writing messages to the server
    writer: Arc<Mutex<OwnedWriteHalf>>,
    /// Background task that continiously reads messages from the server
    reader_handle: JoinHandle<()>,
    /// Pending requests waiting for server responses
    pending_requests: PendingRequests,
    next_id: AtomicU64,
    crypto: Crypto,
}

impl Drop for OriginSdk {
    fn drop(&mut self) {
        // Abort the background reader task when the client is dropped
        self.reader_handle.abort();
    }
}

impl OriginSdk {
    /// Establish a connection to the Origin SDK server
    ///
    /// Returns a tuple of:
    /// - The [`OriginSdk`] client instance
    /// - An [`mpsc::Receiver`] for incoming [`Event`]s
    pub async fn connect(
        config: ClientConfig,
        port: u16,
    ) -> SdkResult<(Self, mpsc::Receiver<Event>)> {
        let stream = TcpStream::connect(("127.0.0.1", port)).await?;
        let (read_half, write_half) = stream.into_split();

        let mut reader = BufReader::new(read_half);
        let mut writer = write_half;
        let mut crypto = Crypto::new(0);

        // Server requires a challenge/response authentication sequence
        // before normal requests can be sent.
        Self::perform_challenge(config, &mut reader, &mut writer, &mut crypto).await?;

        let pending_requests: PendingRequests = Arc::new(Mutex::new(HashMap::new()));

        let (event_tx, event_rx) = mpsc::channel(100);

        // Spawn the background reader lopp
        let reader_handle = tokio::spawn(Self::reader_task(
            reader,
            pending_requests.clone(),
            crypto.clone(),
            event_tx,
        ));

        let sdk = OriginSdk {
            writer: Arc::new(Mutex::new(writer)),
            reader_handle,
            pending_requests,
            next_id: AtomicU64::new(1),
            crypto,
        };

        Ok((sdk, event_rx))
    }

    async fn perform_challenge(
        config: ClientConfig,
        reader: &mut BufReader<OwnedReadHalf>,
        writer: &mut OwnedWriteHalf,
        crypto: &mut Crypto,
    ) -> SdkResult<()> {
        loop {
            let data = Self::read_message(reader).await?;

            if !data.is_empty() {
                let str = match String::from_utf8(data) {
                    Ok(str) => str,
                    Err(err) => {
                        error!("Failed to decode UTF-8: {}", err);
                        continue;
                    }
                };

                let lsx: Lsx = quick_xml::de::from_str(&str)?;

                match lsx.message {
                    Message::Event(event) => {
                        // The server issues a cryptographic challenge, which we must answer using
                        // prepare_challenge_response Once the challenge is accepted,
                        // the session is considered authenticated
                        if let EventBody::Challenge(challenge) = event.body {
                            debug!("Challenge key: {}", challenge.key);

                            let sdk_version = match config.version_override {
                                Some(version) => version,
                                None => "10.6.1.8".to_string(),
                            };

                            // Construct a challenge response payload with session metadata
                            let response_str = crypto.prepare_challenge_response(&challenge.key)?;
                            let challenge_response = ChallengeResponse {
                                content_id: config.content_id,
                                key: challenge.key.clone(),
                                response: response_str,
                                language: config.language,
                                multiplayer_id: config.multiplayer_id,
                                // Protocol versions other than 3 always use the default encryption
                                // key ([0, 1, 2 ... 15]) for the requests. Should be implemented?
                                protocol_version: "3".to_string(),
                                sdk_version,
                                title: config.title,
                            };

                            // Send the challenge response and wait for it to be accepted
                            Self::send_challenge_response(writer, challenge_response).await?;
                            Self::wait_challenge_accepted(reader).await?;

                            return Ok(());
                        }
                    }
                    _ => {
                        continue;
                    }
                }
            }
        }
    }

    async fn send_challenge_response(
        writer: &mut OwnedWriteHalf,
        challenge_response: ChallengeResponse,
    ) -> SdkResult<()> {
        let request = Request {
            recipient: "EALS".to_string(),
            id: "0".to_string(),
            body: RequestBody::ChallengeResponse(challenge_response),
        };

        let lsx = Lsx {
            message: Message::Request(request),
        };

        let serialized = quick_xml::se::to_string(&lsx)?;
        Self::send_raw(writer, serialized.into_bytes()).await?;

        Ok(())
    }

    /// Wait for the server to confirm the challenge was accepted
    async fn wait_challenge_accepted(reader: &mut BufReader<OwnedReadHalf>) -> SdkResult<()> {
        loop {
            let data = Self::read_message(reader).await?;
            if !data.is_empty() {
                let str = match String::from_utf8(data) {
                    Ok(str) => str,
                    Err(err) => {
                        error!("Failed to decode UTF-8: {}", err);
                        continue;
                    }
                };

                let lsx: Lsx = quick_xml::de::from_str(&str)?;

                // If the server responds with a mismatched or an unexpected message,
                // the connection attempt fails
                match lsx.message {
                    Message::Response(response) => match response.body {
                        ResponseBody::ChallengeAccepted(body) => {
                            info!("Challenge accepted");
                            debug!("Received challenge response: {:#?}", body.response);

                            // TODO: check if server response key matches our response key

                            return Ok(());
                        }
                        _ => {
                            return Err(SdkError::Other(
                                "Unexpected response to challenge".to_string(),
                            ));
                        }
                    },
                    _ => {
                        continue;
                    }
                }
            }
        }
    }

    async fn reader_task(
        mut reader: BufReader<OwnedReadHalf>,
        pending_requests: PendingRequests,
        crypto: Crypto,
        event_tx: mpsc::Sender<Event>,
    ) {
        loop {
            match Self::read_message(&mut reader).await {
                Ok(data) => {
                    if data.is_empty() {
                        continue;
                    }

                    let str = match String::from_utf8(data) {
                        Ok(str) => str,
                        Err(err) => {
                            error!("Failed to decode UTF-8: {}", err);
                            continue;
                        }
                    };

                    let Ok(buf) = hex::decode(&str) else {
                        error!("Failed to decode hex: {}", str);
                        continue;
                    };

                    let Ok(xml) = crypto.decrypt(&buf) else {
                        error!("Failed to decrypt");
                        continue;
                    };

                    let lsx: Lsx = match quick_xml::de::from_str(&xml) {
                        Ok(lsx) => lsx,
                        Err(err) => {
                            error!("Failed to parse XML: {}\nError: {}", xml, err);
                            continue;
                        }
                    };

                    // Events are forwarded through the event channel
                    // Responses are matched against their pending IDs
                    match lsx.message {
                        Message::Event(event) => {
                            if event_tx.is_closed() {
                                continue;
                            }

                            if let Err(e) = event_tx.send(event).await {
                                warn!("Failed to send event: {}", e);
                            }
                        }
                        Message::Response(response) => {
                            if let Ok(id) = response.id.parse::<u64>() {
                                let mut pending = pending_requests.lock().await;
                                if let Some(tx) = pending.remove(&id) {
                                    let _ = tx.send(response);
                                } else {
                                    warn!("Received response for unknown request ID: {}", id);
                                }
                            }
                        }
                        _ => {}
                    }
                }
                Err(e) => {
                    error!("Error reading message: {}", e);
                    break;
                }
            }
        }
    }

    async fn read_message(reader: &mut BufReader<OwnedReadHalf>) -> SdkResult<Vec<u8>> {
        let mut buf = Vec::new();

        reader.read_until(0x00, &mut buf).await?;
        buf.pop();

        Ok(buf)
    }

    async fn send_raw(writer: &mut OwnedWriteHalf, mut bytes: Vec<u8>) -> SdkResult<()> {
        bytes.push(0x00);
        writer.write_all(&bytes).await?;
        writer.flush().await?;

        Ok(())
    }

    #[deprecated = "use `request()` instead"]
    pub async fn send_request<T>(&self, body: T) -> SdkResult<T::Response>
    where
        T: RequestResponse + Into<RequestBody>,
    {
        self.request(body).await
    }

    /// Send a typed request to the server and await its response.
    ///
    /// ## How requests & responses are defined
    /// All request types are declared under [`crate::protocol`] and grouped by domain
    /// (`achievements.rs`, `auth.rs`, `chat.rs`, etc). Each request type implements
    /// [`RequestResponse`], which is generated by the [`crate::request_response!`] macro.
    ///
    /// This macro wires requests to their corresponding response body variants
    /// in [`ResponseBody`]. For example:
    ///
    /// ```rust,ignore
    /// pub enum RequestBody {
    ///     // ...
    ///     GetProfile(GetProfile)
    ///     // ...
    /// }
    ///
    /// pub enum ResponseBody {
    ///     // ...
    ///     GetProfileResponse(GetProfileResponse),
    ///     // ...
    /// }
    ///
    /// request_response! {
    ///     // ...
    ///     GetProfile => GetProfileResponse,
    ///     // ...
    /// }
    /// ```
    ///
    /// This means calling:
    ///
    /// ```rust,ignore
    /// let profile: GetProfileResponse = client.send_request(GetProfile { /* ... */ }).await?;
    /// ```
    ///
    /// automatically deserializes the server response into the correct type.
    pub async fn request<T>(&self, body: T) -> SdkResult<T::Response>
    where
        T: RequestResponse + Into<RequestBody>,
    {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let (tx, rx) = oneshot::channel();

        {
            let mut pending = self.pending_requests.lock().await;
            pending.insert(id, tx);
        }

        // In the actual implementation, OriginSDK keeps a map of
        // Facility -> Recipient that comes from GetConfig request
        //
        // This isn't implemented here as EA Desktop
        // uses "EbisuSDK" for all its services
        let request = Request {
            recipient: "EbisuSDK".to_string(),
            id: id.to_string(),
            body: body.into(),
        };

        let lsx = Lsx {
            message: Message::Request(request),
        };

        let serialized = quick_xml::se::to_string(&lsx)?;
        let encrypted = self.crypto.encrypt(&serialized)?;
        let hex = hex::encode(&encrypted);

        {
            let mut writer = self.writer.lock().await;
            Self::send_raw(&mut writer, hex.into_bytes()).await?;
        }

        match tokio::time::timeout(Duration::from_secs(5), rx).await {
            Ok(Ok(response)) => Ok(T::extract_response(response.body)?),
            Ok(Err(_)) => Err(SdkError::Other("Channel closed".to_string())),
            Err(_) => {
                self.pending_requests.lock().await.remove(&id);
                Err(SdkError::Other("Timeout".to_string()))
            }
        }
    }

    pub async fn request_unknown(&self, body: RequestBody) -> SdkResult<ResponseBody> {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let (tx, rx) = oneshot::channel();

        {
            let mut pending = self.pending_requests.lock().await;
            pending.insert(id, tx);
        }

        let request = Request {
            recipient: "EbisuSDK".to_string(),
            id: id.to_string(),
            body,
        };

        let lsx = Lsx {
            message: Message::Request(request),
        };

        let serialized = quick_xml::se::to_string(&lsx)?;
        let encrypted = self.crypto.encrypt(&serialized)?;
        let hex = hex::encode(&encrypted);

        {
            let mut writer = self.writer.lock().await;
            Self::send_raw(&mut writer, hex.into_bytes()).await?;
        }

        match tokio::time::timeout(Duration::from_secs(5), rx).await {
            Ok(Ok(response)) => Ok(response.body),
            Ok(Err(_)) => Err(SdkError::Other("Channel closed".to_string())),
            Err(_) => {
                self.pending_requests.lock().await.remove(&id);
                Err(SdkError::Other("Timeout".to_string()))
            }
        }
    }
}
