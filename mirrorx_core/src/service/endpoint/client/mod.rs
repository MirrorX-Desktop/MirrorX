mod tcp;
mod udp;

use self::{tcp::serve_tcp, udp::serve_udp};
use super::{id::EndPointID, message::*, EndPointStream};
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
        switch_display::handle_switch_screen_request,
    },
    utility::{
        bincode::{bincode_deserialize, bincode_serialize},
        nonce_value::NonceValue,
    },
};
use bytes::Bytes;
use moka::sync::{Cache, CacheBuilder};
use ring::aead::{OpeningKey, SealingKey};
use scopeguard::defer;
use serde::de::DeserializeOwned;
use std::{
    sync::{atomic::AtomicU16, Arc},
    time::Duration,
};
use tokio::sync::mpsc::{Receiver, Sender};

const RECV_MESSAGE_TIMEOUT: Duration = Duration::from_secs(30);

#[derive(Debug)]
pub struct EndPointClient {
    endpoint_id: EndPointID,
    tx: Sender<Vec<u8>>,
    call_id: Arc<AtomicU16>,
    call_store: moka::sync::Cache<u16, Sender<Vec<u8>>>,
}

impl EndPointClient {
    pub async fn new_active_endpoint(
        endpoint_id: EndPointID,
        stream_key: Option<(OpeningKey<NonceValue>, SealingKey<NonceValue>)>,
        stream: EndPointStream,
        video_frame_tx: Sender<EndPointVideoFrame>,
        audio_frame_tx: Sender<EndPointAudioFrame>,
        visit_credentials: Option<Vec<u8>>,
    ) -> CoreResult<EndPointClient> {
        EndPointClient::create(
            endpoint_id,
            stream_key,
            stream,
            Some(video_frame_tx),
            Some(audio_frame_tx),
            visit_credentials,
        )
        .await
    }

    pub async fn new_passive_endpoint(
        endpoint_id: EndPointID,
        stream_key: Option<(OpeningKey<NonceValue>, SealingKey<NonceValue>)>,
        stream: EndPointStream,
        visit_credentials: Option<Vec<u8>>,
    ) -> CoreResult<EndPointClient> {
        EndPointClient::create(
            endpoint_id,
            stream_key,
            stream,
            None,
            None,
            visit_credentials,
        )
        .await
    }

    #[allow(clippy::too_many_arguments)]
    async fn create(
        endpoint_id: EndPointID,
        key_pair: Option<(OpeningKey<NonceValue>, SealingKey<NonceValue>)>,
        stream: EndPointStream,
        video_frame_tx: Option<Sender<EndPointVideoFrame>>,
        audio_frame_tx: Option<Sender<EndPointAudioFrame>>,
        visit_credentials: Option<Vec<u8>>,
    ) -> CoreResult<EndPointClient> {
        let (opening_key, sealing_key) = match key_pair {
            Some((opening_key, sealing_key)) => (Some(opening_key), Some(sealing_key)),
            None => (None, None),
        };

        let (tx, rx) = match stream {
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

        let call_store = CacheBuilder::new(32)
            .time_to_live(Duration::from_secs(60))
            .build();

        let client = EndPointClient {
            endpoint_id,
            tx: tx.clone(),
            call_id: Arc::new(AtomicU16::new(0)),
            call_store: call_store.clone(),
        };

        handle_message(
            call_store,
            ClientSendStream(tx),
            rx,
            video_frame_tx,
            audio_frame_tx,
        );

        Ok(client)
    }
}

impl EndPointClient {
    pub fn new_send_stream(&self) -> ClientSendStream {
        ClientSendStream(self.tx.clone())
    }

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

impl std::fmt::Display for EndPointClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "EndPointClient({})", self.endpoint_id)
    }
}

#[derive(Clone)]
pub struct ClientSendStream(Sender<Vec<u8>>);

impl ClientSendStream {
    pub async fn send(&self, message: &EndPointMessage) -> CoreResult<()> {
        let buffer = bincode_serialize(message)?;
        self.0
            .send(buffer)
            .await
            .map_err(|_| CoreError::OutgoingMessageChannelDisconnect)
    }

    pub fn blocking_send(&self, message: &EndPointMessage) -> CoreResult<()> {
        let buffer = bincode_serialize(message)?;
        self.0
            .blocking_send(buffer)
            .map_err(|_| CoreError::OutgoingMessageChannelDisconnect)
    }
}

fn handle_message(
    call_store: Cache<u16, Sender<Vec<u8>>>,
    client_send_stream: ClientSendStream,
    mut rx: Receiver<Bytes>,
    video_frame_tx: Option<Sender<EndPointVideoFrame>>,
    audio_frame_tx: Option<Sender<EndPointAudioFrame>>,
) {
    tokio::spawn(async move {
        let mut current_screen: Option<Screen> = None;

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
                    if let Some(ref screen) = current_screen {
                        for event in input_event.events {
                            match event {
                                InputEvent::Mouse(mouse_event) => {
                                    screen.input_mouse_event(mouse_event)
                                }
                                InputEvent::Keyboard(keyboard_event) => {
                                    screen.input_keyboard_event(keyboard_event)
                                }
                            }
                        }
                    }
                }
                EndPointMessage::CallRequest(call_id, message) => {
                    let client_send_stream = client_send_stream.clone();

                    let reply = match message {
                        EndPointCallRequest::NegotiateRequest(req) => {
                            call!(handle_negotiate_request(req).await)
                        }
                        EndPointCallRequest::SwitchScreenRequest(req) => {
                            call!(handle_switch_screen_request(
                                &mut current_screen,
                                req,
                                client_send_stream.clone()
                            ))
                        }
                        EndPointCallRequest::VisitDirectoryRequest(req) => {
                            call!(handle_visit_directory_request(req).await)
                        }
                        EndPointCallRequest::SendFileRequest(req) => {
                            call!(handle_send_file_request(req).await)
                        }
                        EndPointCallRequest::DownloadFileRequest(req) => {
                            call!(
                                handle_download_file_request(client_send_stream.clone(), req).await
                            )
                        }
                    };

                    match reply {
                        Ok(reply_bytes) => {
                            if let Err(err) = client_send_stream
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
                EndPointMessage::CallReply(call_id, reply) => {
                    tracing::info!(?call_id, "receive call reply");
                    if let Some(tx) = call_store.get(&call_id) {
                        let _ = tx.send(reply).await;
                    }
                    call_store.invalidate(&call_id)
                }
                EndPointMessage::FileTransferBlock(block) => {
                    append_file_block(client_send_stream.clone(), block).await
                }
                EndPointMessage::FileTransferError(message) => {
                    delete_file_append_session(&message.id).await
                }
            }
        }

        tracing::info!("message handle loop exit");
    });
}
