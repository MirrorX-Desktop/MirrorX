use super::message::{
    ConnectRemoteRequest, ConnectRemoteResponse, ConnectionKeyExchangeRequest,
    ConnectionKeyExchangeResponse, HandshakeRequest, HandshakeResponse, HeartBeatRequest,
    HeartBeatResponse, SignalingMessage, SignalingMessageError, SignalingMessagePacket,
};
use crate::{
    error::{MirrorXError, MirrorXResult},
    utility::serializer::BINCODE_SERIALIZER,
};
use bincode::Options;
use bytes::Bytes;
use dashmap::DashMap;
use futures::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use std::{
    sync::atomic::{AtomicU8, Ordering},
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

macro_rules! make_signaling_call {
    ($name:tt,$req_type:ident,$req_message_type:path,$resp_type:ident,$resp_message_type:path) => {
        pub async fn $name(&self, req: $req_type) -> MirrorXResult<$resp_type> {
            let reply = self.call($req_message_type(req), CALL_TIMEOUT).await?;

            if let $resp_message_type(message) = reply {
                Ok(message)
            } else if let SignalingMessage::Error(error_message) = reply {
                Err(MirrorXError::Signaling(error_message))
            } else {
                Err(MirrorXError::Signaling(SignalingMessageError::Mismatched))
            }
        }
    };
}

pub struct SignalingClient {
    packet_tx: Sender<Vec<u8>>,
    atomic_call_id: AtomicU8,
    call_reply_tx_map: DashMap<u8, Sender<SignalingMessage>>,
}

impl SignalingClient {
    pub async fn connect<A>(addr: A) -> MirrorXResult<Self>
    where
        A: ToSocketAddrs,
    {
        let stream = timeout(Duration::from_secs(10), TcpStream::connect(addr))
            .await
            .map_err(|err| {
                error!("SignalingClient: connect timeout");
                MirrorXError::Timeout
            })?
            .map_err(|err| {
                error!(err=?err,"SignalingClient: connect error");
                MirrorXError::Raw(err.to_string())
            })?;

        stream.set_nodelay(true).map_err(|err| {
            error!(err=?err,"SignalingClient: set connection option nodelay error");
            MirrorXError::Raw(err.to_string())
        })?;

        let framed_stream = LengthDelimitedCodec::builder()
            .little_endian()
            .max_frame_length(8 * 1024)
            .new_framed(stream);

        let (sink, stream) = framed_stream.split();

        let (packet_tx, packet_rx) = tokio::sync::mpsc::channel(128);

        serve_stream(stream);
        serve_sink(packet_rx, sink);

        Ok(Self {
            packet_tx,
            atomic_call_id: AtomicU8::new(0),
            call_reply_tx_map: DashMap::new(),
        })
    }

    async fn call(
        &self,
        message: SignalingMessage,
        duration: Duration,
    ) -> anyhow::Result<SignalingMessage, MirrorXError> {
        let call_id = self.atomic_call_id.fetch_add(1, Ordering::SeqCst);

        let packet = SignalingMessagePacket {
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

    async fn send(&self, packet: SignalingMessagePacket) -> MirrorXResult<()> {
        let call_id = packet.call_id;

        let buffer = BINCODE_SERIALIZER.serialize(&packet).map_err(|err| {
            if let Some(call_id) = call_id {
                self.remove_call(call_id);
            }

            error!(err=?err,"SignalingClient: serialize error");
            MirrorXError::Raw(err.to_string())
        })?;

        self.packet_tx.send(buffer).await.map_err(|err| {
            error!(err=?err,"SignalingClient: send error");
            MirrorXError::Raw(err.to_string())
        })
    }

    fn set_call_reply(&self, call_id: u8, message: SignalingMessage) {
        self.remove_call(call_id).map(|tx| {
            if let Err(err) = tx.try_send(message) {
                tracing::error!(err = %err, "set_call_reply: set reply failed")
            }
        });
    }

    fn register_call(&self, call_id: u8) -> Receiver<SignalingMessage> {
        let (tx, rx) = tokio::sync::mpsc::channel(1);
        self.call_reply_tx_map.insert(call_id, tx);
        rx
    }

    fn remove_call(&self, call_id: u8) -> Option<Sender<SignalingMessage>> {
        self.call_reply_tx_map.remove(&call_id).map(|entry| entry.1)
    }

    make_signaling_call!(
        heartbeat,
        HeartBeatRequest,
        SignalingMessage::HeartBeatRequest,
        HeartBeatResponse,
        SignalingMessage::HeartBeatResponse
    );

    make_signaling_call!(
        handshake,
        HandshakeRequest,
        SignalingMessage::HandshakeRequest,
        HandshakeResponse,
        SignalingMessage::HandshakeResponse
    );

    make_signaling_call!(
        connect_remote,
        ConnectRemoteRequest,
        SignalingMessage::ConnectRemoteRequest,
        ConnectRemoteResponse,
        SignalingMessage::ConnectRemoteResponse
    );

    make_signaling_call!(
        connection_key_exchange,
        ConnectionKeyExchangeRequest,
        SignalingMessage::ConnectionKeyExchangeRequest,
        ConnectionKeyExchangeResponse,
        SignalingMessage::ConnectionKeyExchangeResponse
    );
}

fn serve_stream(stream: SplitStream<Framed<TcpStream, LengthDelimitedCodec>>) {
    tokio::spawn(async move {
        loop {
            let packet_bytes = match stream.next().await {
                Some(res) => match res {
                    Ok(packet_bytes) => packet_bytes,
                    Err(err) => {
                        tracing::error!(err = ?err, "signaling_serve_stream: read failed");
                        break;
                    }
                },
                None => {
                    tracing::info!("signaling_serve_stream: stream closed, going to exit");
                    break;
                }
            };

            let packet = match BINCODE_SERIALIZER
                .deserialize::<SignalingMessagePacket>(&packet_bytes)
            {
                Ok(packet) => packet,
                Err(err) => {
                    tracing::error!(err = ?err, "signaling_serve_stream: deserialize packet failed");
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
) {
    tokio::spawn(async move {
        loop {
            let buffer = match packet_rx.recv().await {
                Some(buffer) => buffer,
                None => {
                    tracing::info!(
                        "signaling_serve_sink: packet_rx all sender has dropped, going to exit"
                    );
                    break;
                }
            };

            tracing::trace!(buffer = ?format!("{:02X?}", buffer), "signaling_serve_sink: send");

            if let Err(err) = sink.send(Bytes::from(buffer)).await {
                tracing::error!(err = ?err, "signaling_serve_sink: send failed, going to exit");
                break;
            }
        }

        tracing::info!("signaling_serve_sink: exit");
    });
}

async fn handle_message(packet: SignalingMessagePacket) {
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
