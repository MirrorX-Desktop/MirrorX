use super::{
    handler::{handle_media_transmission, handle_start_media_transmission_request},
    message::{EndPointMessage, EndPointMessagePacketType, HandshakeRequest, HandshakeResponse},
};
use crate::media::desktop_duplicator::DesktopDuplicator;
use crate::{
    error::{anyhow::Result, MirrorXError},
    media::video_encoder::VideoEncoder,
    socket::endpoint::message::EndPointMessagePacket,
    utility::{nonce_value::NonceValue, serializer::BINCODE_SERIALIZER},
};
use bincode::Options;
use bytes::Bytes;
use dashmap::DashMap;
use futures::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use once_cell::sync::OnceCell;
use ring::aead::{OpeningKey, SealingKey};
use scopeguard::defer;
use std::{
    sync::atomic::{AtomicU16, Ordering},
    time::Duration,
};
use tokio::{
    net::{TcpStream, ToSocketAddrs},
    sync::mpsc::{Receiver, Sender},
    time::timeout,
};
use tokio_util::codec::{Framed, LengthDelimitedCodec};
use tracing::{error, info, trace};

const CALL_TIMEOUT: Duration = Duration::from_secs(5);

pub static ENDPOINTS: DashMap<String, EndPoint> = DashMap::new();

macro_rules! make_endpoint_call {
    ($name:tt, $req_type:ident, $req_message_type:path, $resp_type:ident, $resp_message_type:path) => {
        pub async fn $name(&self, req: $req_type) -> Result<$resp_type, MirrorXError> {
            let reply = self.call($req_message_type(req), CALL_TIMEOUT).await?;

            if let $resp_message_type(message) = reply {
                Ok(message)
            } else if let MirrorXError::Error(remote_error) = reply {
                Err(error_message)
            } else {
                Err(MirrorXError::EndPointCallResponseMismatched(
                    self.remote_device_id.clone(),
                ))
            }
        }
    };
}

macro_rules! handle_endpoint_call {
    ($remote_device_id:expr, $call_id:expr, $req:tt, $resp_type:path, $handler:tt) => {{
        tokio::spawn(async move {
            if let Some(call_id) = $call_id {
                if let Some(endpoint) = ENDPOINTS.get(&remote_device_id) {
                    let resp_message = match $handler(endpoint.value, $req).await {
                        Ok(resp) => $resp_type(resp),
                        Err(_) => EndPointMessage::Error(EndPointMessageError::Internal),
                    };

                    if let Err(err) = endpoint.reply(call_id,resp_message){
                        error!(err=?err,remote_device_id=?$remote_device_id,"handle_message: reply message failed");
                    }
                }else{
                    error!(remote_device_id=?$remote_device_id,"handle_message: endpoint not exists")
                }
            } else {
                error!("handle_message: EndPoint Request Message without call_id")
            }
        });
    }};
}

macro_rules! handle_endpoint_push {
    ($remote_device_id:expr, $req:tt, $handler:tt) => {{
        tokio::spawn(async move {
            if let Err(err) = $handler($remote_device_id, $req).await {
                error!(remote_device_id=?$remote_device_id,"handle_message: handle push message failed")
            }
        });
    }};
}

pub struct EndPoint {
    local_device_id: String,
    remote_device_id: String,
    atomic_call_id: AtomicU16,
    call_reply_tx_map: DashMap<u16, Sender<EndPointMessage>>,
    packet_tx: Sender<EndPointMessagePacket>,
    video_decoder_tx: OnceCell<Sender<Vec<u8>>>,
}

impl EndPoint {
    pub async fn connect<A>(
        addr: A,
        local_device_id: String,
        remote_device_id: String,
        opening_key: OpeningKey<NonceValue>,
        sealing_key: SealingKey<NonceValue>,
    ) -> Result<Self, MirrorXError>
    where
        A: ToSocketAddrs,
    {
        let stream = timeout(Duration::from_secs(10), TcpStream::connect(addr))
            .await
            .map_err(|err| MirrorXError::Timeout)?
            .map_err(|err| MirrorXError::IO(err))?;

        stream
            .set_nodelay(true)
            .map_err(|err| MirrorXError::IO(err))?;

        let framed_stream = LengthDelimitedCodec::builder()
            .little_endian()
            .max_frame_length(16 * 1024 * 1024)
            .new_framed(stream);

        let (sink, stream) = framed_stream.split();

        let (packet_tx, packet_rx) = tokio::sync::mpsc::channel(128);

        serve_stream(remote_device_id.clone(), stream, opening_key);
        serve_sink(packet_rx, sink, sealing_key);

        Ok(Self {
            local_device_id,
            remote_device_id,
            atomic_call_id: AtomicU16::new(0),
            call_reply_tx_map: DashMap::new(),
            packet_tx,
            video_decoder_tx: OnceCell::new(),
        })
    }

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

        let mut rx = self.register_call(call_id);
        defer! {
            self.remove_call(call_id);
        }

        timeout(duration, async move {
            if let Err(err) = self.send(packet).await {
                return Err(err);
            }

            rx.recv().await.ok_or(MirrorXError::Timeout)
        })
        .await
        .map_err(|err| MirrorXError::Timeout)?
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
            .map_err(|err| MirrorXError::Other(err))
    }

    fn set_call_reply(&self, call_id: u16, message: EndPointMessage) {
        self.remove_call(call_id).map(|tx| {
            if let Err(err) = tx.try_send(message) {
                error!(err = %err,remote_device_id=?self.remote_device_id,"set_call_reply: set reply failed")
            }
        });
    }

    fn register_call(&self, call_id: u16) -> Receiver<EndPointMessage> {
        let (tx, rx) = tokio::sync::mpsc::channel(1);
        self.call_reply_tx_map.insert(call_id, tx);
        rx
    }

    fn remove_call(&self, call_id: u16) -> Option<Sender<EndPointMessage>> {
        self.call_reply_tx_map.remove(&call_id).map(|entry| entry.1)
    }

    pub async fn begin_screen_capture() -> anyhow::Result<()> {
        let encoder_name: &str;

        if cfg!(target_os = "macos") {
            encoder_name = "h264_videotoolbox";
        } else if cfg!(target_os = "windows") {
            encoder_name = "libx264";
        } else {
            panic!("unsupported platform");
        }

        let mut encoder = VideoEncoder::new(encoder_name, 60, 1920, 1080)?;

        encoder.set_opt("profile", "high", 0)?;
        encoder.set_opt("level", "5.2", 0)?;

        if encoder_name == "libx264" {
            encoder.set_opt("preset", "ultrafast", 0)?;
            encoder.set_opt("tune", "zerolatency", 0)?;
            encoder.set_opt("sc_threshold", "499", 0)?;
        } else {
            encoder.set_opt("realtime", "1", 0)?;
            encoder.set_opt("allow_sw", "0", 0)?;
        }

        let packet_rx = encoder.open()?;
        let (mut desktop_duplicator, capture_frame_rx) = DesktopDuplicator::new(60)?;

        std::thread::spawn(move || {
            // make sure the media_transmission after start_media_transmission send
            std::thread::sleep(Duration::from_secs(1));

            if let Err(err) = desktop_duplicator.start() {
                error!(?err, "DesktopDuplicator start capture failed");
                return;
            }

            loop {
                let capture_frame = match capture_frame_rx.recv() {
                    Ok(frame) => frame,
                    Err(err) => {
                        tracing::error!(?err, "capture_frame_rx.recv");
                        break;
                    }
                };

                // encode will block current thread until capture_frame released (FFMpeg API 'avcodec_send_frame' finished)
                encoder.encode(capture_frame);
            }

            desktop_duplicator.stop();
        });

        std::thread::spawn(move || loop {
            match packet_rx.recv() {
                Ok(packet) => {
                    if let Err(err) =
                        runtime_provider.block_on(socket_provider.desktop_media_transmission(
                            endpoint.clone(),
                            MediaTransmission {
                                data: packet.data,
                                timestamp: 0,
                            },
                        ))
                    {
                        error!(?err, "desktop_media_transmission failed");
                    }
                }
                Err(err) => {
                    error!(err=?err, "packet_rx.recv");
                    break;
                }
            };
        });
    }

    make_endpoint_call!(
        handshake,
        HandshakeRequest,
        EndPointMessage::HandshakeRequest,
        HandshakeResponse,
        EndPointMessage::HandshakeResponse
    );
}

fn serve_stream(
    remote_device_id: String,
    mut stream: SplitStream<Framed<TcpStream, LengthDelimitedCodec>>,
    mut opening_key: OpeningKey<NonceValue>,
) {
    tokio::spawn(async move {
        loop {
            let mut packet_bytes = match stream.next().await {
                Some(res) => match res {
                    Ok(packet_bytes) => packet_bytes,
                    Err(err) => {
                        error!(err = ?err, "serve_stream: read failed");
                        break;
                    }
                },
                None => {
                    info!("serve_stream: stream closed, going to exit");
                    break;
                }
            };

            if let Err(err) = opening_key.open_in_place(ring::aead::Aad::empty(), &mut packet_bytes)
            {
                error!(err = ?err, "serve_stream: decrypt buffer failed");
                break;
            }

            let packet =
                match BINCODE_SERIALIZER.deserialize::<EndPointMessagePacket>(&packet_bytes) {
                    Ok(packet) => packet,
                    Err(err) => {
                        error!(err = ?err, "serve_stream: deserialize packet failed");
                        break;
                    }
                };

            tokio::spawn(async move {
                handle_message(remote_device_id.clone(), packet).await;
            });
        }

        info!("serve stream read loop exit");
    });
}

fn serve_sink(
    mut packet_rx: Receiver<EndPointMessagePacket>,
    mut sink: SplitSink<Framed<TcpStream, LengthDelimitedCodec>, Bytes>,
    mut sealing_key: SealingKey<NonceValue>,
) {
    tokio::spawn(async move {
        loop {
            let packet = match packet_rx.recv().await {
                Some(buffer) => buffer,
                None => {
                    info!("serve_sink: packet_rx all sender has dropped, going to exit");
                    break;
                }
            };

            trace!(packet = ?packet, "serve_sink: send");

            let mut buffer = match BINCODE_SERIALIZER.serialize(&packet) {
                Ok(buffer) => buffer,
                Err(err) => {
                    error!(err=?err,"serve_sink: packet serialize failed");
                    break;
                }
            };

            if matches!(packet.message, EndPointMessage::HandshakeRequest(_)) {
                if let Err(err) =
                    sealing_key.seal_in_place_append_tag(ring::aead::Aad::empty(), &mut buffer)
                {
                    error!(err = ?err, "serve_sink: crypt buffer failed");
                    break;
                }
            }

            if let Err(err) = sink.send(Bytes::from(buffer)).await {
                error!(err = ?err, "signaling_serve_sink: send failed, going to exit");
                break;
            }
        }

        info!("signaling_serve_sink: exit");
    });
}

async fn handle_message(remote_device_id: String, packet: EndPointMessagePacket) {
    match packet.typ {
        EndPointMessagePacketType::Request => match packet.message {
            EndPointMessage::StartMediaTransmissionRequest(req) => {
                handle_endpoint_call!(
                    remote_device_id,
                    packet.call_id,
                    req,
                    EndPointMessage::StartMediaTransmissionResponse,
                    handle_start_media_transmission_request
                )
            }
            _ => error!("handle_message: received unexpected EndPoint Request Message"),
        },
        EndPointMessagePacketType::Response => {
            if let Some(call_id) = packet.call_id {
                if let Some(endpoint) = ENDPOINTS.get(&remote_device_id) {
                    endpoint.set_call_reply(call_id, req);
                }
            } else {
                error!("handle_message: EndPoint Response Message without call_id")
            }
        }
        EndPointMessagePacketType::Push => match packet.message {
            EndPointMessage::MediaFrame(req) => {
                handle_endpoint_push!(remote_device_id, req, handle_media_transmission)
            }
            _ => error!("handle_message: received unexpected EndPoint Push Message"),
        },
    }
}
