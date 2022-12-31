mod tcp;
mod udp;

use self::{tcp::serve_tcp, udp::serve_udp};
use super::{
    handlers::negotiate_desktop_params::handle_negotiate_desktop_params_request, id::EndPointID,
    message::*, EndPointStream,
};
use crate::{
    api::endpoint::handlers::{
        directory::handle_directory_request, input::handle_input,
        negotiate_finished::handle_negotiate_finished_request,
    },
    component::desktop::monitor::Monitor,
    core_error,
    error::{CoreError, CoreResult},
    utility::{
        bincode::{bincode_deserialize, bincode_serialize},
        nonce_value::NonceValue,
    },
};
use bytes::Bytes;
use ring::aead::{OpeningKey, SealingKey};
use std::{fmt::Display, ops::Deref, sync::Arc, time::Duration};
use tokio::sync::{mpsc::Sender, RwLock};

const RECV_MESSAGE_TIMEOUT: Duration = Duration::from_secs(30);

#[derive(Debug, Clone)]
pub struct EndPointClient {
    endpoint_id: EndPointID,
    monitor: Arc<RwLock<Option<Arc<Monitor>>>>,
    tx: Sender<Vec<u8>>,
}

impl EndPointClient {
    pub async fn new_desktop_active(
        endpoint_id: EndPointID,
        stream_key: Option<(OpeningKey<NonceValue>, SealingKey<NonceValue>)>,
        stream: EndPointStream,
        video_frame_tx: Sender<EndPointVideoFrame>,
        audio_frame_tx: Sender<EndPointAudioFrame>,
        directory_tx: Sender<EndPointDirectoryResponse>,
        visit_credentials: Option<Vec<u8>>,
    ) -> CoreResult<Arc<EndPointClient>> {
        EndPointClient::create(
            true,
            endpoint_id,
            stream_key,
            stream,
            Some(video_frame_tx),
            Some(audio_frame_tx),
            Some(directory_tx),
            visit_credentials,
        )
        .await
    }

    pub async fn new_file_manager_active(
        endpoint_id: EndPointID,
        stream_key: Option<(OpeningKey<NonceValue>, SealingKey<NonceValue>)>,
        stream: EndPointStream,
        directory_tx: Sender<EndPointDirectoryResponse>,
        visit_credentials: Option<Vec<u8>>,
    ) -> CoreResult<Arc<EndPointClient>> {
        EndPointClient::create(
            true,
            endpoint_id,
            stream_key,
            stream,
            None,
            None,
            Some(directory_tx),
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
            None,
            visit_credentials,
        )
        .await?;
        Ok(())
    }

    async fn create(
        active: bool,
        endpoint_id: EndPointID,
        key_pair: Option<(OpeningKey<NonceValue>, SealingKey<NonceValue>)>,
        stream: EndPointStream,
        video_frame_tx: Option<Sender<EndPointVideoFrame>>,
        audio_frame_tx: Option<Sender<EndPointAudioFrame>>,
        directory_tx: Option<Sender<EndPointDirectoryResponse>>,
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
                .await??;
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

        let client = Arc::new(EndPointClient {
            endpoint_id,
            monitor: Arc::new(RwLock::new(primary_monitor)),
            tx,
        });

        handle_message(
            client.clone(),
            rx,
            video_frame_tx,
            audio_frame_tx,
            directory_tx,
        );

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
        .await?
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
    directory_tx: Option<Sender<EndPointDirectoryResponse>>,
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
                EndPointMessage::DirectoryRequest(req) => {
                    handle_directory_request(client.clone(), req).await
                }
                EndPointMessage::DirectoryResponse(resp) => {
                    if let Some(ref tx) = directory_tx {
                        if let Err(err) = tx.send(resp).await {
                            tracing::error!(%err, "endpoint directory message channel send failed");
                            return;
                        }
                    } else {
                        tracing::error!(
                            "as passive endpoint, shouldn't receive directory response"
                        );
                    }
                }
            }
        }

        tracing::info!("message handle loop exit");
    });
}
