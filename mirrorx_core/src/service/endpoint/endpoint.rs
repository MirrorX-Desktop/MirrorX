use super::{
    handler::{handle_get_display_info_request, handle_start_media_transmission_request},
    message::*,
    processor::{audio::*, desktop::start_desktop_capture_process},
    processor::{desktop::start_desktop_render_process, video::*},
};
use crate::{
    component::desktop::monitor::Monitor,
    core_error,
    error::{CoreError, CoreResult},
    service::endpoint::handler::handle_input,
    utility::{nonce_value::NonceValue, runtime::TOKIO_RUNTIME, serializer::BINCODE_SERIALIZER},
};
use bincode::Options;
use bytes::Bytes;
use crossbeam::channel::Sender;
use dashmap::DashMap;
use futures::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use once_cell::sync::{Lazy, OnceCell};
use ring::aead::{OpeningKey, SealingKey};
use rtrb::RingBuffer;
use scopeguard::defer;
use std::{
    sync::{
        atomic::{AtomicU16, Ordering},
        Arc,
    },
    time::Duration,
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpStream, ToSocketAddrs},
};
use tokio_util::codec::{Framed, LengthDelimitedCodec};

const CALL_TIMEOUT: Duration = Duration::from_secs(5);

pub static ENDPOINTS: Lazy<DashMap<String, Arc<EndPoint>>> = Lazy::new(|| DashMap::new());

macro_rules! make_endpoint_call {
    ($name:tt, $req_type:ident, $req_message_type:path, $resp_type:ident, $resp_message_type:path) => {
        pub async fn $name(&self, req: $req_type) -> CoreResult<$resp_type> {
            let reply = self.call($req_message_type(req), CALL_TIMEOUT).await?;

            if let $resp_message_type(message) = reply {
                Ok(message)
            } else {
                Err(core_error!(
                    "endpoint call received mismatched response message"
                ))
            }
        }
    };
}

macro_rules! make_endpoint_push {
    ($name:tt, $req_type:ident, $req_message_type:path) => {
        pub async fn $name(&self, req: $req_type) -> CoreResult<()> {
            self.send(EndPointMessagePacket {
                typ: EndPointMessagePacketType::Push,
                call_id: None,
                message: $req_message_type(req),
            })
            .await
        }
    };
}

macro_rules! handle_call_message {
    ($endpoint:expr, $call_id:expr, $req:tt, $resp_type:path, $handler:tt) => {{
        let endpoint = $endpoint.clone();
        TOKIO_RUNTIME.spawn(async move {
            if let Some(call_id) = $call_id {
                let resp_message = match $handler(&endpoint, $req).await {
                    Ok(resp) => $resp_type(resp),
                    Err(err) => {
                        tracing::error!(?err, "handler '{}' returns error", stringify!($handler));
                        EndPointMessage::Error
                    }
                };

                if let Err(err) = endpoint.reply(call_id,resp_message).await{
                    tracing::error!(?err, remote_device_id = ?endpoint.remote_device_id(), "handler '{}' reply message failed", stringify!($handler));
                }
            } else {
                tracing::error!("received request message '{}' without call id", stringify!($req));
            }
        });
    }};
}

macro_rules! handle_push_message {
    ($endpoint:expr, $req:tt, $handler:tt) => {{
        let endpoint = $endpoint.clone();
        TOKIO_RUNTIME.spawn(async move {
            if let Err(err) = $handler(&endpoint, $req).await {
                tracing::error!(?err, remote_device_id = ?endpoint.remote_device_id(), "handle_push_message: handler '{}' returns error", stringify!($handler));
            }
        });
    }};
}

pub struct EndPoint {
    monitor: OnceCell<Monitor>,
    local_device_id: String,
    remote_device_id: String,
    atomic_call_id: AtomicU16,
    call_reply_tx_map: DashMap<u16, tokio::sync::oneshot::Sender<EndPointMessage>>,
    packet_tx: tokio::sync::mpsc::Sender<EndPointMessagePacket>,
    video_frame_tx: OnceCell<Sender<VideoFrame>>,
    audio_frame_tx: OnceCell<Sender<AudioFrame>>,
    exit_tx: async_broadcast::Sender<()>,
    exit_rx: async_broadcast::Receiver<()>,
}

pub async fn connect<A>(
    addr: A,
    is_active_side: bool,
    local_device_id: String,
    remote_device_id: String,
    opening_key: OpeningKey<NonceValue>,
    sealing_key: SealingKey<NonceValue>,
) -> CoreResult<()>
where
    A: ToSocketAddrs,
{
    let mut stream =
        tokio::time::timeout(Duration::from_secs(10), TcpStream::connect(addr)).await??;
    stream.set_nodelay(true)?;

    // handshake for endpoint

    let (active_device_id, passive_device_id) = if is_active_side {
        (
            format!("{:0>10}", local_device_id),
            format!("{:0>10}", remote_device_id),
        )
    } else {
        (
            format!("{:0>10}", remote_device_id),
            format!("{:0>10}", local_device_id),
        )
    };

    let active_device_id_buf = active_device_id.as_bytes();
    if active_device_id_buf.len() != 10 {
        return Err(core_error!("active device id bytes length is not 10"));
    }

    let passive_device_id_buf = passive_device_id.as_bytes();
    if passive_device_id_buf.len() != 10 {
        return Err(core_error!("passive device id bytes length is not 10"));
    }

    stream.write(active_device_id_buf).await?;
    stream.write(passive_device_id_buf).await?;

    let mut handshake_response_buf = [0u8; 1];
    tokio::time::timeout(
        Duration::from_secs(60),
        stream.read_exact(&mut handshake_response_buf),
    )
    .await??;

    if handshake_response_buf[0] != 1 {
        return Err(core_error!(
            "endpoint handshake failed, response returns false"
        ));
    }

    let framed_stream = LengthDelimitedCodec::builder()
        .little_endian()
        .max_frame_length(16 * 1024 * 1024)
        .new_framed(stream);

    let (sink, stream) = framed_stream.split();

    let (packet_tx, packet_rx) = tokio::sync::mpsc::channel(128);

    let (exit_tx, exit_rx) = async_broadcast::broadcast(1);

    let endpoint = Arc::new(EndPoint {
        monitor: OnceCell::new(),
        local_device_id,
        remote_device_id: remote_device_id.clone(),
        atomic_call_id: AtomicU16::new(0),
        call_reply_tx_map: DashMap::new(),
        packet_tx,
        video_frame_tx: OnceCell::new(),
        audio_frame_tx: OnceCell::new(),
        exit_tx: exit_tx.clone(),
        exit_rx,
    });

    serve_reader(
        endpoint.clone(),
        exit_tx.new_receiver(),
        stream,
        opening_key,
    );

    serve_writer(
        endpoint.clone(),
        remote_device_id.clone(),
        exit_tx.new_receiver(),
        packet_rx,
        sink,
        sealing_key,
    );

    ENDPOINTS.insert(remote_device_id, endpoint);

    Ok(())
}

impl EndPoint {
    pub fn remote_device_id<'a>(&'a self) -> &'a str {
        &self.remote_device_id
    }

    pub fn local_device_id<'a>(&'a self) -> &'a str {
        &self.local_device_id
    }

    pub fn monitor(&self) -> Option<&Monitor> {
        self.monitor.get()
    }

    pub fn subscribe_exit(&self) -> async_broadcast::Receiver<()> {
        self.exit_tx.new_receiver()
    }
}

impl EndPoint {
    async fn call(
        &self,
        message: EndPointMessage,
        duration: Duration,
    ) -> CoreResult<EndPointMessage> {
        let call_id = self.atomic_call_id.fetch_add(1, Ordering::SeqCst);

        let packet = EndPointMessagePacket {
            typ: EndPointMessagePacketType::Request,
            call_id: Some(call_id),
            message,
        };

        let rx = self.register_call(call_id);
        defer! {
            self.remove_call(call_id);
        }

        tokio::time::timeout(duration, async move {
            self.send(packet).await?;
            rx.await
                .map_err(|err| core_error!("endpoint call reply channel returns error ({:?})", err))
        })
        .await?
    }

    async fn reply(&self, call_id: u16, message: EndPointMessage) -> CoreResult<()> {
        let packet = EndPointMessagePacket {
            typ: EndPointMessagePacketType::Response,
            call_id: Some(call_id),
            message,
        };

        self.send(packet).await
    }

    async fn send(&self, packet: EndPointMessagePacket) -> CoreResult<()> {
        self.packet_tx
            .try_send(packet)
            .map_err(|err| core_error!("packet tx try send failed"))
    }

    fn set_call_reply(&self, call_id: u16, message: EndPointMessage) {
        self.remove_call(call_id).map(|tx| {
            if let Err(_) = tx.send(message) {
                tracing::error!(remote_device_id=?self.remote_device_id, "endpoint set call reply failed");
            }
        });
    }

    fn register_call(&self, call_id: u16) -> tokio::sync::oneshot::Receiver<EndPointMessage> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.call_reply_tx_map.insert(call_id, tx);
        rx
    }

    fn remove_call(&self, call_id: u16) -> Option<tokio::sync::oneshot::Sender<EndPointMessage>> {
        self.call_reply_tx_map.remove(&call_id).map(|entry| entry.1)
    }

    pub async fn start_video_capture(
        &self,
        display_id: &str,
        except_fps: u8,
    ) -> CoreResult<Monitor> {
        let monitors = crate::component::desktop::monitor::get_active_monitors()?;

        let monitor = match monitors.iter().find(|m| m.id == display_id) {
            Some(m) => m,
            None => match monitors.iter().find(|m| m.is_primary) {
                Some(m) => m,
                None => {
                    return Err(core_error!(
                        "can not find primary monitor or monitor with id ({})",
                        display_id
                    ));
                }
            },
        };

        let width = monitor.width;
        let height = monitor.height;
        let fps = monitor.refresh_rate.min(except_fps);

        // let (capture_frame_tx, capture_frame_rx) = crossbeam::channel::bounded(1);

        // start_video_encode_process(
        //     self.remote_device_id.clone(),
        //     self.exit_tx.clone(),
        //     self.exit_tx.new_receiver(),
        //     width as i32,
        //     height as i32,
        //     fps as i32,
        //     capture_frame_rx,
        //     self.packet_tx.clone(),
        // )?;

        // start_desktop_capture_process(
        //     self.remote_device_id.clone(),
        //     self.exit_tx.clone(),
        //     self.exit_tx.new_receiver(),
        //     capture_frame_tx,
        //     display_id,
        //    fps,
        // )?;

        let _ = self.monitor.set(monitor.clone());

        Ok(monitor.clone())
    }

    pub async fn start_video_render(
        &self,
        width: i32,
        height: i32,
        fps: i32,
        texture_id: i64,
        video_texture_ptr: i64,
        update_frame_callback_ptr: i64,
    ) -> Result<(), CoreError> {
        let (video_frame_tx, video_frame_rx) = crossbeam::channel::bounded(600);
        let (decoded_frame_tx, decoded_frame_rx) = crossbeam::channel::bounded(600);

        start_video_decode_process(
            self.remote_device_id.clone(),
            self.exit_tx.clone(),
            self.exit_tx.new_receiver(),
            width,
            height,
            fps,
            video_frame_rx,
            decoded_frame_tx,
        )?;

        start_desktop_render_process(
            self.remote_device_id.clone(),
            decoded_frame_rx,
            texture_id,
            video_texture_ptr,
            update_frame_callback_ptr,
        )?;

        let _ = self.video_frame_tx.set(video_frame_tx);

        Ok(())
    }

    pub async fn start_audio_capture(&self) -> Result<(), CoreError> {
        let (pcm_tx, pcm_rx) = crossbeam::channel::bounded(48000 / 960 * 2);

        start_audio_encode_process(
            self.remote_device_id.clone(),
            pcm_rx,
            self.packet_tx.clone(),
            48000,
            2,
        )?;

        let exit_tx = start_audio_capture_process(self.remote_device_id.clone(), pcm_tx).await?;

        Ok(())
    }

    pub async fn start_audio_play(&self) -> Result<(), CoreError> {
        let (audio_frame_tx, audio_frame_rx) =
            crossbeam::channel::bounded::<AudioFrame>(48000 / 960 * 2);
        let (pcm_producer, pcm_consumer) = RingBuffer::new(48000 * 2);

        start_audio_decode_process(
            self.remote_device_id.clone(),
            48000,
            2,
            audio_frame_rx,
            pcm_producer,
        )?;

        let exit_tx = start_audio_play_process(self.remote_device_id.clone(), pcm_consumer).await?;

        let _ = self.audio_frame_tx.set(audio_frame_tx);

        Ok(())
    }

    pub fn manually_close(&self) {
        let _ = self.exit_tx.try_broadcast(());
    }

    make_endpoint_call!(
        start_media_transmission,
        StartMediaTransmissionRequest,
        EndPointMessage::StartMediaTransmissionRequest,
        StartMediaTransmissionResponse,
        EndPointMessage::StartMediaTransmissionResponse
    );

    make_endpoint_call!(
        get_display_info,
        GetDisplayInfoRequest,
        EndPointMessage::GetDisplayInfoRequest,
        GetDisplayInfoResponse,
        EndPointMessage::GetDisplayInfoResponse
    );

    make_endpoint_push!(trigger_input, Input, EndPointMessage::Input);
}

impl Drop for EndPoint {
    fn drop(&mut self) {
        let _ = self.exit_tx.try_broadcast(());
        tracing::info!(remote_device_id = ?self.remote_device_id, "endpoint dropped");
    }
}

fn serve_reader(
    endpoint: Arc<EndPoint>,
    mut exit_rx: async_broadcast::Receiver<()>,
    mut stream: SplitStream<Framed<TcpStream, LengthDelimitedCodec>>,
    mut opening_key: OpeningKey<NonceValue>,
) {
    TOKIO_RUNTIME.spawn(async move {
        loop {
            tokio::select! {
                _ = exit_rx.recv() => break,
                res = stream.next() => match res {
                    Some(res) => {
                        let mut packet_bytes = match res {
                            Ok(packet_bytes) => packet_bytes,
                            Err(err) => {
                                tracing::error!(remote_device_id=?endpoint.remote_device_id(), ?err, "read from network stream failed");
                                break;
                            }
                        };

                        let opened_packet_bytes =
                            match opening_key.open_in_place(ring::aead::Aad::empty(), &mut packet_bytes) {
                                Ok(v) => v,
                                Err(err) => {
                                    tracing::error!(remote_device_id=?endpoint.remote_device_id(), ?err, "decrypt packet data failed");
                                    break;
                                }
                            };

                        let packet = match BINCODE_SERIALIZER
                            .deserialize::<EndPointMessagePacket>(&opened_packet_bytes)
                        {
                            Ok(packet) => packet,
                            Err(err) => {
                                tracing::error!(remote_device_id=?endpoint.remote_device_id(), ?err, "deserialize packet failed");
                                break;
                            }
                        };

                        handle_message(&endpoint, packet);
                    },
                    None => {
                        tracing::info!(remote_device_id=?endpoint.remote_device_id(), "network stream closed");
                        break;
                    },
                }
            }
        }

        endpoint.manually_close();
        tracing::info!(remote_device_id=?endpoint.remote_device_id(), "read process exit");
    });
}

fn serve_writer(
    endpoint: Arc<EndPoint>,
    remote_device_id: String,
    mut exit_rx: async_broadcast::Receiver<()>,
    mut packet_rx: tokio::sync::mpsc::Receiver<EndPointMessagePacket>,
    mut sink: SplitSink<Framed<TcpStream, LengthDelimitedCodec>, Bytes>,
    mut sealing_key: SealingKey<NonceValue>,
) {
    TOKIO_RUNTIME.spawn(async move {
        loop {
            tokio::select! {
                _ = exit_rx.recv() => {
                    tracing::info!("serve writer receive");
                    break;
                },
                res = packet_rx.recv() => match res {
                    Some(packet) => {
                        let mut packet_buffer = match BINCODE_SERIALIZER.serialize(&packet) {
                            Ok(buffer) => buffer,
                            Err(err) => {
                                tracing::error!(?remote_device_id, ?err, "packet serialize failed");
                                break;
                            }
                        };

                        if let Err(err) = sealing_key
                            .seal_in_place_append_tag(ring::aead::Aad::empty(), &mut packet_buffer)
                        {
                            tracing::error!(?remote_device_id, ?err, "crypt packet data failed");
                            break;
                        }

                        if let Err(_) = sink.send(Bytes::from(packet_buffer)).await {
                            tracing::error!(?remote_device_id, "write to network stream failed");
                            break;
                        }
                    },
                    None => {
                        tracing::info!(?remote_device_id, "writer tx closed");
                        break;
                    }
                }
            }
        }

        endpoint.manually_close();
        tracing::info!(?remote_device_id, "write process exit");
    });
}

fn handle_message(endpoint: &Arc<EndPoint>, packet: EndPointMessagePacket) {
    match packet.typ {
        EndPointMessagePacketType::Request => match packet.message {
            EndPointMessage::GetDisplayInfoRequest(req) => {
                handle_call_message!(
                    endpoint,
                    packet.call_id,
                    req,
                    EndPointMessage::GetDisplayInfoResponse,
                    handle_get_display_info_request
                )
            }
            EndPointMessage::StartMediaTransmissionRequest(req) => {
                handle_call_message!(
                    endpoint,
                    packet.call_id,
                    req,
                    EndPointMessage::StartMediaTransmissionResponse,
                    handle_start_media_transmission_request
                )
            }
            _ => tracing::error!("handle_message: received unknown request message"),
        },
        EndPointMessagePacketType::Response => {
            if let Some(call_id) = packet.call_id {
                endpoint.set_call_reply(call_id, packet.message);
            } else {
                tracing::error!("handle_message: received response message without call_id")
            }
        }
        EndPointMessagePacketType::Push => match packet.message {
            EndPointMessage::VideoFrame(req) => {
                if let Some(tx) = endpoint.video_frame_tx.get() {
                    if let Err(err) = tx.try_send(req) {
                        if err.is_full() {
                            tracing::warn!(remote_device_id = ?endpoint.remote_device_id, "video frame queue is full");
                        }
                    }
                }
            }
            EndPointMessage::AudioFrame(req) => {
                if let Some(tx) = endpoint.audio_frame_tx.get() {
                    if let Err(err) = tx.try_send(req) {
                        if err.is_full() {
                            tracing::warn!(remote_device_id = ?endpoint.remote_device_id, "audio frame queue is full");
                        }
                    }
                }
            }
            EndPointMessage::Input(req) => {
                handle_push_message!(endpoint, req, handle_input);
            }
            _ => tracing::error!("handle_message: received unknown push message"),
        },
    }
}
