use super::{
    handlers::negotiate_desktop_params::handle_negotiate_desktop_params_request, id::EndPointID,
    message::*, EndPointStream,
};
use crate::{
    api::endpoint::handlers::{
        input::handle_input, negotiate_finished::handle_negotiate_finished_request,
    },
    component::desktop::monitor::Monitor,
    core_error,
    error::{CoreError, CoreResult},
    utility::nonce_value::NonceValue,
};
use bincode::{
    config::{LittleEndian, VarintEncoding, WithOtherEndian, WithOtherIntEncoding},
    DefaultOptions, Options,
};
use bytes::Bytes;
use futures::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use once_cell::sync::Lazy;
use ring::aead::{OpeningKey, SealingKey};
use std::{fmt::Display, net::SocketAddr, ops::Deref, sync::Arc, time::Duration};
use tokio::{
    net::TcpStream,
    sync::{mpsc::Sender, RwLock},
};
use tokio_util::{
    codec::{Framed, LengthDelimitedCodec},
    udp::UdpFramed,
};

const RECV_MESSAGE_TIMEOUT: Duration = Duration::from_secs(30);

static BINARY_SERIALIZER: Lazy<
    WithOtherIntEncoding<WithOtherEndian<DefaultOptions, LittleEndian>, VarintEncoding>,
> = Lazy::new(|| {
    bincode::DefaultOptions::new()
        .with_little_endian()
        .with_varint_encoding()
});

#[derive(Debug, Clone)]
pub struct EndPointClient {
    endpoint_id: EndPointID,
    monitor: Arc<RwLock<Option<Arc<Monitor>>>>,
    exit_tx: async_broadcast::Sender<()>,
    tx: tokio::sync::mpsc::Sender<Vec<u8>>,
}

impl EndPointClient {
    pub async fn new_active(
        endpoint_id: EndPointID,
        stream_key: Option<(OpeningKey<NonceValue>, SealingKey<NonceValue>)>,
        stream: EndPointStream,
        video_frame_tx: tokio::sync::mpsc::Sender<EndPointVideoFrame>,
        audio_frame_tx: tokio::sync::mpsc::Sender<EndPointAudioFrame>,
    ) -> CoreResult<Arc<EndPointClient>> {
        EndPointClient::create(
            true,
            endpoint_id,
            stream_key,
            stream,
            Some(video_frame_tx),
            Some(audio_frame_tx),
        )
        .await
    }

    pub async fn new_passive(
        endpoint_id: EndPointID,
        key_pair: Option<(OpeningKey<NonceValue>, SealingKey<NonceValue>)>,
        stream: EndPointStream,
    ) -> CoreResult<()> {
        let _ = EndPointClient::create(false, endpoint_id, key_pair, stream, None, None).await?;
        Ok(())
    }

    async fn create(
        active: bool,
        endpoint_id: EndPointID,
        key_pair: Option<(OpeningKey<NonceValue>, SealingKey<NonceValue>)>,
        stream: EndPointStream,
        video_frame_tx: Option<tokio::sync::mpsc::Sender<EndPointVideoFrame>>,
        audio_frame_tx: Option<tokio::sync::mpsc::Sender<EndPointAudioFrame>>,
    ) -> CoreResult<Arc<EndPointClient>> {
        let (exit_tx, exit_rx) = async_broadcast::broadcast(16);
        let (opening_key, sealing_key) = match key_pair {
            Some((opening_key, sealing_key)) => (Some(opening_key), Some(sealing_key)),
            None => (None, None),
        };

        let (tx, mut rx) = match stream {
            EndPointStream::PublicTCP(addr) => {
                let stream = tokio::time::timeout(
                    Duration::from_secs(10),
                    tokio::net::TcpStream::connect(addr),
                )
                .await??;
                serve_tcp(stream, endpoint_id, sealing_key, opening_key)?
            }
            EndPointStream::PublicUDP(_) => panic!("not support yet"),
            EndPointStream::PrivateTCP(stream) => {
                serve_tcp(stream, endpoint_id, sealing_key, opening_key)?
            }
            EndPointStream::PrivateUDP {
                remote_addr,
                socket,
            } => {
                let (tx, rx) = tokio::sync::mpsc::channel(1);
                let framed = UdpFramed::new(
                    socket,
                    LengthDelimitedCodec::builder()
                        .big_endian()
                        .length_field_length(4)
                        .new_codec(),
                );
                let (sink, stream) = framed.split();
                serve_udp_write(remote_addr, rx, sealing_key, sink);
                let rx = serve_udp_read(remote_addr, opening_key, stream)?;
                (tx, rx)
            }
        };

        // active endpoint should start negotiate with passive endpoint
        let primary_monitor = if active {
            let params = serve_active_negotiate(&tx, &mut rx).await?;
            Some(Arc::new(params.primary_monitor))
        } else {
            None
        };

        let client = Arc::new(EndPointClient {
            endpoint_id,
            monitor: Arc::new(RwLock::new(primary_monitor)),
            exit_tx,
            tx,
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
    pub fn blocking_send(&self, message: &EndPointMessage) -> CoreResult<()> {
        let buffer = BINARY_SERIALIZER.serialize(message)?;
        self.tx
            .blocking_send(buffer)
            .map_err(|_| CoreError::OutgoingMessageChannelDisconnect)
    }

    pub async fn send(&self, message: &EndPointMessage) -> CoreResult<()> {
        let buffer = BINARY_SERIALIZER.serialize(message)?;
        self.tx
            .send(buffer)
            .await
            .map_err(|_| CoreError::OutgoingMessageChannelDisconnect)
    }

    pub fn close(&self) {
        let _ = self.exit_tx.try_broadcast(());
    }

    pub fn close_receiver(&self) -> async_broadcast::Receiver<()> {
        self.exit_tx.new_receiver()
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
    let negotiate_request_buffer = BINARY_SERIALIZER.serialize(
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
        BINARY_SERIALIZER.deserialize(negotiate_response_buffer.deref())? else {
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

    let negotiate_request_buffer = BINARY_SERIALIZER.serialize(
        &EndPointMessage::NegotiateFinishedRequest(EndPointNegotiateFinishedRequest {
            expected_frame_rate: 60,
        }),
    )?;

    tx.send(negotiate_request_buffer)
        .await
        .map_err(|_| CoreError::OutgoingMessageChannelDisconnect)?;

    Ok(params)
}

fn serve_tcp(
    stream: TcpStream,
    endpoint_id: EndPointID,
    sealing_key: Option<SealingKey<NonceValue>>,
    opening_key: Option<OpeningKey<NonceValue>>,
) -> Result<(Sender<Vec<u8>>, tokio::sync::mpsc::Receiver<Bytes>), CoreError> {
    let (tx, rx) = tokio::sync::mpsc::channel(1);
    let framed = Framed::new(
        stream,
        LengthDelimitedCodec::builder()
            .big_endian()
            .length_field_length(4)
            .new_codec(),
    );
    let (sink, stream) = framed.split();
    serve_tcp_write(endpoint_id, rx, sealing_key, sink);
    let rx = serve_tcp_read(endpoint_id, opening_key, stream)?;
    Ok((tx, rx))
}

fn serve_tcp_read(
    endpoint_id: EndPointID,
    mut opening_key: Option<OpeningKey<NonceValue>>,
    mut stream: SplitStream<Framed<TcpStream, LengthDelimitedCodec>>,
) -> CoreResult<tokio::sync::mpsc::Receiver<Bytes>> {
    let (tx, rx) = tokio::sync::mpsc::channel(1);

    tokio::spawn(async move {
        loop {
            let mut buffer = match stream.next().await {
                Some(packet) => match packet {
                    Ok(v) => v,
                    Err(err) => {
                        tracing::error!(?endpoint_id, ?err, "read stream failed");
                        break;
                    }
                },
                None => {
                    tracing::error!(?endpoint_id, "read stream is closed");
                    break;
                }
            };

            if let Some(ref mut opening_key) = opening_key {
                if let Err(err) =
                    opening_key.open_in_place(ring::aead::Aad::empty(), buffer.as_mut())
                {
                    tracing::error!(?err, "open endpoint message packet failed");
                    break;
                }
            }

            if tx.send(buffer.freeze()).await.is_err() {
                tracing::error!(?endpoint_id, "output channel closed");
                break;
            }
        }

        tracing::info!(?endpoint_id, "tcp read loop exit");
    });

    Ok(rx)
}

fn serve_tcp_write(
    endpoint_id: EndPointID,
    mut rx: tokio::sync::mpsc::Receiver<Vec<u8>>,
    mut sealing_key: Option<SealingKey<NonceValue>>,
    mut sink: SplitSink<Framed<TcpStream, LengthDelimitedCodec>, Bytes>,
) {
    tokio::spawn(async move {
        loop {
            match rx.recv().await {
                Some(mut buffer) => {
                    if let Some(ref mut sealing_key) = sealing_key {
                        if let Err(err) = sealing_key
                            .seal_in_place_append_tag(ring::aead::Aad::empty(), &mut buffer)
                        {
                            tracing::error!(?err, "seal endpoint message packet failed");
                            break;
                        }
                    }

                    if sink.send(Bytes::from(buffer)).await.is_err() {
                        tracing::error!(?endpoint_id, "tcp write failed");
                        break;
                    }
                }
                None => {
                    tracing::error!(?endpoint_id, "input channel closed");
                    break;
                }
            }
        }

        tracing::info!(?endpoint_id, "tcp write loop exit");
    });
}

fn serve_udp_read(
    remote_addr: SocketAddr,
    mut opening_key: Option<OpeningKey<NonceValue>>,
    mut stream: SplitStream<UdpFramed<LengthDelimitedCodec>>,
) -> CoreResult<tokio::sync::mpsc::Receiver<Bytes>> {
    let (tx, rx) = tokio::sync::mpsc::channel(1);

    tokio::spawn(async move {
        loop {
            let mut buffer = match stream.next().await {
                Some(packet) => match packet {
                    Ok((buffer, addr)) => {
                        if addr != remote_addr {
                            continue;
                        }

                        buffer
                    }
                    Err(err) => {
                        tracing::error!(?remote_addr, ?err, "read stream failed");
                        break;
                    }
                },
                None => {
                    tracing::error!(?remote_addr, "read stream is closed");
                    break;
                }
            };

            if let Some(ref mut opening_key) = opening_key {
                if let Err(err) =
                    opening_key.open_in_place(ring::aead::Aad::empty(), buffer.as_mut())
                {
                    tracing::error!(?err, "open endpoint message packet failed");
                    break;
                }
            }

            if tx.send(Bytes::from(buffer)).await.is_err() {
                tracing::error!(?remote_addr, "output channel closed");
                break;
            }
        }

        tracing::info!(?remote_addr, "tcp read loop exit");
    });

    Ok(rx)
}

fn serve_udp_write(
    remote_addr: SocketAddr,
    mut rx: tokio::sync::mpsc::Receiver<Vec<u8>>,
    mut sealing_key: Option<SealingKey<NonceValue>>,
    mut sink: SplitSink<UdpFramed<LengthDelimitedCodec>, (Bytes, SocketAddr)>,
) {
    tokio::spawn(async move {
        loop {
            match rx.recv().await {
                Some(mut buffer) => {
                    if let Some(ref mut sealing_key) = sealing_key {
                        if let Err(err) = sealing_key
                            .seal_in_place_append_tag(ring::aead::Aad::empty(), &mut buffer)
                        {
                            tracing::error!(?err, "seal endpoint message packet failed");
                            break;
                        }
                    }

                    if sink.send((Bytes::from(buffer), remote_addr)).await.is_err() {
                        tracing::error!(?remote_addr, "tcp write failed");
                        break;
                    }
                }
                None => {
                    tracing::error!(?remote_addr, "input channel closed");
                    break;
                }
            }
        }

        tracing::info!(?remote_addr, "tcp write loop exit");
    });
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

            let message = match BINARY_SERIALIZER.deserialize(&buffer) {
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
            }
        }

        tracing::info!("message handle loop exit");
    });
}
