pub mod flutter_message;
pub mod handlers;
pub mod message;

use self::{
    handlers::{
        input::{handle_keyboard, handle_mouse},
        negotiate_desktop_params::handle_negotiate_desktop_params_request,
    },
    message::*,
};
use crate::{
    api::endpoint::handlers::{
        audio_frame::handle_audio_frame, negotiate_finished::handle_negotiate_finished_request,
        video_frame::handle_video_frame,
    },
    component::desktop::monitor::Monitor,
    core_error,
    error::CoreResult,
    utility::nonce_value::NonceValue,
};
use bincode::{
    config::{LittleEndian, VarintEncoding, WithOtherEndian, WithOtherIntEncoding},
    DefaultOptions, Options,
};
use bytes::{Buf, Bytes, BytesMut};
use dashmap::DashMap;
use futures::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use once_cell::sync::Lazy;
use ring::aead::{OpeningKey, SealingKey};
use std::time::Duration;
use tokio::{net::TcpStream, select, sync::mpsc::Sender};
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

#[derive(Clone)]
pub struct EndPointClient {
    id: EndPointID,
    message_tx: Sender<Option<EndPointMessage>>,
    exit_tx: async_broadcast::Sender<()>,
}

impl EndPointClient {
    pub async fn new(
        local_device_id: i64,
        remote_device_id: i64,
        opening_key: OpeningKey<NonceValue>,
        sealing_key: SealingKey<NonceValue>,
        visit_credentials: String,
    ) -> CoreResult<Self> {
        let mut stream = connect("192.168.0.101:28001").await?;

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

        let client = EndPointClient {
            id: EndPointID(local_device_id, remote_device_id),
            message_tx,
            exit_tx: exit_tx.clone(),
        };

        // serve_video_decode(
        //     req.active_device_id,
        //     req.passive_device_id,
        //     req.texture_id,
        //     stream,
        // );

        // serve_audio_decode(req.active_device_id, req.passive_device_id);

        serve_reader(
            client.clone(),
            exit_tx.clone(),
            exit_rx.clone(),
            stream,
            opening_key,
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

    pub fn negotiate_finish(&self) -> CoreResult<()> {
        let req = EndPointMessage::NegotiateFinishedRequest(EndPointNegotiateFinishedRequest {
            expected_frame_rate: 60,
        });

        self.send_message(req)
    }

    pub fn input(&self, event: InputEvent) -> CoreResult<()> {
        let req = EndPointMessage::Input(EndPointInput { event });
        self.send_message(req)
    }

    pub fn send_audio_frame(
        &self,
        params: Option<(u32, AudioSampleFormat, u8, u16)>,
        buffer: &[u8],
    ) -> CoreResult<()> {
        let message = EndPointMessage::AudioFrame(EndPointAudioFrame {
            params,
            buffer: buffer.to_vec(),
        });
        self.send_message(message)
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
        self.message_tx
            .try_send(Some(message))
            .map_err(|_| core_error!("message send failed"))
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
                    handle_message(client.clone(), message);
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
                        Err(core_error!("read processor receive exit tx signal"))
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

fn handle_message(client: EndPointClient, message: EndPointMessage) {
    match message {
        EndPointMessage::Error => {
            // handle_error(active_device_id, passive_device_id);
        }
        EndPointMessage::NegotiateDesktopParamsRequest(req) => {
            tokio::task::spawn_blocking(move || {
                handle_negotiate_desktop_params_request(client, req)
            });
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
            handle_video_frame(client.id, video_frame);
        }
        EndPointMessage::AudioFrame(audio_frame) => {
            handle_audio_frame(client.id, audio_frame);
        }
        EndPointMessage::Input(input_event) => match input_event.event {
            InputEvent::Mouse(event) => {
                if let Some(entry) = PASSIVE_ENDPOINTS_MONITORS.get(&client.id) {
                    handle_mouse(event, entry.value());
                }
            }
            InputEvent::Keyboard(event) => handle_keyboard(event),
        },
    }
}
