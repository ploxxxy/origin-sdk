use std::{
    collections::HashMap,
    error::Error,
    sync::{
        Arc,
        atomic::{AtomicU64, Ordering},
    },
    time::Duration,
};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{
        TcpStream,
        tcp::{OwnedReadHalf, OwnedWriteHalf},
    },
    sync::{
        Mutex,
        mpsc::{self},
        oneshot::{self},
    },
    task::JoinHandle,
};
use tracing::{debug, error, info};

use crate::{
    crypto::Crypto,
    protocol::{
        Event, EventBody, Lsx, Message, Request, RequestBody, RequestResponse, Response,
        ResponseBody, auth::ChallengeResponse,
    },
};

// TODO: error handling
pub type SdkError = Box<dyn Error + Send + Sync>;
type SdkResult<T> = Result<T, SdkError>;

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
    pub async fn connect(address: &str) -> SdkResult<(Self, mpsc::Receiver<Event>)> {
        let stream = TcpStream::connect(address).await?;
        let (read_half, write_half) = stream.into_split();

        let mut reader = BufReader::new(read_half);
        let mut writer = write_half;
        let mut crypto = Crypto::new(0);

        // Server requires a challenge/response authentication sequence
        // before normal requests can be sent.
        Self::perform_challenge(&mut reader, &mut writer, &mut crypto).await?;

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
        reader: &mut BufReader<OwnedReadHalf>,
        writer: &mut OwnedWriteHalf,
        crypto: &mut Crypto,
    ) -> SdkResult<()> {
        loop {
            let data = Self::read_message(reader).await?;

            if !data.is_empty() {
                let str = String::from_utf8_lossy(&data);
                let lsx: Lsx = quick_xml::de::from_str(&str)?;

                match lsx.message {
                    Message::Event(event) => {
                        // The server issues a cryptographic challenge, which we must answer using
                        // prepare_challenge_response Once the challenge is accepted,
                        // the session is considered authenticated
                        if let EventBody::Challenge(challenge) = event.body {
                            debug!("Challenge key: {}", challenge.key);

                            // Construct a challenge response payload with session metadata
                            // TODO: replace the hardcoded values with user input
                            let response_str = crypto.prepare_challenge_response(&challenge.key)?;
                            let challenge_response = ChallengeResponse {
                                content_id: "Origin.OFR.50.0001000".to_string(),
                                key: challenge.key.clone(),
                                response: response_str,
                                language: "en_US".to_string(),
                                multiplayer_id: "".to_string(),
                                protocol_version: "2".to_string(),
                                sdk_version: "9.10.1.7".to_string(),
                                title: "".to_string(),
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
                let str = String::from_utf8_lossy(&data);
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
                            return Err("Unexpected response to challenge".into());
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

                    let str = String::from_utf8_lossy(&data);
                    let Ok(buf) = hex::decode(str.as_ref()) else {
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
                            if let Err(e) = event_tx.send(event).await {
                                error!("Failed to send event: {}", e);
                            }
                        }
                        Message::Response(response) => {
                            if let Ok(id) = response.id.parse::<u64>() {
                                let mut pending = pending_requests.lock().await;
                                if let Some(tx) = pending.remove(&id) {
                                    let _ = tx.send(response);
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
    pub async fn send_request<T>(&self, body: T) -> SdkResult<T::Response>
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

        let mut writer = self.writer.lock().await;
        Self::send_raw(&mut writer, hex.into_bytes()).await?;

        match tokio::time::timeout(Duration::from_secs(5), rx).await {
            Ok(Ok(response)) => Ok(T::extract_response(response.body)?),
            Ok(Err(_)) => Err("Channel closed".into()),
            Err(_) => {
                self.pending_requests.lock().await.remove(&id);
                Err("Timeout".into())
            }
        }
    }
}
