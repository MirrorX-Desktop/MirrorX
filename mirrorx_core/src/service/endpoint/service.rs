use super::{id::EndPointID, message::*};
use crate::{
    call,
    component::{
        fs::transfer::{append_file_block, delete_file_append_session},
        screen::Screen,
    },
    core_error,
    error::{CoreError, CoreResult},
    service::endpoint::handler::{
        fs_download_file::handle_download_file_request, fs_send_file::handle_send_file_request,
        fs_visit_directory::handle_visit_directory_request, negotiate::handle_negotiate_request,
        switch_screen::handle_switch_screen_request,
    },
    utility::{
        bincode::{bincode_deserialize, bincode_serialize},
        nonce_value::NonceValue,
    },
};
use moka::sync::{Cache, CacheBuilder};
use ring::aead::{OpeningKey, SealingKey};
use scopeguard::defer;
use serde::de::DeserializeOwned;
use std::{
    net::SocketAddr,
    sync::{atomic::AtomicU16, Arc},
    time::Duration,
};
use tokio::{
    net::{TcpStream, UdpSocket},
    sync::mpsc::{Receiver, Sender},
};

const RECV_MESSAGE_TIMEOUT: Duration = Duration::from_secs(30);

pub enum EndPointStreamType {
    ActiveTCP(SocketAddr),
    ActiveUDP(SocketAddr),
    PassiveTCP(TcpStream),
    PassiveUDP {
        remote_addr: SocketAddr,
        socket: UdpSocket,
    },
}

pub enum ServiceCommand {
    UpdateVideoFrameTx(Sender<EndPointVideoFrame>),
    UpdateAudioFrameTx(Sender<EndPointAudioFrame>),
    UpdateScreen(Screen),
}

#[derive(Debug)]
pub struct Service {
    endpoint_id: EndPointID,
    stream_tx: Sender<Vec<u8>>,
    command_tx: Sender<ServiceCommand>,
    call_id: Arc<AtomicU16>,
    pending_calls: Cache<u16, Sender<Vec<u8>>>,
}

impl Service {
    #[allow(clippy::too_many_arguments)]
    pub async fn new(
        endpoint_id: EndPointID,
        stream_type: EndPointStreamType,
        key_pair: Option<(OpeningKey<NonceValue>, SealingKey<NonceValue>)>,
        visit_credentials: Option<Vec<u8>>,
    ) -> CoreResult<Arc<Service>> {
        let (opening_key, sealing_key) = match key_pair {
            Some((opening_key, sealing_key)) => (Some(opening_key), Some(sealing_key)),
            None => (None, None),
        };

        let (stream_tx, stream_rx) = match stream_type {
            EndPointStreamType::ActiveTCP(addr) => {
                let stream = tokio::time::timeout(
                    Duration::from_secs(10),
                    tokio::net::TcpStream::connect(addr),
                )
                .await
                .map_err(|_| CoreError::Timeout)??;

                super::transport::serve_tcp(
                    stream,
                    endpoint_id,
                    sealing_key,
                    opening_key,
                    visit_credentials,
                )
                .await?
            }
            EndPointStreamType::ActiveUDP(_) => panic!("not support yet"),
            EndPointStreamType::PassiveTCP(stream) => {
                super::transport::serve_tcp(
                    stream,
                    endpoint_id,
                    sealing_key,
                    opening_key,
                    visit_credentials,
                )
                .await?
            }
            EndPointStreamType::PassiveUDP { socket, .. } => {
                super::transport::serve_udp(
                    socket,
                    endpoint_id,
                    sealing_key,
                    opening_key,
                    visit_credentials,
                )
                .await?
            }
        };

        let pending_calls = CacheBuilder::new(32)
            .time_to_live(Duration::from_secs(60))
            .build();

        let (command_tx, command_rx) = tokio::sync::mpsc::channel(8);

        let service = Arc::new(Service {
            endpoint_id,
            stream_tx,
            command_tx,
            call_id: Arc::new(AtomicU16::new(0)),
            pending_calls,
        });

        let service_clone = service.clone();
        tokio::spawn(async move {
            service_task(service_clone, stream_rx, command_rx).await;
            tracing::info!("service task exit");
        });

        Ok(service)
    }

    pub fn try_send(&self, message: &EndPointMessage) -> CoreResult<()> {
        let buffer = bincode_serialize(message)?;
        self.stream_tx
            .try_send(buffer)
            .map_err(|_| CoreError::OutgoingMessageChannelDisconnect)
    }

    pub fn blocking_send(&self, message: &EndPointMessage) -> CoreResult<()> {
        let buffer = bincode_serialize(message)?;
        self.stream_tx
            .blocking_send(buffer)
            .map_err(|_| CoreError::OutgoingMessageChannelDisconnect)
    }

    pub async fn send(&self, message: &EndPointMessage) -> CoreResult<()> {
        let buffer = bincode_serialize(message)?;
        self.stream_tx
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

        self.pending_calls.insert(call_id, tx);
        defer! {
            self.pending_calls.invalidate(&call_id);
        }

        self.send(&EndPointMessage::CallRequest(call_id, message))
            .await?;

        let reply_bytes = rx.recv().await.ok_or(CoreError::Timeout)?;

        bincode_deserialize::<Result<TReply, String>>(&reply_bytes)?
            .map_err(|err_str| core_error!("{}", err_str))
    }

    pub async fn update_video_frame_channel(
        &self,
        tx: Sender<EndPointVideoFrame>,
    ) -> CoreResult<()> {
        self.push_command(ServiceCommand::UpdateVideoFrameTx(tx))
            .await
    }

    pub async fn update_audio_frame_channel(
        &self,
        tx: Sender<EndPointAudioFrame>,
    ) -> CoreResult<()> {
        self.push_command(ServiceCommand::UpdateAudioFrameTx(tx))
            .await
    }

    pub async fn update_screen(&self, screen: Screen) -> CoreResult<()> {
        self.push_command(ServiceCommand::UpdateScreen(screen))
            .await
    }

    async fn push_command(&self, command: ServiceCommand) -> CoreResult<()> {
        self.command_tx
            .send(command)
            .await
            .map_err(|_| CoreError::OutgoingMessageChannelDisconnect)
    }
}

async fn service_task(
    service: Arc<Service>,
    mut stream_rx: Receiver<EndPointMessage>,
    mut command_rx: Receiver<ServiceCommand>,
) {
    let mut current_video_frame_tx = None;
    let mut current_audio_frame_tx = None;
    let mut current_screen: Option<Screen> = None;

    loop {
        if let Ok(command) = command_rx.try_recv() {
            match command {
                ServiceCommand::UpdateVideoFrameTx(video_frame_tx) => {
                    current_video_frame_tx = Some(video_frame_tx)
                }
                ServiceCommand::UpdateAudioFrameTx(audio_frame_tx) => {
                    current_audio_frame_tx = Some(audio_frame_tx);
                }
                ServiceCommand::UpdateScreen(mut new_screen) => {
                    if let Some(mut old_screen) = current_screen.take() {
                        old_screen.stop_capture_desktop();
                        drop(old_screen);
                    }

                    new_screen.start_capture_desktop();
                    current_screen = Some(new_screen);
                }
            }
        }

        let message = match stream_rx.recv().await {
            Some(buffer) => buffer,
            None => break,
        };

        match message {
            EndPointMessage::Error => {
                // handle_error(active_device_id, passive_device_id);
            }
            EndPointMessage::VideoFrame(video_frame) => {
                if let Some(ref tx) = current_video_frame_tx {
                    if tx.send(video_frame).await.is_err() {
                        tracing::error!("endpoint video frame channel closed");
                        break;
                    }
                }
            }
            EndPointMessage::AudioFrame(audio_frame) => {
                if let Some(ref tx) = current_audio_frame_tx {
                    if tx.send(audio_frame).await.is_err() {
                        tracing::error!("endpoint audio frame channel closed");
                        break;
                    }
                }
            }
            EndPointMessage::InputCommand(input_event) => {
                if let Some(ref screen) = current_screen {
                    if screen.send_input_event(input_event).await.is_err() {
                        tracing::error!("endpoint input command channel closed");
                        break;
                    }
                }
            }
            EndPointMessage::CallRequest(call_id, req) => {
                tokio::spawn(handle_call(service.clone(), call_id, req));
            }
            EndPointMessage::CallReply(call_id, reply) => {
                if let Some(tx) = service.pending_calls.get(&call_id) {
                    let _ = tx.try_send(reply);
                }
                service.pending_calls.invalidate(&call_id)
            }
            EndPointMessage::FileTransferBlock(block) => {
                append_file_block(service.clone(), block).await
            }
            EndPointMessage::FileTransferError(message) => {
                delete_file_append_session(&message.id).await
            }
        }
    }
}

async fn handle_call(service: Arc<Service>, call_id: u16, req: EndPointCallRequest) {
    let reply = match req {
        EndPointCallRequest::NegotiateRequest(req) => {
            call!(handle_negotiate_request(req).await)
        }
        EndPointCallRequest::SwitchScreenRequest(req) => {
            call!(handle_switch_screen_request(service.clone(), req).await)
        }
        EndPointCallRequest::VisitDirectoryRequest(req) => {
            call!(handle_visit_directory_request(req).await)
        }
        EndPointCallRequest::SendFileRequest(req) => {
            call!(handle_send_file_request(req).await)
        }
        EndPointCallRequest::DownloadFileRequest(req) => {
            call!(handle_download_file_request(service.clone(), req).await)
        }
    };

    match reply {
        Ok(reply_bytes) => {
            if let Err(err) = service
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
}
