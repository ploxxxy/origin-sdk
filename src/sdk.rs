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
        oneshot::{self, Sender},
    },
    task::JoinHandle,
};

use crate::{
    crypto::Crypto,
    protocol::{
        lsx::{Lsx, Message, Request, Response},
        message::{EventBody, RequestBody, ResponseBody},
        model::{ChallengeResponse, GetInternetConnectedState, InternetConnectedState},
    },
};

// TODO: error handling
type SdkError = Box<dyn Error + Send + Sync>;
type SdkResult<T> = Result<T, SdkError>;

type PendingRequests = Arc<Mutex<HashMap<u64, Sender<Response>>>>;

pub struct OriginSdk {
    writer: Arc<Mutex<OwnedWriteHalf>>,
    reader_handle: JoinHandle<()>,
    pending_requests: PendingRequests,
    next_id: AtomicU64,
    crypto: Crypto,
}

impl Drop for OriginSdk {
    fn drop(&mut self) {
        self.reader_handle.abort();
    }
}

impl OriginSdk {
    pub async fn connect(address: &str) -> SdkResult<Self> {
        let stream = TcpStream::connect(address).await?;
        let (read_half, write_half) = stream.into_split();

        let mut reader = BufReader::new(read_half);
        let mut writer = write_half;
        let mut crypto = Crypto::new(0);

        Self::perform_handshake(&mut reader, &mut writer, &mut crypto).await?;

        let pending_requests: PendingRequests = Arc::new(Mutex::new(HashMap::new()));

        let reader_handle = tokio::spawn(Self::reader_task(
            reader,
            pending_requests.clone(),
            crypto.clone(),
        ));

        Ok(OriginSdk {
            writer: Arc::new(Mutex::new(writer)),
            reader_handle,
            pending_requests,
            next_id: AtomicU64::new(1),
            crypto,
        })
    }

    async fn perform_handshake(
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
                        if let EventBody::Challenge(challenge) = event.body {
                            println!("Challenge key: {}", challenge.key);

                            let response_str = crypto.prepare_challenge_response(&challenge.key)?;
                            let challenge_response = ChallengeResponse {
                                content_id: "Origin.OFR.50.0005734".to_string(),
                                key: challenge.key.clone(),
                                response: response_str,
                                language: "en_US".to_string(),
                                multiplayer_id: "".to_string(),
                                protocol_version: "3".to_string(),
                                sdk_version: "9.10.1.7".to_string(),
                                title: "skate".to_string(),
                            };

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

    async fn wait_challenge_accepted(reader: &mut BufReader<OwnedReadHalf>) -> SdkResult<()> {
        loop {
            let data = Self::read_message(reader).await?;
            if !data.is_empty() {
                let str = String::from_utf8_lossy(&data);
                let lsx: Lsx = quick_xml::de::from_str(&str)?;

                match lsx.message {
                    Message::Response(response) => match response.body {
                        ResponseBody::ChallengeAccepted(body) => {
                            println!("Handshake successful!");

                            println!("Challenge response: {:#?}", body.response);

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
    ) {
        loop {
            match Self::read_message(&mut reader).await {
                Ok(data) => {
                    if data.is_empty() {
                        continue;
                    }

                    let str = String::from_utf8_lossy(&data);
                    let Ok(buf) = hex::decode(str.as_ref()) else {
                        println!("Failed to decode hex: {}", str);
                        continue;
                    };

                    let Ok(xml) = crypto.decrypt(&buf) else {
                        println!("Failed to decrypt");
                        continue;
                    };

                    let Ok(lsx): Result<Lsx, _> = quick_xml::de::from_str(&xml) else {
                        println!("Failed to parse XML: {}", xml);
                        continue;
                    };

                    match lsx.message {
                        Message::Event(event) => {
                            println!("Received event from {}", event.sender);
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
                    println!("Error reading message: {}", e);
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

    async fn send_request(&self, body: RequestBody, recipient: &str) -> SdkResult<ResponseBody> {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let (tx, rx) = oneshot::channel();

        {
            let mut pending = self.pending_requests.lock().await;
            pending.insert(id, tx);
        }

        let request = Request {
            recipient: recipient.to_string(),
            id: id.to_string(),
            body,
        };

        let lsx = Lsx {
            message: Message::Request(request),
        };

        let serialized = quick_xml::se::to_string(&lsx)?;
        let encrypted = self.crypto.encrypt(&serialized)?;
        let hex = hex::encode(&encrypted);

        let mut writer = self.writer.lock().await;
        Self::send_raw(&mut writer, hex.into_bytes()).await?;

        match tokio::time::timeout(Duration::from_secs(30), rx).await {
            Ok(Ok(response)) => Ok(response.body),
            Ok(Err(_)) => Err("Channel closed".into()),
            Err(_) => {
                let mut pending = self.pending_requests.lock().await;
                pending.remove(&id);
                Err("Timeout".into())
            }
        }
    }

    pub async fn get_internet_connected_state(&self) -> SdkResult<InternetConnectedState> {
        let request = GetInternetConnectedState {};
        let response = self
            .send_request(RequestBody::GetInternetConnectedState(request), "EbisuSDK")
            .await?;

        match response {
            ResponseBody::InternetConnectedState(state) => Ok(state),
            _ => Err("Unexpected response".into()),
        }
    }
}
