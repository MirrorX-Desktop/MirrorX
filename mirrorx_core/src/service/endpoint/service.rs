use super::{id::EndPointID, message::*};
use crate::{
    component::{
        audio::{decoder::AudioDecoder, player::default_output_config},
        fs::transfer::{append_file_block, delete_file_append_session},
        screen::Screen,
        video_decoder::decoder::VideoDecoder,
    },
    core_error,
    error::{CoreError, CoreResult},
    service::endpoint::handler::{
        fs_download_file::handle_download_file_request, fs_send_file::handle_send_file_request,
        fs_visit_directory::handle_visit_directory_request, negotiate::handle_negotiate_request,
        switch_screen::handle_switch_screen_request,
    },
    utility::{bincode::bincode_serialize, nonce_value::NonceValue},
    DesktopDecodeFrame,
};
use cpal::traits::StreamTrait;
use moka::sync::{Cache, CacheBuilder};
use ring::aead::{OpeningKey, SealingKey};
use scopeguard::defer;
use std::{
    net::SocketAddr,
    path::PathBuf,
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
    pending_calls: Cache<u16, Sender<Option<EndPointCallReply>>>,
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
            .time_to_live(RECV_MESSAGE_TIMEOUT)
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

    pub(crate) fn blocking_send(&self, message: &EndPointMessage) -> CoreResult<()> {
        let buffer = bincode_serialize(message)?;
        self.stream_tx
            .blocking_send(buffer)
            .map_err(|_| CoreError::OutgoingMessageChannelDisconnect)
    }

    pub(crate) async fn send(&self, message: &EndPointMessage) -> CoreResult<()> {
        let buffer = bincode_serialize(message)?;
        self.stream_tx
            .send(buffer)
            .await
            .map_err(|_| CoreError::OutgoingMessageChannelDisconnect)
    }

    pub async fn call_negotiate(&self) -> CoreResult<EndPointNegotiateReply> {
        let reply = self
            .call(EndPointCallRequest::NegotiateRequest(
                EndPointNegotiateRequest {
                    video_codecs: vec![VideoCodec::H264],
                },
            ))
            .await?;

        if let EndPointCallReply::NegotiateReply(reply) = reply {
            Ok(reply)
        } else {
            Err(core_error!("call reply unexpected message"))
        }
    }

    pub async fn call_switch_screen(
        &self,
        display_id: String,
    ) -> CoreResult<EndPointSwitchScreenReply> {
        let reply = self
            .call(EndPointCallRequest::SwitchScreenRequest(
                EndPointSwitchScreenRequest { display_id },
            ))
            .await?;

        if let EndPointCallReply::SwitchScreenReply(reply) = reply {
            Ok(reply)
        } else {
            Err(core_error!("call reply unexpected message"))
        }
    }

    pub async fn call_visit_directory(
        &self,
        path: Option<PathBuf>,
    ) -> CoreResult<EndPointVisitDirectoryReply> {
        let reply = self
            .call(EndPointCallRequest::VisitDirectoryRequest(
                EndPointVisitDirectoryRequest { path },
            ))
            .await?;

        if let EndPointCallReply::VisitDirectoryReply(reply) = reply {
            Ok(reply)
        } else {
            Err(core_error!("call reply unexpected message"))
        }
    }

    pub async fn call_send_file(
        &self,
        id: String,
        filename: String,
        path: PathBuf,
        size: u64,
    ) -> CoreResult<EndPointSendFileReply> {
        let reply = self
            .call(EndPointCallRequest::SendFileRequest(
                EndPointSendFileRequest {
                    id,
                    filename,
                    path,
                    size,
                },
            ))
            .await?;

        if let EndPointCallReply::SendFileReply(reply) = reply {
            Ok(reply)
        } else {
            Err(core_error!("call reply unexpected message"))
        }
    }

    pub async fn call_download_file(
        &self,
        id: String,
        path: PathBuf,
    ) -> CoreResult<EndPointDownloadFileReply> {
        let reply = self
            .call(EndPointCallRequest::DownloadFileRequest(
                EndPointDownloadFileRequest { id, path },
            ))
            .await?;

        if let EndPointCallReply::DownloadFileReply(reply) = reply {
            Ok(reply)
        } else {
            Err(core_error!("call reply unexpected message"))
        }
    }

    pub(crate) async fn call(&self, message: EndPointCallRequest) -> CoreResult<EndPointCallReply> {
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

        let reply = rx
            .recv()
            .await
            .ok_or(CoreError::Timeout)?
            .ok_or(core_error!("internal"))?;

        Ok(reply)
    }

    pub async fn tell_file_transfer_error(&self, id: String) {
        let _ = self
            .send(&EndPointMessage::FileTransferError(
                EndPointFileTransferError { id: id.clone() },
            ))
            .await;
    }

    pub async fn update_screen(&self, screen: Screen) -> CoreResult<()> {
        self.push_command(ServiceCommand::UpdateScreen(screen))
            .await
    }

    pub async fn spawn_video_decode_task(&self) -> CoreResult<Receiver<DesktopDecodeFrame>> {
        let endpoint_id = self.endpoint_id;
        let (render_tx, render_rx) = tokio::sync::mpsc::channel(1);
        let (decode_tx, mut decode_rx) = tokio::sync::mpsc::channel(120);

        tokio::task::spawn_blocking(move || {
            tracing::info!(?endpoint_id, "video decode process");

            let mut decoder = VideoDecoder::new(render_tx);

            while let Some(video_frame) = decode_rx.blocking_recv() {
                if let Err(err) = decoder.decode(video_frame) {
                    tracing::error!(?err, "decode video frame failed");
                    break;
                }
            }

            tracing::info!("video decode process exit");
        });

        self.push_command(ServiceCommand::UpdateVideoFrameTx(decode_tx))
            .await?;

        Ok(render_rx)
    }

    pub async fn spawn_audio_play_task(&self) -> CoreResult<()> {
        let endpoint_id = self.endpoint_id;
        let (decode_tx, mut decode_rx) = tokio::sync::mpsc::channel(1);

        tokio::task::spawn_blocking(move || loop {
            tracing::info!(?endpoint_id, "audio decode process");

            let Ok(config) = default_output_config() else {
                tracing::error!("get default audio output config failed");
                return;
            };

            tracing::info!(?config, "default output config");

            let mut audio_decoder = AudioDecoder::new(
                config.channels() as _,
                config.sample_format(),
                config.sample_rate(),
            );

            let mut stream = None;
            let mut samples_tx = None;

            loop {
                match decode_rx.blocking_recv() {
                    Some(audio_frame) => {
                        match audio_decoder.decode(audio_frame) {
                            Ok(buffer) => {
                                // because active endpoint always output 48000hz and 480 samples per channel after
                                // opus encode, so here we simply div (48000/480)=100 to get samples count after
                                // resample.
                                let valid_min_samples_per_channel = config.sample_rate().0 / 100;

                                if stream.is_none() {
                                    let buffer_size = buffer.len()
                                        / (config.channels() as usize)
                                        / config.sample_format().sample_size();

                                    // drop the beginning frames
                                    if buffer_size < (valid_min_samples_per_channel as usize) {
                                        continue;
                                    }

                                    tracing::info!(?buffer_size, "use buffer size");

                                    match crate::component::audio::player::new_play_stream_and_tx(
                                        config.channels(),
                                        config.sample_format(),
                                        config.sample_rate(),
                                        buffer_size as u32,
                                    ) {
                                        Ok((play_stream, audio_sample_tx)) => {
                                            if let Err(err) = play_stream.play() {
                                                tracing::error!(?err, "play audio stream failed");
                                                return;
                                            }

                                            stream = Some(play_stream);
                                            samples_tx = Some(audio_sample_tx);
                                        }
                                        Err(err) => {
                                            tracing::error!(
                                                ?err,
                                                "initialize audio play stream failed"
                                            );
                                            continue;
                                        }
                                    };
                                }

                                if let Some(ref samples_tx) = samples_tx {
                                    if samples_tx.blocking_send(buffer).is_err() {
                                        tracing::error!("send audio play buffer failed");
                                        return;
                                    }
                                }
                            }
                            Err(err) => {
                                tracing::error!(?err, "decode audio frame failed");
                                break;
                            }
                        };
                    }
                    None => {
                        if let Some(ref stream) = stream {
                            let _ = stream.pause();
                        }

                        tracing::info!("audio decode process exit");
                        return;
                    }
                }
            }

            if let Some(ref stream) = stream {
                let _ = stream.pause();
            }
        });

        self.push_command(ServiceCommand::UpdateAudioFrameTx(decode_tx))
            .await?;

        Ok(())
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
        EndPointCallRequest::NegotiateRequest(req) => handle_negotiate_request(req)
            .await
            .map(EndPointCallReply::NegotiateReply),
        EndPointCallRequest::SwitchScreenRequest(req) => {
            handle_switch_screen_request(service.clone(), req)
                .await
                .map(EndPointCallReply::SwitchScreenReply)
        }
        EndPointCallRequest::VisitDirectoryRequest(req) => handle_visit_directory_request(req)
            .await
            .map(EndPointCallReply::VisitDirectoryReply),
        EndPointCallRequest::SendFileRequest(req) => handle_send_file_request(req)
            .await
            .map(EndPointCallReply::SendFileReply),
        EndPointCallRequest::DownloadFileRequest(req) => {
            handle_download_file_request(service.clone(), req)
                .await
                .map(EndPointCallReply::DownloadFileReply)
        }
    };

    let reply = match reply {
        Ok(reply_bytes) => Some(reply_bytes),
        Err(err) => {
            tracing::error!(?err, "reply call failed");
            None
        }
    };

    if let Err(err) = service
        .send(&EndPointMessage::CallReply(call_id, reply))
        .await
    {
        tracing::error!(?err, "reply call send message failed");
    }
}
