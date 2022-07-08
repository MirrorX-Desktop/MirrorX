use super::{
    ffi::create_callback_fn,
    handler::{handle_get_display_info_request, handle_start_media_transmission_request},
    message::{
        AudioFrame, EndPointMessage, EndPointMessagePacket, EndPointMessagePacketType,
        GetDisplayInfoRequest, GetDisplayInfoResponse, StartMediaTransmissionRequest,
        StartMediaTransmissionResponse, VideoFrame,
    },
};
use crate::{
    component::{
        audio_decoder::audio_decoder::AudioDecoder,
        audio_encoder::audio_encoder::AudioEncoder,
        desktop::{Duplicator, Frame},
        video_decoder::{DecodedFrame, VideoDecoder},
        video_encoder::VideoEncoder,
    },
    error::MirrorXError,
    socket::endpoint::handler::{handle_audio_frame, handle_video_frame},
    utility::{nonce_value::NonceValue, runtime::TOKIO_RUNTIME, serializer::BINCODE_SERIALIZER},
};
use anyhow::anyhow;
use bincode::Options;
use bytes::Bytes;
use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    InputCallbackInfo, OutputCallbackInfo, SampleFormat, SampleRate, SupportedStreamConfigRange,
};
use crossbeam::channel::TrySendError;
use dashmap::DashMap;
use futures::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use once_cell::sync::{Lazy, OnceCell};
use ring::aead::{OpeningKey, SealingKey};
use rtrb::{Consumer, Producer, RingBuffer};
use scopeguard::defer;
use std::{
    collections::HashMap,
    os::raw::c_void,
    panic::UnwindSafe,
    sync::{
        atomic::{AtomicU16, Ordering},
        Arc,
    },
    time::Duration,
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpStream, ToSocketAddrs},
    sync::Mutex,
    time::timeout,
};
use tokio_util::codec::{Framed, LengthDelimitedCodec};
use tracing::{error, info, warn};

const CALL_TIMEOUT: Duration = Duration::from_secs(5);

pub static ENDPOINTS: Lazy<DashMap<String, Arc<EndPoint>>> = Lazy::new(|| DashMap::new());

macro_rules! make_endpoint_call {
    ($name:tt, $req_type:ident, $req_message_type:path, $resp_type:ident, $resp_message_type:path) => {
        pub async fn $name(&self, req: $req_type) -> Result<$resp_type, MirrorXError> {
            let reply = self.call($req_message_type(req), CALL_TIMEOUT).await?;

            if let $resp_message_type(message) = reply {
                Ok(message)
            } else {
                Err(MirrorXError::EndPointError(self.remote_device_id.clone()))
            }
        }
    };
}

macro_rules! handle_call_message {
    ($endpoint:expr, $call_id:expr, $req:tt, $resp_type:path, $handler:tt) => {{
        if let Some(call_id) = $call_id {
            let resp_message = match $handler($endpoint, $req).await {
                Ok(resp) => $resp_type(resp),
                Err(err) => {
                    error!(?err, "handle_call_message: handler '{}' returns error", stringify!($handler));
                    EndPointMessage::Error
                }
            };

            if let Err(err) = $endpoint.reply(call_id,resp_message).await{
                error!(?err, remote_device_id = ?$endpoint.remote_device_id(), "handle_call_message: handler '{}' reply message failed", stringify!($handler));
            }
        } else {
            error!("handle_call_message: received request message '{}' without call id", stringify!($req));
        }
    }};
}

macro_rules! handle_push_message {
     ($endpoint:expr, $req:tt, $handler:tt) => {{
        if let Err(err) = $handler($endpoint, $req).await {
            error!(?err, remote_device_id = ?$endpoint.remote_device_id(), "handle_push_message: handler '{}' returns error", stringify!($handler));
        }
    }};
}

pub struct EndPoint {
    local_device_id: String,
    remote_device_id: String,
    atomic_call_id: AtomicU16,
    call_reply_tx_map: DashMap<u16, tokio::sync::oneshot::Sender<EndPointMessage>>,
    packet_tx: tokio::sync::mpsc::Sender<EndPointMessagePacket>,
    video_frame_tx: OnceCell<crossbeam::channel::Sender<VideoFrame>>,
    audio_frame_tx: OnceCell<crossbeam::channel::Sender<AudioFrame>>,
    video_process_exit_tx: OnceCell<crossbeam::channel::Sender<()>>,
    audio_process_exit_tx: OnceCell<crossbeam::channel::Sender<()>>,
}

impl EndPoint {
    pub fn remote_device_id<'a>(&'a self) -> &'a str {
        &self.remote_device_id
    }

    pub fn local_device_id<'a>(&'a self) -> &'a str {
        &self.local_device_id
    }
}

impl EndPoint {
    async fn call(
        &self,
        message: EndPointMessage,
        duration: Duration,
    ) -> Result<EndPointMessage, MirrorXError> {
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

        timeout(duration, async move {
            if let Err(err) = self.send(packet).await {
                return Err(err);
            }

            match rx.await {
                Ok(message) => Ok(message),
                Err(_) => Err(MirrorXError::Timeout),
            }
        })
        .await
        .map_err(|_| MirrorXError::Timeout)?
    }

    async fn reply(&self, call_id: u16, message: EndPointMessage) -> Result<(), MirrorXError> {
        let packet = EndPointMessagePacket {
            typ: EndPointMessagePacketType::Response,
            call_id: Some(call_id),
            message,
        };

        self.send(packet).await
    }

    async fn send(&self, packet: EndPointMessagePacket) -> Result<(), MirrorXError> {
        self.packet_tx
            .try_send(packet)
            .map_err(|err| MirrorXError::Other(anyhow!(err)))
    }

    fn set_call_reply(&self, call_id: u16, message: EndPointMessage) {
        self.remove_call(call_id).map(|tx| {
            if let Err(_) = tx.send(message) {
                error!(remote_device_id=?self.remote_device_id,"set_call_reply: set reply failed")
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

    pub async fn start_video_capture_process(&self) -> Result<(), MirrorXError> {
        let exit_tx =
            start_video_capture_process(self.remote_device_id.clone(), self.packet_tx.clone())
                .await?;

        let _ = self.video_process_exit_tx.set(exit_tx);

        Ok(())
    }

    pub async fn start_audio_capture_process(&self) -> Result<(), MirrorXError> {
        let exit_tx =
            start_audio_capture_process(self.remote_device_id.clone(), self.packet_tx.clone())
                .await?;

        let _ = self.audio_process_exit_tx.set(exit_tx);

        Ok(())
    }

    pub async fn start_video_render_process(
        &self,
        texture_id: i64,
        video_texture_ptr: i64,
        update_frame_callback_ptr: i64,
    ) -> Result<(), MirrorXError> {
        let (decoder_name, options) = if cfg!(target_os = "macos") {
            ("h264", HashMap::new())
        } else if cfg!(target_os = "windows") {
            (
                "h264_qsv",
                HashMap::from([("async_depth", "1"), ("gpu_copy", "on")]),
            )
        } else {
            return Err(MirrorXError::Other(anyhow::anyhow!(
                "unsupport platform decode"
            )));
        };

        let decoder = VideoDecoder::new(decoder_name, options)?;
        let (video_frame_tx, video_frame_rx) = crossbeam::channel::bounded::<VideoFrame>(120);
        let (decoded_frame_tx, decoded_frame_rx) = crossbeam::channel::bounded::<DecodedFrame>(120);

        std::thread::spawn(move || loop {
            let video_frame = match video_frame_rx.recv() {
                Ok(frame) => frame,
                Err(_) => {
                    info!("video frame decode process exit");
                    break;
                }
            };

            let frames = match decoder.decode(video_frame.buffer, 0, 0) {
                Ok(frames) => frames,
                Err(err) => {
                    error!(?err, "video decode failed");
                    break;
                }
            };

            for frame in frames {
                if let Err(_) = decoded_frame_tx.try_send(frame) {
                    info!("video frame decode process exit");
                    break;
                }
            }
        });

        start_video_render_process(
            self.remote_device_id.clone(),
            decoded_frame_rx,
            texture_id,
            video_texture_ptr,
            update_frame_callback_ptr,
        );

        let _ = self.video_frame_tx.set(video_frame_tx);

        Ok(())
    }

    pub async fn start_audio_play_process(&self) -> Result<(), MirrorXError> {
        let mut audio_decoder = AudioDecoder::new()?;
        let (audio_frame_tx, audio_frame_rx) = crossbeam::channel::bounded::<AudioFrame>(120);
        let (mut samples_tx, samples_rx) = RingBuffer::new(48000 * 2);

        std::thread::spawn(move || loop {
            let audio_frame = match audio_frame_rx.recv() {
                Ok(audio_frame) => audio_frame,
                Err(_) => {
                    info!("audio play process thread exit");
                    break;
                }
            };

            let audio_buffer =
                match audio_decoder.decode(&audio_frame.buffer, audio_frame.frame_size) {
                    Ok(pcm) => pcm,
                    Err(err) => {
                        error!(?err, "audio decoder decode failed");
                        break;
                    }
                };

            for v in audio_buffer {
                if let Err(_) = samples_tx.push(v) {
                    break;
                }
            }
        });

        let exit_tx = start_audio_play_process(self.remote_device_id.clone(), samples_rx).await?;

        let _ = self.audio_frame_tx.set(audio_frame_tx);
        let _ = self.audio_process_exit_tx.set(exit_tx);

        Ok(())
    }

    pub async fn enqueue_video_frame(&self, video_frame: VideoFrame) {
        if let Some(tx) = self.video_frame_tx.get() {
            if let Err(err) = tx.try_send(video_frame) {
                if err.is_full() {
                    warn!(remote_device_id = ?self.remote_device_id, "video frame queue is full");
                }
            }
        }
    }

    pub async fn enqueue_audio_frame(&self, audio_frame: AudioFrame) {
        if let Some(tx) = self.audio_frame_tx.get() {
            if let Err(err) = tx.try_send(audio_frame) {
                if err.is_full() {
                    warn!(remote_device_id = ?self.remote_device_id, "audio frame queue is full");
                }
            }
        }
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
}

impl Drop for EndPoint {
    fn drop(&mut self) {
        info!(remote_device_id = ?self.remote_device_id, "endpoint dropped");
    }
}

pub async fn connect<A>(
    addr: A,
    is_active_side: bool,
    local_device_id: String,
    remote_device_id: String,
    opening_key: OpeningKey<NonceValue>,
    sealing_key: SealingKey<NonceValue>,
) -> Result<(), MirrorXError>
where
    A: ToSocketAddrs,
{
    let mut stream = timeout(Duration::from_secs(10), TcpStream::connect(addr))
        .await
        .map_err(|_| MirrorXError::Timeout)?
        .map_err(|err| MirrorXError::IO(err))?;

    stream
        .set_nodelay(true)
        .map_err(|err| MirrorXError::IO(err))?;

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
        return Err(MirrorXError::Other(anyhow::anyhow!(
            "active device id bytes length is not 10"
        )));
    }

    let passive_device_id_buf = passive_device_id.as_bytes();
    if passive_device_id_buf.len() != 10 {
        return Err(MirrorXError::Other(anyhow::anyhow!(
            "passive device id bytes length is not 10"
        )));
    }

    stream
        .write(active_device_id_buf)
        .await
        .map_err(|err| MirrorXError::IO(err))?;
    stream
        .write(passive_device_id_buf)
        .await
        .map_err(|err| MirrorXError::IO(err))?;

    let mut handshake_response_buf = [0u8; 1];
    timeout(
        Duration::from_secs(60),
        stream.read_exact(&mut handshake_response_buf),
    )
    .await
    .map_err(|_| MirrorXError::Timeout)?
    .map_err(|err| MirrorXError::IO(err))?;

    if handshake_response_buf[0] != 1 {
        return Err(MirrorXError::EndPointError(String::from(
            "handshake failed",
        )));
    }

    let framed_stream = LengthDelimitedCodec::builder()
        .little_endian()
        .max_frame_length(16 * 1024 * 1024)
        .new_framed(stream);

    let (sink, stream) = framed_stream.split();

    let (packet_tx, packet_rx) = tokio::sync::mpsc::channel(128);

    let endpoint = Arc::new(EndPoint {
        local_device_id,
        remote_device_id: remote_device_id.clone(),
        atomic_call_id: AtomicU16::new(0),
        call_reply_tx_map: DashMap::new(),
        packet_tx,
        video_frame_tx: OnceCell::new(),
        audio_frame_tx: OnceCell::new(),
        video_process_exit_tx: OnceCell::new(),
        audio_process_exit_tx: OnceCell::new(),
    });

    serve_reader(endpoint.clone(), stream, opening_key);
    serve_writer(remote_device_id.clone(), packet_rx, sink, sealing_key);

    ENDPOINTS.insert(remote_device_id, endpoint);

    Ok(())
}

fn serve_reader(
    endpoint: Arc<EndPoint>,
    mut stream: SplitStream<Framed<TcpStream, LengthDelimitedCodec>>,
    mut opening_key: OpeningKey<NonceValue>,
) {
    TOKIO_RUNTIME.spawn(async move {
        loop {
            let mut packet_bytes = match stream.next().await {
                Some(res) => match res {
                    Ok(packet_bytes) => packet_bytes,
                    Err(err) => {
                        error!(remote_device_id=?endpoint.remote_device_id(), ?err, "reader: read packet failed");
                        break;
                    }
                },
                None => {
                    info!(remote_device_id=?endpoint.remote_device_id(), "reader: remote closed");
                    break;
                }
            };

            let opened_packet_bytes =
                match opening_key.open_in_place(ring::aead::Aad::empty(), &mut packet_bytes) {
                    Ok(v) => v,
                    Err(err) => {
                        error!(remote_device_id=?endpoint.remote_device_id(), ?err, "reader: decrypt packet failed");
                        break;
                    }
                };

            let packet = match BINCODE_SERIALIZER
                .deserialize::<EndPointMessagePacket>(&opened_packet_bytes)
            {
                Ok(packet) => packet,
                Err(err) => {
                    error!(remote_device_id=?endpoint.remote_device_id(), ?err, "reader: deserialize packet failed");
                    break;
                }
            };

            let endpoint = endpoint.clone();
            TOKIO_RUNTIME.spawn(async move {
                handle_message(endpoint, packet).await;
            });
        }

        ENDPOINTS.remove(endpoint.remote_device_id());
        info!(remote_device_id=?endpoint.remote_device_id(), "reader: exit");
    });
}

fn serve_writer(
    remote_device_id: String,
    mut packet_rx: tokio::sync::mpsc::Receiver<EndPointMessagePacket>,
    mut sink: SplitSink<Framed<TcpStream, LengthDelimitedCodec>, Bytes>,
    mut sealing_key: SealingKey<NonceValue>,
) {
    TOKIO_RUNTIME.spawn(async move {
        loop {
            let packet = match packet_rx.recv().await {
                Some(buffer) => buffer,
                None => {
                    info!(?remote_device_id, "writer: EndPointMessagePacket tx closed");
                    break;
                }
            };

            let mut packet_buffer = match BINCODE_SERIALIZER.serialize(&packet) {
                Ok(buffer) => buffer,
                Err(err) => {
                    error!(?remote_device_id, ?err, "writer: packet serialize failed");
                    break;
                }
            };

            if let Err(err) =
                sealing_key.seal_in_place_append_tag(ring::aead::Aad::empty(), &mut packet_buffer)
            {
                error!(?remote_device_id, ?err, "writer: crypt buffer failed");
                break;
            }

            if let Err(_) = sink.send(Bytes::from(packet_buffer)).await {
                error!(?remote_device_id, "writer: send failed");
                break;
            }
        }

        ENDPOINTS.remove(&remote_device_id);
        info!(?remote_device_id, "writer: exit");
    });
}

async fn handle_message(endpoint: Arc<EndPoint>, packet: EndPointMessagePacket) {
    match packet.typ {
        EndPointMessagePacketType::Request => match packet.message {
            EndPointMessage::GetDisplayInfoRequest(req) => {
                handle_call_message!(
                    &endpoint,
                    packet.call_id,
                    req,
                    EndPointMessage::GetDisplayInfoResponse,
                    handle_get_display_info_request
                )
            }
            EndPointMessage::StartMediaTransmissionRequest(req) => {
                handle_call_message!(
                    &endpoint,
                    packet.call_id,
                    req,
                    EndPointMessage::StartMediaTransmissionResponse,
                    handle_start_media_transmission_request
                )
            }
            _ => error!("handle_message: received unknown request message"),
        },
        EndPointMessagePacketType::Response => {
            if let Some(call_id) = packet.call_id {
                endpoint.set_call_reply(call_id, packet.message);
            } else {
                error!("handle_message: received response message without call_id")
            }
        }
        EndPointMessagePacketType::Push => match packet.message {
            EndPointMessage::VideoFrame(req) => {
                handle_push_message!(&endpoint, req, handle_video_frame);
            }
            EndPointMessage::AudioFrame(req) => {
                handle_push_message!(&endpoint, req, handle_audio_frame);
            }
            _ => error!("handle_message: received unknown push message"),
        },
    }
}

async fn start_audio_capture_process(
    remote_device_id: String,
    packet_tx: tokio::sync::mpsc::Sender<EndPointMessagePacket>,
) -> Result<crossbeam::channel::Sender<()>, MirrorXError> {
    let (exit_tx, exit_rx) = crossbeam::channel::bounded(1);
    let (inner_error_tx, inner_error_rx) = tokio::sync::oneshot::channel();

    let _ = std::thread::Builder::new()
        .name(format!("audio_capture_process:{}", remote_device_id))
        .spawn(move || {
            let host = cpal::default_host();

            let device = match host.default_output_device() {
                Some(device) => device,
                None => {
                    let _ = inner_error_tx.send(Some(MirrorXError::Other(anyhow::anyhow!(
                        "default audio output device is null"
                    ))));
                    return;
                }
            };

            info!(name=?device.name(),"select default audio output device");

            let supported_configs = match device.supported_output_configs() {
                Ok(config) => config,
                Err(err) => {
                    let _ = inner_error_tx.send(Some(MirrorXError::Other(anyhow::anyhow!(
                        "get audio device supported config failed ({})",
                        err
                    ))));
                    return;
                }
            };

            let supported_config_vec: Vec<SupportedStreamConfigRange> =
                supported_configs.into_iter().collect();

            if supported_config_vec.len() == 0 {
                let _ = inner_error_tx.send(Some(MirrorXError::Other(anyhow::anyhow!(
                    "no supported audio device output config"
                ))));
                return;
            }

            let sample_format = supported_config_vec[0].sample_format();

            if sample_format != SampleFormat::F32 {
                let _ = inner_error_tx.send(Some(MirrorXError::Other(anyhow::anyhow!(
                    "unsupported audio sample format {}",
                    sample_format.sample_size()
                ))));
                return;
            }

            let output_config = supported_config_vec[0]
                .clone()
                .with_sample_rate(SampleRate(48000))
                .config();

            let mut audio_encoder = match AudioEncoder::new() {
                Ok(encoder) => encoder,
                Err(err) => {
                    let _ = inner_error_tx.send(Some(err));
                    return;
                }
            };

            let mut audio_epoch: Option<std::time::Instant> = None;

            let input_callback = move |data: &[f32], info: &InputCallbackInfo| unsafe {
                let elpased = if let Some(instant) = audio_epoch {
                    instant.elapsed().as_millis()
                } else {
                    let instant = std::time::Instant::now();
                    audio_epoch = Some(instant);
                    0
                };

                match audio_encoder.encode(data) {
                    Ok(buffer) => {
                        let _ = packet_tx.try_send(EndPointMessagePacket {
                            typ: EndPointMessagePacketType::Push,
                            call_id: None,
                            message: EndPointMessage::AudioFrame(AudioFrame {
                                buffer,
                                frame_size: data.len() as u16,
                                elpased,
                            }),
                        });
                    }
                    Err(err) => {
                        error!(?err, "audio encoder encode failed");
                    }
                }
            };

            let err_callback = |err| error!(?err, "error occurred on the output input stream");

            let loopback_stream =
                match device.build_input_stream(&output_config, input_callback, err_callback) {
                    Ok(stream) => stream,
                    Err(err) => {
                        let _ = inner_error_tx.send(Some(MirrorXError::Other(anyhow::anyhow!(
                            "build input stream failed ({})",
                            err
                        ))));
                        return;
                    }
                };

            if let Err(err) = loopback_stream.play() {
                let _ = inner_error_tx.send(Some(MirrorXError::Other(anyhow::anyhow!(
                    "loop back stream play failed ({})",
                    err
                ))));
                return;
            }

            defer! {
                let _ = loopback_stream.pause();
                info!("audio capture process exit");
            }

            let _ = inner_error_tx.send(None);
            let _ = exit_rx.recv();
        });

    match inner_error_rx.await {
        Ok(inner_err) => match inner_err {
            Some(err) => Err(err),
            None => Ok(exit_tx),
        },
        Err(err) => Err(MirrorXError::Other(anyhow::anyhow!(
            "receive start_audio_capture_process result failed ({})",
            err
        ))),
    }
}

async fn start_audio_play_process(
    remote_device_id: String,
    mut samples_rx: Consumer<f32>,
) -> Result<crossbeam::channel::Sender<()>, MirrorXError> {
    let (exit_tx, exit_rx) = crossbeam::channel::bounded(1);
    let (inner_error_tx, inner_error_rx) = tokio::sync::oneshot::channel();

    let _ = std::thread::Builder::new()
        .name(format!("audio_play_process:{}", remote_device_id))
        .spawn(move || {
            let host = cpal::default_host();

            let device = match host.default_output_device() {
                Some(device) => device,
                None => {
                    let _ = inner_error_tx.send(Some(MirrorXError::Other(anyhow::anyhow!(
                        "default audio output device is null"
                    ))));
                    return;
                }
            };

            info!(name=?device.name(),"select default audio output device");

            let supported_configs = match device.supported_output_configs() {
                Ok(config) => config,
                Err(err) => {
                    let _ = inner_error_tx.send(Some(MirrorXError::Other(anyhow::anyhow!(
                        "get audio device supported config failed ({})",
                        err
                    ))));
                    return;
                }
            };

            let supported_config_vec: Vec<SupportedStreamConfigRange> =
                supported_configs.into_iter().collect();

            if supported_config_vec.len() == 0 {
                let _ = inner_error_tx.send(Some(MirrorXError::Other(anyhow::anyhow!(
                    "no supported audio device output config"
                ))));
                return;
            }

            let output_config = if let Some(config) = supported_config_vec
                .iter()
                .find(|config| config.max_sample_rate() == SampleRate(48000))
            {
                config.clone().with_sample_rate(SampleRate(48000)).config()
            } else {
                let _ = inner_error_tx.send(Some(MirrorXError::Other(anyhow::anyhow!(
                    "no supported audio device output config with sample rate 48000"
                ))));
                return;
            };

            let sample_format = supported_config_vec[0].sample_format();

            if sample_format != SampleFormat::F32 {
                let _ = inner_error_tx.send(Some(MirrorXError::Other(anyhow::anyhow!(
                    "unsupported audio sample format {}",
                    sample_format.sample_size()
                ))));
                return;
            }

            let input_callback = move |data: &mut [f32], info: &OutputCallbackInfo| unsafe {
                for b in data {
                    *b = match samples_rx.pop() {
                        Ok(v) => v,
                        Err(_) => Default::default(),
                    };
                }
            };

            let err_callback = |err| error!(?err, "error occurred on the output audio stream");

            let loopback_stream =
                match device.build_output_stream(&output_config, input_callback, err_callback) {
                    Ok(stream) => stream,
                    Err(err) => {
                        let _ = inner_error_tx.send(Some(MirrorXError::Other(anyhow::anyhow!(
                            "build output stream failed ({})",
                            err
                        ))));
                        return;
                    }
                };

            if let Err(err) = loopback_stream.play() {
                let _ = inner_error_tx.send(Some(MirrorXError::Other(anyhow::anyhow!(
                    "loop back stream play failed ({})",
                    err
                ))));
                return;
            }

            defer! {
                let _ = loopback_stream.pause();
                info!("audio play process exit");
            }

            let _ = inner_error_tx.send(None);
            let _ = exit_rx.recv();
        });

    match inner_error_rx.await {
        Ok(inner_error) => match inner_error {
            Some(err) => Err(err),
            None => Ok(exit_tx),
        },
        Err(err) => Err(MirrorXError::Other(anyhow::anyhow!(
            "receive start_audio_play_process result failed ({})",
            err
        ))),
    }
}

async fn start_video_capture_process(
    remote_device_id: String,
    packet_tx: tokio::sync::mpsc::Sender<EndPointMessagePacket>,
) -> Result<crossbeam::channel::Sender<()>, MirrorXError> {
    let encoder_name = if cfg!(target_os = "macos") {
        "h264_videotoolbox"
    } else if cfg!(target_os = "windows") {
        "libx264"
    } else {
        panic!("unsupported platform")
    };

    let mut video_encoder = VideoEncoder::new(encoder_name, 60, 1920, 1080)?;

    video_encoder.set_opt("profile", "high", 0)?;
    video_encoder.set_opt("level", "5.2", 0)?;

    if encoder_name == "libx264" {
        video_encoder.set_opt("preset", "ultrafast", 0)?;
        video_encoder.set_opt("tune", "zerolatency", 0)?;
        video_encoder.set_opt("sc_threshold", "499", 0)?;
    } else {
        video_encoder.set_opt("realtime", "1", 0)?;
        video_encoder.set_opt("allow_sw", "0", 0)?;
    }

    let av_packet_rx = video_encoder.open()?;

    let (mut desktop_duplicator, capture_frame_rx) = Duplicator::new(60)?;

    std::thread::spawn(move || {
        // make sure the media_transmission after start_media_transmission send
        std::thread::sleep(Duration::from_secs(1));

        if let Err(err) = desktop_duplicator.start() {
            error!(?err, "desktop_duplicator: start capture failed");
            return;
        }

        info!("desktop_duplicator: start capture");

        loop {
            let capture_frame = match capture_frame_rx.recv() {
                Ok(frame) => frame,
                Err(err) => {
                    error!(?err, "capture_frame_rx: closed");
                    break;
                }
            };

            // encode will block current thread until capture_frame released (after FFMpeg API 'avcodec_send_frame' finished)
            if let Err(err) = video_encoder.encode(capture_frame) {
                error!(?err, "video_encoder: encode failed");
                break;
            }
        }

        desktop_duplicator.stop();
        info!("desktop_duplicator: capture stopped");
    });

    std::thread::spawn(move || loop {
        match av_packet_rx.recv() {
            Ok(av_packet) => {
                let packet = EndPointMessagePacket {
                    typ: EndPointMessagePacketType::Push,
                    call_id: None,
                    message: EndPointMessage::VideoFrame(VideoFrame {
                        buffer: av_packet.0,
                        timestamp: 0,
                    }),
                };

                if let Err(err) = packet_tx.try_send(packet) {
                    match err {
                        tokio::sync::mpsc::error::TrySendError::Full(_) => {
                            warn!("packet_tx: full")
                        }
                        tokio::sync::mpsc::error::TrySendError::Closed(_) => {
                            error!("packet_tx: closed");
                            break;
                        }
                    }
                }
            }
            Err(_) => {
                error!("av_packet_rx: closed");
                break;
            }
        };
    });

    let (tx, rx) = crossbeam::channel::bounded(1);
    Ok(tx)
}

fn start_video_render_process(
    remote_device_id: String,
    decoded_video_frame_rx: crossbeam::channel::Receiver<DecodedFrame>,
    texture_id: i64,
    video_texture_ptr: i64,
    update_frame_callback_ptr: i64,
) {
    let update_callback_fn = unsafe { create_callback_fn(update_frame_callback_ptr) };

    let _ = std::thread::Builder::new()
        .name(format!("video_render_process:{}", remote_device_id))
        .spawn(move || loop {
            let decoded_video_frame = match decoded_video_frame_rx.recv() {
                Ok(frame) => frame,
                Err(_) => {
                    info!("video frame render process exit");
                    break;
                }
            };

            #[cfg(target_os = "macos")]
            unsafe {
                update_callback_fn(
                    texture_id,
                    video_texture_ptr as *mut c_void,
                    decoded_video_frame.0,
                );
            }

            #[cfg(target_os = "windows")]
            unsafe {
                update_callback_fn(
                    video_texture_ptr as *mut c_void,
                    decoded_video_frame.0.as_ptr(),
                    1920,
                    1080,
                );
            }
        });
}
