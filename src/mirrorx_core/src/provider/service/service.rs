use super::{
    desktop::DesktopService,
    message::{
        reply::{ConnectReply, HeartBeatReply, KeyExchangeAndVerifyPasswordReply, RegisterIdReply},
        reply_error::ReplyError,
        request::{
            ConnectRequest, HeartBeatRequest, KeyExchangeAndVerifyPasswordRequest,
            RegisterIdRequest,
        },
    },
    network::client::Client,
};
use crate::{
    instance::{BINCODE_INSTANCE, RUNTIME_PROVIDER_INSTANCE},
    provider::service::{
        message::{reply::ReplyMessage, request::RequestMessage},
        network::packet::{Packet, ReplyPacket},
    },
};
use bincode::Options;
use bytes::Bytes;
use futures::{SinkExt, StreamExt};
use log::{error, info, warn};
use ring::agreement::EphemeralPrivateKey;
use rsa::{RsaPrivateKey, RsaPublicKey};
use std::{sync::Arc, time::Duration};
use tokio::{net::TcpStream, sync::mpsc};
use tokio_util::codec::LengthDelimitedCodec;

pub struct ServiceProvider {
    client: Arc<Client>,
}

impl ServiceProvider {
    pub fn new(addr: &str) -> anyhow::Result<Self> {
        let client = connect(addr)?;
        Ok(ServiceProvider { client })
    }

    pub async fn begin_heart_beat(&self) {
        let mut heart_beat_miss_counter = 0;
        let mut ticker = tokio::time::interval(Duration::from_secs(20));

        loop {
            if heart_beat_miss_counter >= 3 {
                error!("heart_beat: missed times >=3, break");
                return;
            }

            ticker.tick().await;

            let time_stamp_now = chrono::Utc::now().timestamp() as u32;

            let res = self
                .client
                .call(
                    RequestMessage::HeartBeatRequest(HeartBeatRequest {
                        time_stamp: chrono::Utc::now().timestamp() as u32,
                    }),
                    Duration::from_secs(10),
                )
                .await
                .and_then(|resp| match resp {
                    ReplyMessage::HeartBeatReply(message) => Ok(message),
                    _ => Err(ReplyError::CastFailed),
                });

            match res {
                Ok(resp) => {
                    if resp.time_stamp > time_stamp_now + 5 {
                        warn!("heart_beat: response received before deadline but inner timestamp is greater than deadline");
                    }
                }
                Err(err) => {
                    error!("heart_beat failed: {:?}", err);
                    heart_beat_miss_counter += 1;
                }
            }
        }
    }

    pub async fn device_register_id(
        &self,
        req: RegisterIdRequest,
        timeout: Duration,
    ) -> Result<RegisterIdReply, ReplyError> {
        self.client
            .call(RequestMessage::RegisterIdRequest(req), timeout)
            .await
            .and_then(|resp| match resp {
                ReplyMessage::RegisterIdReply(message) => Ok(message),
                _ => Err(ReplyError::CastFailed),
            })
    }

    pub async fn desktop_connect(
        &self,
        req: ConnectRequest,
        timeout: Duration,
    ) -> Result<ConnectReply, ReplyError> {
        self.client
            .call(RequestMessage::ConnectRequest(req), timeout)
            .await
            .and_then(|resp| match resp {
                ReplyMessage::ConnectReply(message) => Ok(message),
                _ => Err(ReplyError::CastFailed),
            })
    }

    pub async fn desktop_key_exchange_and_verify_password(
        &self,
        req: KeyExchangeAndVerifyPasswordRequest,
        timeout: Duration,
    ) -> Result<KeyExchangeAndVerifyPasswordReply, ReplyError> {
        self.client
            .call(
                RequestMessage::KeyExchangeAndVerifyPasswordRequest(req),
                timeout,
            )
            .await
            .and_then(|resp| match resp {
                ReplyMessage::KeyExchangeAndVerifyPasswordReply(message) => Ok(message),
                _ => Err(ReplyError::CastFailed),
            })
    }

    pub fn store_verify_password_pub_key(&self, device_id: String, pub_key: RsaPublicKey) {
        self.client
            .store_verify_password_pub_key(device_id, pub_key);
    }

    pub fn remove_verify_password_pub_key(&self, device_id: &str) -> Option<RsaPublicKey> {
        self.client.remove_verify_password_pub_key(device_id)
    }

    pub fn store_verify_password_priv_key(&self, device_id: String, priv_key: RsaPrivateKey) {
        self.client
            .store_verify_password_priv_key(device_id, priv_key);
    }

    pub fn remove_verify_password_priv_key(&self, device_id: &str) -> Option<RsaPrivateKey> {
        self.client.remove_verify_password_priv_key(device_id)
    }
}

fn connect(addr: &str) -> anyhow::Result<Arc<Client>> {
    let runtime = RUNTIME_PROVIDER_INSTANCE
        .get()
        .ok_or_else(|| anyhow::anyhow!("runtime not initialized"))?;

    runtime.block_on(async move {
        let stream = TcpStream::connect("192.168.0.101:45555").await?;

        let desktop_service = Arc::new(DesktopService::new());

        let (tx, rx) = mpsc::channel(16);
        let client = Arc::new(Client::new(tx));

        serve_stream(stream, client.clone(), rx, desktop_service).await?;

        Ok(client)
    })
}

async fn serve_stream(
    stream: TcpStream,
    client: Arc<Client>,
    mut client_rx: mpsc::Receiver<Vec<u8>>,
    desktop_service: Arc<DesktopService>,
) -> anyhow::Result<()> {
    let runtime = RUNTIME_PROVIDER_INSTANCE
        .get()
        .ok_or_else(|| anyhow::anyhow!("runtime not initialized"))?;

    let framed_stream = LengthDelimitedCodec::builder()
        .little_endian()
        .max_frame_length(16 * 1024 * 1024)
        .new_framed(stream);

    let (mut sink, mut stream) = framed_stream.split();

    runtime.spawn(async move {
        loop {
            let packet_bytes = tokio::select! {
                // _ = shutdown_notify_rx.recv() => break,
                res = stream.next() => match res {
                    Some(Ok(packet)) => packet,
                    Some(Err(err)) => match err.kind() {
                        std::io::ErrorKind::UnexpectedEof => {
                            info!("client: disconnected");
                            break;
                        },
                        std::io::ErrorKind::ConnectionReset => {
                            info!("client: connection reset");
                            break;
                        },
                        _ => {
                            error!("client: stream_loop read packet error: {:?}", err);
                            continue;
                        }
                    }
                    None => break,
                }
            };

            let packet = match BINCODE_INSTANCE.deserialize::<Packet>(&packet_bytes) {
                Ok(packet) => packet,
                Err(err) => {
                    error!("client: stream_loop deserialize packet error: {:?}", err);
                    continue;
                }
            };

            if let Some(request_packet) = packet.request_packet {
                let inner_desktop_service = desktop_service.clone();

                let client = client.clone();
                tokio::spawn(async move {
                    let res = match request_packet.payload {
                        RequestMessage::ConnectRequest(message) => inner_desktop_service
                            .connect(client.clone(), message)
                            .await
                            .map(|msg| ReplyMessage::ConnectReply(msg)),
                        _ => {
                            error!("unexpect message");
                            return;
                        }
                    };

                    let reply_packet = ReplyPacket {
                        call_id: request_packet.call_id,
                        payload: res,
                    };

                    if let Err(err) = client.reply_request(reply_packet).await {
                        error!("client reply_request failed: {:?}", err);
                    }
                });
            } else if let Some(reply_packet) = packet.reply_packet {
                client.reply_call(reply_packet.call_id, reply_packet);
            } else {
                warn!("client receive unknown packet");
            }
        }

        info!("client: stream_loop exit");
    });

    runtime.spawn(async move {
        loop {
            let buf = match client_rx.recv().await {
                Some(buf) => buf,
                None => break,
            };

            if let Err(err) = sink.send(Bytes::from(buf)).await {
                error!("send error: {:?}", err);
            }
        }
    });

    Ok(())
}
