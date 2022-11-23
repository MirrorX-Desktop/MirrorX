pub mod handlers;
pub mod message;

use self::{
    handlers::{
        audio_frame::serve_audio_decode,
        input::{handle_keyboard, handle_mouse, handle_mouse_double_click},
        negotiate_desktop_params::handle_negotiate_desktop_params_request,
        video_frame::serve_video_decode,
    },
    message::*,
};
use crate::{
    api::endpoint::handlers::negotiate_finished::handle_negotiate_finished_request,
    component::{desktop::monitor::Monitor, frame::DesktopDecodeFrame},
    core_error,
    error::{CoreError, CoreResult},
    utility::nonce_value::NonceValue,
};
use bincode::{
    config::{LittleEndian, VarintEncoding, WithOtherEndian, WithOtherIntEncoding},
    DefaultOptions, Options,
};
use bytes::{Buf, Bytes, BytesMut};
use cpal::SampleFormat;
use dashmap::DashMap;
use futures::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use once_cell::sync::Lazy;
use ring::aead::{OpeningKey, SealingKey};
use std::{sync::Arc, time::Duration};
use tokio::{
    net::TcpStream,
    select,
    sync::{
        mpsc::{error::TrySendError, Receiver, Sender},
        Mutex,
    },
};
use tokio_util::codec::{Framed, LengthDelimitedCodec};
use tracing::Instrument;

const RECV_MESSAGE_TIMEOUT: Duration = Duration::from_secs(30);

pub static PASSIVE_ENDPOINT_CLIENTS: Lazy<DashMap<EndPointID, EndPointClient>> =
    Lazy::new(DashMap::new);

pub static PASSIVE_ENDPOINTS_MONITORS: Lazy<DashMap<EndPointID, Monitor>> = Lazy::new(DashMap::new);

static BINARY_SERIALIZER: Lazy<
    WithOtherIntEncoding<WithOtherEndian<DefaultOptions, LittleEndian>, VarintEncoding>,
> = Lazy::new(|| {
    bincode::DefaultOptions::new()
        .with_little_endian()
        .with_varint_encoding()
});

// todo: ttl
static NEGOTIATE_DESKTOP_PARAMS_RESPONSE_RECEIVERS: Lazy<
    DashMap<EndPointID, tokio::sync::oneshot::Sender<EndPointNegotiateDesktopParamsResponse>>,
> = Lazy::new(DashMap::new);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct EndPointID(i64, i64);

impl EndPointID {
    pub fn local_device_id(self) -> i64 {
        self.0
    }

    pub fn remote_device_id(self) -> i64 {
        self.1
    }
}

#[derive(Debug, Clone)]
pub struct EndPointClient {
    id: EndPointID,
    message_tx: Sender<Option<EndPointMessage>>,
    exit_tx: async_broadcast::Sender<()>,
    video_frame_rx: Arc<Mutex<Option<Receiver<EndPointVideoFrame>>>>,
    audio_decode_rx: Arc<Mutex<Option<Receiver<EndPointAudioFrame>>>>,
}

impl EndPointClient {
    pub async fn new(
        local_device_id: i64,
        remote_device_id: i64,
        opening_key: OpeningKey<NonceValue>,
        sealing_key: SealingKey<NonceValue>,
        visit_credentials: String,
        addr: &str,
    ) -> CoreResult<Self> {
        let mut stream = connect(&format!("{}:29000", addr)).await?;

        handshake(
            &mut stream,
            local_device_id,
            remote_device_id,
            visit_credentials,
        )
        .await?;

        let (exit_tx, exit_rx) = async_broadcast::broadcast(16);
        let (message_tx, message_rx) = tokio::sync::mpsc::channel(180);
        let (sink, stream) = stream.split();

        let (video_frame_tx, video_frame_rx) = tokio::sync::mpsc::channel(180);
        let (audio_frame_tx, audio_frame_rx) = tokio::sync::mpsc::channel(180);

        let client = EndPointClient {
            id: EndPointID(local_device_id, remote_device_id),
            message_tx,
            exit_tx: exit_tx.clone(),
            video_frame_rx: Arc::new(Mutex::new(Some(video_frame_rx))),
            audio_decode_rx: Arc::new(Mutex::new(Some(audio_frame_rx))),
        };

        serve_reader(
            client.clone(),
            exit_tx.clone(),
            exit_rx.clone(),
            stream,
            opening_key,
            video_frame_tx,
            audio_frame_tx,
        );

        serve_writer(client.id, exit_tx, exit_rx, sink, sealing_key, message_rx);

        Ok(client)
    }

    pub fn id(&self) -> EndPointID {
        self.id
    }

    pub fn negotiate_desktop_params(
        &self,
    ) -> CoreResult<tokio::sync::oneshot::Receiver<EndPointNegotiateDesktopParamsResponse>> {
        let req =
            EndPointMessage::NegotiateDesktopParamsRequest(EndPointNegotiateDesktopParamsRequest {
                video_codecs: vec![VideoCodec::H264],
            });

        let (tx, rx) = tokio::sync::oneshot::channel();

        self.send_message(req)?;

        NEGOTIATE_DESKTOP_PARAMS_RESPONSE_RECEIVERS.insert(self.id, tx);

        Ok(rx)
    }

    pub async fn negotiate_finish(
        &mut self,
        expected_frame_rate: u8,
    ) -> CoreResult<Receiver<DesktopDecodeFrame>> {
        let (video_render_tx, video_render_rx) = tokio::sync::mpsc::channel(120);

        let video_decode_tx = serve_video_decode(self.id, video_render_tx);
        if let Some(mut video_frame_rx) = self.video_frame_rx.lock().await.take() {
            tokio::spawn(async move {
                while let Some(video_frame) = video_frame_rx.recv().await {
                    if let Err(err) = video_decode_tx.try_send(video_frame) {
                        match err {
                            TrySendError::Full(_) => tracing::warn!("video decode tx is full!"),
                            TrySendError::Closed(_) => {
                                tracing::error!("video decode tx was closed");
                                return;
                            }
                        }
                    }
                }
            });
        }

        if let Some(audio_decode_rx) = self.audio_decode_rx.lock().await.take() {
            serve_audio_decode(self.id, audio_decode_rx);
        }

        let req = EndPointMessage::NegotiateFinishedRequest(EndPointNegotiateFinishedRequest {
            expected_frame_rate,
        });

        self.send_message(req)?;

        Ok(video_render_rx)
    }

    pub fn send_input_command(&self, events: Vec<InputEvent>) -> CoreResult<()> {
        let req = EndPointMessage::InputCommand(EndPointInput { events });
        self.send_message(req)
    }

    pub fn send_audio_frame(&self, frame: EndPointAudioFrame) -> CoreResult<()> {
        self.send_message(EndPointMessage::AudioFrame(frame))
    }

    pub fn send_video_frame(
        &self,
        width: i32,
        height: i32,
        pts: i64,
        buffer: &[u8],
    ) -> CoreResult<()> {
        let message = EndPointMessage::VideoFrame(EndPointVideoFrame {
            width,
            height,
            pts,
            buffer: buffer.to_vec(),
        });
        self.send_message(message)
    }

    fn send_message(&self, message: EndPointMessage) -> CoreResult<()> {
        if let Err(err) = self.message_tx.try_send(Some(message)) {
            match err {
                TrySendError::Full(_) => Err(CoreError::OutgoingMessageChannelFull),
                TrySendError::Closed(_) => Err(CoreError::OutgoingMessageChannelDisconnect),
            }
        } else {
            Ok(())
        }
    }

    pub fn close(&self) {
        let _ = self.exit_tx.try_broadcast(());
    }
}

async fn connect(addr: &str) -> CoreResult<Framed<TcpStream, LengthDelimitedCodec>> {
    let stream = tokio::time::timeout(Duration::from_secs(10), TcpStream::connect(addr)).await??;

    stream.set_nodelay(true)?;

    let stream = LengthDelimitedCodec::builder()
        .little_endian()
        .max_frame_length(32 * 1024 * 1024)
        .new_framed(stream);

    Ok(stream)
}

async fn handshake(
    stream: &mut Framed<TcpStream, LengthDelimitedCodec>,
    local_device_id: i64,
    remote_device_id: i64,
    visit_credentials: String,
) -> CoreResult<()> {
    let req = EndPointHandshakeRequest {
        device_id: local_device_id,
        visit_credentials,
    };

    let req_buffer = Bytes::from(BINARY_SERIALIZER.serialize(&req)?);

    stream.send(req_buffer).await?;

    let resp_buffer = tokio::time::timeout(RECV_MESSAGE_TIMEOUT, stream.next())
        .await?
        .ok_or(core_error!("stream was closed"))?
        .map_err(|err| core_error!("stream read failed ({})", err))?;

    let handshake_resp: EndPointHandshakeResponse =
        BINARY_SERIALIZER.deserialize_from(resp_buffer.reader())?;

    if handshake_resp.remote_device_id != remote_device_id {
        return Err(core_error!(
            "signaling server matched incorrect stream pair"
        ));
    }

    Ok(())
}

fn serve_reader(
    client: EndPointClient,
    exit_tx: async_broadcast::Sender<()>,
    mut exit_rx: async_broadcast::Receiver<()>,
    mut stream: SplitStream<Framed<TcpStream, LengthDelimitedCodec>>,
    mut opening_key: OpeningKey<NonceValue>,
    video_frame_tx: Sender<EndPointVideoFrame>,
    audio_frame_tx: Sender<EndPointAudioFrame>,
) {
    tokio::spawn(async move {
        let span = tracing::info_span!("serve reader", id = ?client.id());
        let _entered = span.enter();

        loop {
            let mut endpoint_message_bytes = match async {
                select! {
                    _ = exit_rx.recv() => {
                        Err(core_error!("read processor receive exit tx signal"))
                    }
                    message = stream.next() => match message {
                        Some(packet) => match packet {
                            Ok(endpoint_message_bytes) => Ok(endpoint_message_bytes),
                            Err(err) => Err(core_error!("read stream failed ({})", err)),
                        },
                        None => Err(core_error!("stream was closed")),
                    }
                }
            }
            .instrument(span.clone())
            .await
            {
                Ok(message_bytes) => message_bytes,
                Err(err) => {
                    tracing::error!(?err, "exit read loop");
                    break;
                }
            };

            match open_packet(&mut opening_key, &mut endpoint_message_bytes) {
                Ok(message) => {
                    handle_message(client.clone(), message, &video_frame_tx, &audio_frame_tx);
                }
                Err(err) => {
                    tracing::error!(?err, "open packet failed");
                }
            }
        }

        let _ = exit_tx.broadcast(()).await;
        tracing::info!("read process exit");
    });
}

fn serve_writer(
    id: EndPointID,
    exit_tx: async_broadcast::Sender<()>,
    mut exit_rx: async_broadcast::Receiver<()>,
    mut sink: SplitSink<Framed<TcpStream, LengthDelimitedCodec>, Bytes>,
    mut sealing_key: SealingKey<NonceValue>,
    mut message_rx: tokio::sync::mpsc::Receiver<Option<EndPointMessage>>,
) {
    tokio::spawn(async move {
        let span = tracing::info_span!("serve writer", id = ?id);
        let _entered = span.enter();

        loop {
            let endpoint_message = match async {
                select! {
                    _ = exit_rx.recv() => {
                        Err(core_error!("write processor receive exit tx signal"))
                    }
                    message = message_rx.recv() => match message {
                        Some(packet) => match packet {
                            Some(endpoint_message_bytes) => Ok(endpoint_message_bytes),
                            None => Err(core_error!("inner processor notify exit")),
                        },
                        None => Err(core_error!("stream was closed")),
                    }
                }
            }
            .instrument(span.clone())
            .await
            {
                Ok(message) => message,
                Err(err) => {
                    tracing::error!(?err, "exit write loop");
                    break;
                }
            };

            let buffer = match seal_packet(&mut sealing_key, &endpoint_message) {
                Ok(buffer) => buffer,
                Err(err) => {
                    tracing::error!(?err, "seal packet failed");
                    break;
                }
            };

            if let Err(err) = sink.send(buffer).await {
                tracing::error!(?err, "sink send failed");
                break;
            }
        }

        let _ = exit_tx.broadcast(()).await;
        tracing::info!("write process exit");
    });
}

fn open_packet(
    opening_key: &mut OpeningKey<NonceValue>,
    buffer: &mut BytesMut,
) -> CoreResult<EndPointMessage> {
    let opened_buffer = opening_key.open_in_place(ring::aead::Aad::empty(), buffer)?;
    let message = BINARY_SERIALIZER.deserialize(opened_buffer)?;
    Ok(message)
}

fn seal_packet(
    sealing_key: &mut SealingKey<NonceValue>,
    message: &EndPointMessage,
) -> CoreResult<Bytes> {
    let mut packet_buffer = BINARY_SERIALIZER.serialize(message)?;
    sealing_key.seal_in_place_append_tag(ring::aead::Aad::empty(), &mut packet_buffer)?;
    Ok(Bytes::from(packet_buffer))
}

fn handle_message(
    client: EndPointClient,
    message: EndPointMessage,
    video_frame_tx: &Sender<EndPointVideoFrame>,
    audio_frame_tx: &Sender<EndPointAudioFrame>,
) {
    match message {
        EndPointMessage::Error => {
            // handle_error(active_device_id, passive_device_id);
        }
        EndPointMessage::NegotiateDesktopParamsRequest(req) => {
            tokio::spawn(async move { handle_negotiate_desktop_params_request(client, req) });
        }
        EndPointMessage::NegotiateDesktopParamsResponse(resp) => {
            if let Some((_, tx)) = NEGOTIATE_DESKTOP_PARAMS_RESPONSE_RECEIVERS.remove(&client.id) {
                let _ = tx.send(resp);
            }
        }
        EndPointMessage::NegotiateFinishedRequest(_) => {
            handle_negotiate_finished_request(client);
        }
        EndPointMessage::VideoFrame(video_frame) => {
            // handle_video_frame(client.id, video_frame);
            if let Err(err) = video_frame_tx.try_send(video_frame) {
                tracing::error!(%err, "endpoint video frame message channel send failed");
            }
        }
        EndPointMessage::AudioFrame(audio_frame) => {
            if let Err(err) = audio_frame_tx.try_send(audio_frame) {
                tracing::error!(%err, "endpoint audio frame message channel send failed");
            }
        }
        EndPointMessage::InputCommand(input_event) => {
            let mut i = 0;
            while i < input_event.events.len() {
                if input_event.events.len() - i >= 4 {
                    if let InputEvent::Mouse(MouseEvent::Up(key1, x1, y1)) = input_event.events[i] {
                        if let InputEvent::Mouse(MouseEvent::Up(key2, x2, y2)) =
                            input_event.events[i + 1]
                        {
                            if let InputEvent::Mouse(MouseEvent::Up(key3, x3, y3)) =
                                input_event.events[i + 2]
                            {
                                if let InputEvent::Mouse(MouseEvent::Up(key4, x4, y4)) =
                                    input_event.events[i + 3]
                                {
                                    if (key1 == key2 && key2 == key3 && key3 == key4)
                                        && (x1.max(x2).max(x3).max(x4) - x1.min(x2).min(x3).min(x4))
                                            < 5.0
                                        && (y1.max(y2).max(y3).max(y4) - y1.min(y2).min(y3).min(y4))
                                            < 5.0
                                    {
                                        if let Some(entry) =
                                            PASSIVE_ENDPOINTS_MONITORS.get(&client.id)
                                        {
                                            handle_mouse_double_click(
                                                key1,
                                                (x1 + x2 + x3 + x4) / 4.0,
                                                (y1 + y2 + y3 + y4) / 4.0,
                                                entry.value(),
                                            );
                                        }
                                        i += 4;
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }

                match input_event.events[i] {
                    InputEvent::Mouse(event) => {
                        if let Some(entry) = PASSIVE_ENDPOINTS_MONITORS.get(&client.id) {
                            handle_mouse(event, entry.value());
                        }
                    }
                    InputEvent::Keyboard(event) => handle_keyboard(event),
                }

                i += 1;
            }
        }
    }
}
