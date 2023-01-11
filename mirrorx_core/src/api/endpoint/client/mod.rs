mod tcp;
mod udp;

use self::{tcp::serve_tcp, udp::serve_udp};
use super::{
    handlers::negotiate_desktop_params::handle_negotiate_desktop_params_request, id::EndPointID,
    message::*, EndPointStream,
};
use crate::{
    api::endpoint::handlers::{
        fs_download_file::handle_download_file_request, fs_send_file::handle_send_file_request,
        fs_visit_directory::handle_visit_directory_request, input::handle_input,
        negotiate_finished::handle_negotiate_finished_request,
    },
    call,
    component::{
        desktop::monitor::Monitor,
        fs::transfer::{append_file_block, delete_file_append_session},
    },
    core_error,
    error::{CoreError, CoreResult},
    utility::{
        bincode::{bincode_deserialize, bincode_serialize},
        nonce_value::NonceValue,
    },
};
use bytes::Bytes;
use ring::aead::{OpeningKey, SealingKey};
use scopeguard::defer;
use serde::de::DeserializeOwned;
use std::{
    fmt::Display,
    ops::Deref,
    sync::{atomic::AtomicU16, Arc},
    time::Duration,
};
use tokio::sync::{mpsc::Sender, RwLock};

const RECV_MESSAGE_TIMEOUT: Duration = Duration::from_secs(30);

#[derive(Debug, Clone)]
pub struct EndPointClient {
    endpoint_id: EndPointID,
    monitor: Arc<RwLock<Option<Arc<Monitor>>>>,
    tx: Sender<Vec<u8>>,
    call_id: Arc<AtomicU16>,
    call_store: Arc<moka::sync::Cache<u16, Sender<Vec<u8>>>>,
}

impl EndPointClient {
    pub async fn new_desktop_active(
        endpoint_id: EndPointID,
        stream_key: Option<(OpeningKey<NonceValue>, SealingKey<NonceValue>)>,
        stream: EndPointStream,
        video_frame_tx: Sender<EndPointVideoFrame>,
        audio_frame_tx: Sender<EndPointAudioFrame>,
        visit_credentials: Option<Vec<u8>>,
    ) -> CoreResult<Arc<EndPointClient>> {
        EndPointClient::create(
            true,
            endpoint_id,
            stream_key,
            stream,
            Some(video_frame_tx),
            Some(audio_frame_tx),
            visit_credentials,
        )
        .await
    }

    pub async fn new_file_manager_active(
        endpoint_id: EndPointID,
        stream_key: Option<(OpeningKey<NonceValue>, SealingKey<NonceValue>)>,
        stream: EndPointStream,
        visit_credentials: Option<Vec<u8>>,
    ) -> CoreResult<Arc<EndPointClient>> {
        EndPointClient::create(
            true,
            endpoint_id,
            stream_key,
            stream,
            None,
            None,
            visit_credentials,
        )
        .await
    }

    pub async fn new_passive(
        endpoint_id: EndPointID,
        key_pair: Option<(OpeningKey<NonceValue>, SealingKey<NonceValue>)>,
        stream: EndPointStream,
        visit_credentials: Option<Vec<u8>>,
    ) -> CoreResult<()> {
        let _ = EndPointClient::create(
            false,
            endpoint_id,
            key_pair,
            stream,
            None,
            None,
            visit_credentials,
        )
        .await?;
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    async fn create(
        active: bool,
        endpoint_id: EndPointID,
        key_pair: Option<(OpeningKey<NonceValue>, SealingKey<NonceValue>)>,
        stream: EndPointStream,
        video_frame_tx: Option<Sender<EndPointVideoFrame>>,
        audio_frame_tx: Option<Sender<EndPointAudioFrame>>,
        visit_credentials: Option<Vec<u8>>,
    ) -> CoreResult<Arc<EndPointClient>> {
        let (opening_key, sealing_key) = match key_pair {
            Some((opening_key, sealing_key)) => (Some(opening_key), Some(sealing_key)),
            None => (None, None),
        };

        let (tx, mut rx) = match stream {
            EndPointStream::ActiveTCP(addr) => {
                let stream = tokio::time::timeout(
                    Duration::from_secs(10),
                    tokio::net::TcpStream::connect(addr),
                )
                .await
                .map_err(|_| CoreError::Timeout)??;

                serve_tcp(
                    stream,
                    endpoint_id,
                    sealing_key,
                    opening_key,
                    visit_credentials,
                )
                .await?
            }
            EndPointStream::ActiveUDP(_) => panic!("not support yet"),
            EndPointStream::PassiveTCP(stream) => {
                serve_tcp(
                    stream,
                    endpoint_id,
                    sealing_key,
                    opening_key,
                    visit_credentials,
                )
                .await?
            }
            EndPointStream::PassiveUDP { socket, .. } => {
                serve_udp(
                    socket,
                    endpoint_id,
                    sealing_key,
                    opening_key,
                    visit_credentials,
                )
                .await?
            }
        };

        // active endpoint should start negotiate with passive endpoint
        let primary_monitor = if active && video_frame_tx.is_some() && audio_frame_tx.is_some() {
            let params = serve_active_negotiate(&tx, &mut rx).await?;
            Some(Arc::new(params.primary_monitor))
        } else {
            None
        };

        let call_store = moka::sync::CacheBuilder::new(32)
            .time_to_live(Duration::from_secs(60))
            .build();

        let client = Arc::new(EndPointClient {
            endpoint_id,
            monitor: Arc::new(RwLock::new(primary_monitor)),
            tx,
            call_id: Arc::new(AtomicU16::new(0)),
            call_store: Arc::new(call_store),
        });

        handle_message(client.clone(), rx, video_frame_tx, audio_frame_tx);

        Ok(client)
    }
}

impl EndPointClient {
    pub async fn monitor(&self) -> Option<Arc<Monitor>> {
        (*self.monitor.read().await).clone()
    }

    pub async fn set_monitor(&self, monitor: Monitor) {
        (*self.monitor.write().await) = Some(Arc::new(monitor))
    }
}

impl EndPointClient {
    pub fn try_send(&self, message: &EndPointMessage) -> CoreResult<()> {
        let buffer = bincode_serialize(message)?;
        self.tx
            .try_send(buffer)
            .map_err(|_| CoreError::OutgoingMessageChannelDisconnect)
    }

    pub fn blocking_send(&self, message: &EndPointMessage) -> CoreResult<()> {
        let buffer = bincode_serialize(message)?;
        self.tx
            .blocking_send(buffer)
            .map_err(|_| CoreError::OutgoingMessageChannelDisconnect)
    }

    pub async fn send(&self, message: &EndPointMessage) -> CoreResult<()> {
        let buffer = bincode_serialize(message)?;
        self.tx
            .send(buffer)
            .await
            .map_err(|_| CoreError::OutgoingMessageChannelDisconnect)
    }

    pub async fn call<TReply>(&self, message: EndPointCallRequest) -> CoreResult<TReply>
    where
        TReply: DeserializeOwned,
    {
        let call_id = self
            .call_id
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);

        let (tx, mut rx) = tokio::sync::mpsc::channel(1);

        self.call_store.insert(call_id, tx);
        defer! {
            self.call_store.invalidate(&call_id);
        }

        self.send(&EndPointMessage::CallRequest(call_id, message))
            .await?;

        let reply_bytes = rx.recv().await.ok_or(CoreError::Timeout)?;

        bincode_deserialize::<Result<TReply, String>>(&reply_bytes)?
            .map_err(|err_str| core_error!("{}", err_str))
    }
}

impl Display for EndPointClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "EndPointClient({})", self.endpoint_id)
    }
}

async fn serve_active_negotiate(
    tx: &Sender<Vec<u8>>,
    rx: &mut tokio::sync::mpsc::Receiver<Bytes>,
) -> CoreResult<EndPointNegotiateVisitDesktopParams> {
    let negotiate_request_buffer = bincode_serialize(
        &EndPointMessage::NegotiateDesktopParamsRequest(EndPointNegotiateDesktopParamsRequest {
            video_codecs: vec![VideoCodec::H264],
        }),
    )?;

    tx.send(negotiate_request_buffer)
        .await
        .map_err(|_| CoreError::OutgoingMessageChannelDisconnect)?;

    let negotiate_response_buffer = tokio::time::timeout(RECV_MESSAGE_TIMEOUT, rx.recv())
        .await
        .map_err(|_| CoreError::Timeout)?
        .ok_or(CoreError::OutgoingMessageChannelDisconnect)?;

    let EndPointMessage::NegotiateDesktopParamsResponse(negotiate_response) =
        bincode_deserialize(negotiate_response_buffer.deref())? else {
            return Err(core_error!("unexpected negotiate reply"));
        };

    let params = match negotiate_response {
        EndPointNegotiateDesktopParamsResponse::VideoError(err) => {
            tracing::error!(?err, "negotiate failed with video error");
            return Err(core_error!("negotiate failed ({})", err));
        }
        EndPointNegotiateDesktopParamsResponse::MonitorError(err) => {
            tracing::error!(?err, "negotiate failed with display error");
            return Err(core_error!("negotiate failed ({})", err));
        }
        EndPointNegotiateDesktopParamsResponse::Params(params) => {
            tracing::info!(?params, "negotiate success");
            params
        }
    };

    let negotiate_request_buffer = bincode_serialize(&EndPointMessage::NegotiateFinishedRequest(
        EndPointNegotiateFinishedRequest {
            expected_frame_rate: 60,
        },
    ))?;

    tx.send(negotiate_request_buffer)
        .await
        .map_err(|_| CoreError::OutgoingMessageChannelDisconnect)?;

    Ok(params)
}

fn handle_message(
    client: Arc<EndPointClient>,
    mut rx: tokio::sync::mpsc::Receiver<Bytes>,
    video_frame_tx: Option<Sender<EndPointVideoFrame>>,
    audio_frame_tx: Option<Sender<EndPointAudioFrame>>,
) {
    tokio::spawn(async move {
        loop {
            let buffer = match rx.recv().await {
                Some(buffer) => buffer,
                None => {
                    tracing::info!("message handle channel is closed");
                    break;
                }
            };

            let message = match bincode_deserialize(&buffer) {
                Ok(message) => message,
                Err(err) => {
                    tracing::error!(?err, "deserialize endpoint message failed");
                    continue;
                }
            };

            match message {
                EndPointMessage::Error => {
                    // handle_error(active_device_id, passive_device_id);
                }
                EndPointMessage::NegotiateDesktopParamsRequest(req) => {
                    handle_negotiate_desktop_params_request(client.clone(), req).await
                }
                EndPointMessage::NegotiateDesktopParamsResponse(_) => {
                    // this message should not received at handle_message loop because it already handled
                    // at negotiate stage from active endpoint
                }
                EndPointMessage::NegotiateFinishedRequest(_) => {
                    handle_negotiate_finished_request(client.clone());
                }
                EndPointMessage::VideoFrame(video_frame) => {
                    if let Some(ref tx) = video_frame_tx {
                        if let Err(err) = tx.send(video_frame).await {
                            tracing::error!(%err, "endpoint video frame message channel send failed");
                            return;
                        }
                    } else {
                        tracing::error!("as passive endpoint, shouldn't receive video frame");
                    }
                }
                EndPointMessage::AudioFrame(audio_frame) => {
                    if let Some(ref tx) = audio_frame_tx {
                        if let Err(err) = tx.send(audio_frame).await {
                            tracing::error!(%err, "endpoint audio frame message channel send failed");
                            return;
                        }
                    } else {
                        tracing::error!("as passive endpoint, shouldn't receive audio frame");
                    }
                }
                EndPointMessage::InputCommand(input_event) => {
                    handle_input(client.clone(), input_event).await
                }
                EndPointMessage::CallRequest(call_id, message) => {
                    let client = client.clone();
                    tokio::spawn(async move {
                        let reply = match message {
                            EndPointCallRequest::VisitDirectoryRequest(req) => {
                                call!(handle_visit_directory_request(req).await)
                            }
                            EndPointCallRequest::SendFileRequest(req) => {
                                call!(handle_send_file_request(req).await)
                            }
                            EndPointCallRequest::DownloadFileRequest(req) => {
                                call!(handle_download_file_request(client.clone(), req).await)
                            }
                        };

                        match reply {
                            Ok(reply_bytes) => {
                                if let Err(err) = client
                                    .send(&EndPointMessage::CallReply(call_id, reply_bytes))
                                    .await
                                {
                                    tracing::error!(?err, "reply Call send message failed");
                                }
                            }
                            Err(err) => {
                                tracing::error!(?err, "reply Call failed");
                            }
                        }
                    });
                }
                EndPointMessage::CallReply(call_id, reply) => {
                    tracing::info!(?call_id, "receive call reply");
                    if let Some(tx) = client.call_store.get(&call_id) {
                        let _ = tx.send(reply).await;
                    }

                    client.call_store.invalidate(&call_id)
                }
                EndPointMessage::FileTransferBlock(block) => {
                    append_file_block(client.clone(), block).await
                }
                EndPointMessage::FileTransferError(message) => {
                    delete_file_append_session(&message.id).await
                }
            }
        }

        tracing::info!("message handle loop exit");
    });
}
