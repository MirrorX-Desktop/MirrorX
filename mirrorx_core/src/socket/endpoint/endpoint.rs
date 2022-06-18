use super::message::EndPointMessage;
use crate::{
    error::{MirrorXError, MirrorXResult},
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
use tracing::error;

const CALL_TIMEOUT: Duration = Duration::from_secs(5);

pub struct EndPoint {
    active_device_id: String,
    passive_device_id: String,
    atomic_call_id: AtomicU16,
    call_reply_tx_map: DashMap<u16, Sender<EndPointMessage>>,
    packet_tx: Sender<Vec<u8>>,
    video_decoder_tx: OnceCell<Sender<Vec<u8>>>,
}

impl EndPoint {
    pub async fn connect<A>(
        addr: A,
        active_device_id: String,
        passive_device_id: String,
        opening_key: OpeningKey<NonceValue>,
        sealing_key: SealingKey<NonceValue>,
    ) -> MirrorXResult<Self>
    where
        A: ToSocketAddrs,
    {
        let stream = timeout(Duration::from_secs(10), TcpStream::connect(addr))
            .await
            .map_err(|err| {
                error!(passive_device_id=?passive_device_id,"EndPoint: connect timeout");
                MirrorXError::Timeout
            })?
            .map_err(|err| {
                error!(err=?err,passive_device_id=?passive_device_id,"EndPoint: connect error");
                MirrorXError::Raw(err.to_string())
            })?;

        stream.set_nodelay(true).map_err(|err| {
            error!(err=?err,passive_device_id=?passive_device_id,"EndPoint: set connection option nodelay error");
            MirrorXError::Raw(err.to_string())
        })?;

        let framed_stream = LengthDelimitedCodec::builder()
            .little_endian()
            .max_frame_length(16 * 1024 * 1024)
            .new_framed(stream);

        let (sink, stream) = framed_stream.split();

        let (packet_tx, packet_rx) = tokio::sync::mpsc::channel(128);

        serve_stream(stream, opening_key);
        serve_sink(packet_rx, sink, sealing_key);

        Ok(Self {
            active_device_id,
            passive_device_id,
            atomic_call_id: AtomicU16::new(0),
            call_reply_tx_map: DashMap::new(),
            packet_tx,
            video_decoder_tx: OnceCell::new(),
        })
    }

    // pub fn remote_device_id(&self) -> &str {
    //     self.passive_device_id.as_ref()
    // }

    // pub fn local_device_id(&self) -> &str {
    //     self.active_device_id.as_ref()
    // }

    // pub async fn handshake(&self, token: String) -> anyhow::Result<()> {
    //     let reply = self
    //         .call(
    //             LocalToEndPointMessage::HandshakeRequest(HandshakeRequest { token }),
    //             CALL_TIMEOUT,
    //         )
    //         .await?;

    //     if let SignalingToLocalMessage::HandshakeReply(message) = reply {
    //         match message.status {
    //             HandshakeStatus::Accepted => Ok(()),
    //             HandshakeStatus::Repeated => bail!("handshake: repeated"),
    //         }
    //     } else {
    //         bail!("handshake: mismatched reply message");
    //     }
    // }

    // pub async fn heartbeat(&self) -> anyhow::Result<()> {
    //     let reply = self
    //         .call(
    //             LocalToEndPointMessage::HeartBeatRequest(HeartBeatRequest {
    //                 time_stamp: chrono::Utc::now().timestamp() as u32,
    //             }),
    //             CALL_TIMEOUT,
    //         )
    //         .await?;

    //     if let SignalingToLocalMessage::HeartBeatReply(message) = reply {
    //         Ok(())
    //     } else {
    //         bail!("handshake: mismatched reply message");
    //     }
    // }

    // pub async fn start_media_transmission(
    //     &self,
    //     req: StartMediaTransmissionRequest,
    // ) -> anyhow::Result<StartMediaTransmissionReply> {
    //     self.call(
    //         EndPointMessage::StartMediaTransmissionRequest(req),
    //         CALL_TIMEOUT,
    //     )
    //     .await
    //     .and_then(|resp| match resp {
    //         EndPointMessage::Error => {
    //             bail!("desktop_start_media_transmission: remote error")
    //         }
    //         EndPointMessage::StartMediaTransmissionReply(message) => Ok(message),
    //         _ => bail!(
    //             "desktop_start_media_transmission: mismatched reply type, got {:?}",
    //             resp
    //         ),
    //     })
    // }

    // pub async fn send_media_frame(&self, media_transmission: MediaFrame) -> anyhow::Result<()> {
    //     self.send(EndPointMessagePacket::new(
    //         None,
    //         EndPointMessage::MediaFrame(media_transmission),
    //     ))
    //     .await
    // }

    // pub fn start_desktop_render_thread(
    //     &self,
    //     texture_id: i64,
    //     video_texture_ptr: i64,
    //     update_frame_callback_ptr: i64,
    // ) -> anyhow::Result<()> {
    //     unsafe {
    //         let update_frame_callback = std::mem::transmute::<
    //             *mut c_void,
    //             unsafe extern "C" fn(
    //                 texture_id: i64,
    //                 video_texture_ptr: *mut c_void,
    //                 new_frame_ptr: *mut c_void,
    //             ),
    //         >(update_frame_callback_ptr as *mut c_void);

    //         let mut decoder = crate::media::video_decoder::VideoDecoder::new("h264")?;

    //         let frame_rx = decoder.open()?;

    //         let (decoder_tx, decoder_rx) = crossbeam::channel::bounded::<Vec<u8>>(120);
    //         std::thread::spawn(move || loop {
    //             match decoder_rx.recv() {
    //                 Ok(data) => decoder.decode(data.as_ptr(), data.len() as i32, 0, 0),
    //                 Err(err) => {
    //                     error!("decoder_rx.recv: {}", err);
    //                     break;
    //                 }
    //             }
    //         });

    //         std::thread::spawn(move || loop {
    //             match frame_rx.recv() {
    //                 Ok(video_frame) => update_frame_callback(
    //                     texture_id,
    //                     video_texture_ptr as *mut c_void,
    //                     video_frame.0,
    //                 ),
    //                 Err(err) => {
    //                     error!(err= ?err,"desktop render thread error");
    //                     break;
    //                 }
    //             }
    //         });

    //         let _ = self.video_decoder_tx.set(decoder_tx);

    //         Ok(())
    //     }
    // }

    // pub fn transfer_desktop_video_frame(&self, frame: Vec<u8>) {
    //     if let Some(decoder) = self.video_decoder_tx.get() {
    //         if let Err(err) = decoder.try_send(frame) {
    //             match err {
    //                 TrySendError::Full(_) => return,
    //                 TrySendError::Disconnected(_) => return,
    //             }
    //         }
    //     }
    // }

    async fn call(
        &self,
        message: EndPointMessage,
        duration: Duration,
    ) -> MirrorXResult<EndPointMessage> {
        let call_id = self.atomic_call_id.fetch_add(1, Ordering::SeqCst);

        let packet = EndPointMessagePacket {
            call_id: Some(call_id),
            message,
        };

        let mut rx = self.register_call(call_id);

        timeout(duration, async move {
            if let Err(err) = self.send(packet).await {
                self.remove_call(call_id);
                return Err(err);
            };

            rx.recv().await.ok_or(MirrorXError::Timeout)
        })
        .await
        .map_err(|err| {
            self.remove_call(call_id);
            MirrorXError::Timeout
        })?
    }

    async fn reply(&self, call_id: u16, message: EndPointMessage) -> MirrorXResult<()> {
        let packet = EndPointMessagePacket {
            call_id: Some(call_id),
            message,
        };

        self.send(packet).await
    }

    async fn send(&self, packet: EndPointMessagePacket) -> MirrorXResult<()> {
        let call_id = packet.call_id;

        let buffer = BINCODE_SERIALIZER.serialize(&packet).map_err(|err| {
            if let Some(call_id) = call_id {
                self.remove_call(call_id);
            }

            error!(err=?err,passive_device_id=?self.passive_device_id,"EndPoint: serialize error");
            MirrorXError::Raw(err.to_string())
        })?;

        self.packet_tx.send(buffer).await.map_err(|err| {
            error!(err=?err,passive_device_id=?self.passive_device_id,"EndPoint: send error");
            MirrorXError::Raw(err.to_string())
        })
    }

    fn set_call_reply(&self, call_id: u16, message: EndPointMessage) {
        self.remove_call(call_id).map(|tx| {
            if let Err(err) = tx.try_send(message) {
                tracing::error!(err = %err,passive_device_id=?self.passive_device_id,"set_call_reply: set reply failed")
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
}

fn serve_stream(
    stream: SplitStream<Framed<TcpStream, LengthDelimitedCodec>>,
    opening_key: OpeningKey<NonceValue>,
) {
    tokio::spawn(async move {
        loop {
            let mut packet_bytes = match stream.next().await {
                Some(res) => match res {
                    Ok(packet_bytes) => packet_bytes,
                    Err(err) => {
                        tracing::error!(err = ?err, "serve_stream: read failed");
                        break;
                    }
                },
                None => {
                    tracing::info!("serve_stream: stream closed, going to exit");
                    break;
                }
            };

            if let Err(err) = opening_key.open_in_place(ring::aead::Aad::empty(), &mut packet_bytes)
            {
                tracing::error!(err = ?err, "serve_stream: decrypt buffer failed");
                break;
            }

            let packet =
                match BINCODE_SERIALIZER.deserialize::<EndPointMessagePacket>(&packet_bytes) {
                    Ok(packet) => packet,
                    Err(err) => {
                        tracing::error!(err = ?err, "serve_stream: deserialize packet failed");
                        break;
                    }
                };

            tokio::spawn(async move {
                handle_message(packet).await;
            });
        }

        tracing::info!("serve stream read loop exit");
    });
}

fn serve_sink(
    packet_rx: Receiver<Vec<u8>>,
    sink: SplitSink<Framed<TcpStream, LengthDelimitedCodec>, Bytes>,
    sealing_key: SealingKey<NonceValue>,
) {
    tokio::spawn(async move {
        loop {
            let mut buffer = match packet_rx.recv().await {
                Some(buffer) => buffer,
                None => {
                    tracing::info!("serve_sink: packet_rx all sender has dropped, going to exit");
                    break;
                }
            };

            tracing::trace!(buffer = ?format!("{:02X?}", buffer), "serve_sink: send");

            if let Err(err) =
                sealing_key.seal_in_place_append_tag(ring::aead::Aad::empty(), &mut buffer)
            {
                tracing::error!(err = ?err, "serve_sink: crypt buffer failed");
                break;
            }

            if let Err(err) = sink.send(Bytes::from(buffer)).await {
                tracing::error!(err = ?err, "signaling_serve_sink: send failed, going to exit");
                break;
            }
        }

        tracing::info!("signaling_serve_sink: exit");
    });
}

async fn handle_message(packet: EndPointMessagePacket) {
    // if packet.call_id.is_none() {
    //     match packet.message {
    //         SignalingToLocalMessage::Error(ErrorReason::RemoteEndpointOffline(
    //             remote_device_id,
    //         )) => {
    //             tracing::warn!(
    //                 remote_device_id = ?remote_device_id,
    //                 "remote endpoint offline, local endpoint exit"
    //             );

    //             if let Some(endpoint) = EndPointProvider::current()?.remove(&remote_device_id) {}

    //             return Ok(());
    //         }
    //         _ => {}
    //     }
    // }

    // SocketProvider::current()?.set_server_call_reply(call_id, message);
}
