use super::{
    handler::{handle_connect_request, handle_connection_key_exchange_request},
    message::{
        ConnectRequest, ConnectResponse, ConnectionKeyExchangeRequest,
        ConnectionKeyExchangeResponse, HandshakeRequest, HandshakeResponse, HeartBeatRequest,
        HeartBeatResponse, SignalingMessage, SignalingMessageError, SignalingMessagePacket,
        SignalingMessagePacketType,
    },
};
use crate::{
    error::MirrorXError,
    utility::{runtime::TOKIO_RUNTIME, serializer::BINCODE_SERIALIZER},
};
use anyhow::anyhow;
use arc_swap::ArcSwapOption;
use bincode::Options;
use bytes::Bytes;
use dashmap::DashMap;
use futures::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use once_cell::sync::OnceCell;
use scopeguard::defer;
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
use tracing::{error, info};

const CALL_TIMEOUT: Duration = Duration::from_secs(60);

pub static CURRENT_SIGNALING_CLIENT: ArcSwapOption<SignalingClient> = ArcSwapOption::const_empty();

macro_rules! make_signaling_call {
    ($name:tt, $req_type:ident, $req_message_type:path, $resp_type:ident, $resp_message_type:path) => {
        pub async fn $name(
            &self,
            remote_device_id: Option<String>,
            req: $req_type,
        ) -> Result<$resp_type, MirrorXError> {
            match self
                .call(
                    remote_device_id.clone(),
                    $req_message_type(req),
                    CALL_TIMEOUT,
                )
                .await
            {
                Ok(reply) => {
                    if let $resp_message_type(message) = reply {
                        Ok(message)
                    } else {
                        Err(MirrorXError::SignalingError(reply))
                    }
                }
                Err(err) => {
                    let fn_name = stringify!($name);
                    error!(remote_device_id=?remote_device_id,call_fn=fn_name,err=?err, "signaling call failed");
                    return Err(err);
                }
            }
        }
    };
}

macro_rules! handle_signaling_call {
    ($call_id:expr, $req:tt, $resp_type:path, $handler:tt) => {{
        TOKIO_RUNTIME.spawn(async move {
            let resp_message = match $handler($req).await {
                Ok(resp) => $resp_type(resp),
                Err(err) => {
                    let fn_name = stringify!($handler);
                    error!(handler=fn_name,err=?err, "handle signaling call failed");
                    SignalingMessage::Error(SignalingMessageError::Internal)
                }
            };

            match CURRENT_SIGNALING_CLIENT.load().as_ref() {
                Some(signaling_client) => {
                    if let Err(err) =
                        signaling_client.reply($call_id, resp_message).await
                    {
                        error!(err=?err,"handle_message: reply message failed");
                    }
                }
                None => error!("handle_message: current signaling client not exists"),
            }
        });
    }};
}

pub struct SignalingClient {
    device_id: OnceCell<String>,
    packet_tx: Sender<Vec<u8>>,
    atomic_call_id: AtomicU8,
    call_reply_tx_map: DashMap<u8, Sender<SignalingMessage>>,
}

impl SignalingClient {
    pub async fn connect<A>(addr: A) -> Result<Self, MirrorXError>
    where
        A: ToSocketAddrs,
    {
        let stream = timeout(Duration::from_secs(10), TcpStream::connect(addr))
            .await
            .map_err(|_| MirrorXError::Timeout)?
            .map_err(|err| MirrorXError::IO(err))?;

        stream
            .set_nodelay(true)
            .map_err(|err| MirrorXError::IO(err))?;

        let framed_stream = LengthDelimitedCodec::builder()
            .little_endian()
            .max_frame_length(8 * 1024)
            .new_framed(stream);

        let (sink, stream) = framed_stream.split();

        let (packet_tx, packet_rx) = tokio::sync::mpsc::channel(128);
        // todo: servce_sink should exit

        serve_reader(stream);
        serve_writer(packet_rx, sink);

        Ok(Self {
            device_id: OnceCell::new(),
            packet_tx,
            atomic_call_id: AtomicU8::new(0),
            call_reply_tx_map: DashMap::new(),
        })
    }

    pub fn set_device_id(&self, device_id: String) {
        let _ = self.device_id.set(device_id);
    }

    async fn call(
        &self,
        remote_device_id: Option<String>,
        message: SignalingMessage,
        duration: Duration,
    ) -> Result<SignalingMessage, MirrorXError> {
        let direction = if let Some(remote_device_id) = remote_device_id {
            match self.device_id.get() {
                Some(local_device_id) => Some((local_device_id.clone(), remote_device_id)),
                None => return Err(MirrorXError::ComponentUninitialized),
            }
        } else {
            None
        };

        let call_id = self.atomic_call_id.fetch_add(1, Ordering::SeqCst);

        let packet = SignalingMessagePacket {
            direction,
            typ: SignalingMessagePacketType::Request,
            call_id,
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
        .map_err(|_| MirrorXError::Timeout)?
    }

    async fn reply(&self, call_id: u8, message: SignalingMessage) -> Result<(), MirrorXError> {
        let packet = SignalingMessagePacket {
            direction: None,
            typ: SignalingMessagePacketType::Response,
            call_id,
            message,
        };

        self.send(packet).await
    }

    async fn send(&self, packet: SignalingMessagePacket) -> Result<(), MirrorXError> {
        let buffer = BINCODE_SERIALIZER
            .serialize(&packet)
            .map_err(|err| MirrorXError::SerializeFailed(err))?;

        self.packet_tx
            .try_send(buffer)
            .map_err(|err| MirrorXError::Other(anyhow!(err)))
    }

    fn set_call_reply(&self, call_id: u8, message: SignalingMessage) {
        self.remove_call(call_id).map(|tx| {
            if let Err(err) = tx.try_send(message) {
                error!(err = %err, "set_call_reply: set reply failed")
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
        ConnectRequest,
        SignalingMessage::ConnectRequest,
        ConnectResponse,
        SignalingMessage::ConnectResponse
    );

    make_signaling_call!(
        connection_key_exchange,
        ConnectionKeyExchangeRequest,
        SignalingMessage::ConnectionKeyExchangeRequest,
        ConnectionKeyExchangeResponse,
        SignalingMessage::ConnectionKeyExchangeResponse
    );
}

fn serve_reader(mut stream: SplitStream<Framed<TcpStream, LengthDelimitedCodec>>) {
    TOKIO_RUNTIME.spawn(async move {
        loop {
            let packet_bytes = match stream.next().await {
                Some(res) => match res {
                    Ok(packet_bytes) => packet_bytes,
                    Err(err) => {
                        error!(err = ?err, "reader: read packet failed");
                        break;
                    }
                },
                None => {
                    info!("reader: remote closed");
                    break;
                }
            };

            let packet =
                match BINCODE_SERIALIZER.deserialize::<SignalingMessagePacket>(&packet_bytes) {
                    Ok(packet) => packet,
                    Err(err) => {
                        error!(err = ?err, "reader: deserialize packet failed");
                        break;
                    }
                };

            TOKIO_RUNTIME.spawn(async move {
                handle_message(packet).await;
            });
        }

        info!("reader: exit");
    });
}

fn serve_writer(
    mut packet_buffer_rx: Receiver<Vec<u8>>,
    mut sink: SplitSink<Framed<TcpStream, LengthDelimitedCodec>, Bytes>,
) {
    TOKIO_RUNTIME.spawn(async move {
        loop {
            let packet_buffer = match packet_buffer_rx.recv().await {
                Some(buffer) => buffer,
                None => {
                    info!("writer: tx closed");
                    break;
                }
            };

            if let Err(err) = sink.send(Bytes::from(packet_buffer)).await {
                error!(err = ?err, "writer: send failed");
                break;
            }
        }

        info!("writer: exit");
    });
}

async fn handle_message(packet: SignalingMessagePacket) {
    match packet.typ {
        SignalingMessagePacketType::Request => match packet.message {
            SignalingMessage::ConnectRequest(req) => {
                handle_signaling_call!(
                    packet.call_id,
                    req,
                    SignalingMessage::ConnectResponse,
                    handle_connect_request
                )
            }
            SignalingMessage::ConnectionKeyExchangeRequest(req) => {
                handle_signaling_call!(
                    packet.call_id,
                    req,
                    SignalingMessage::ConnectionKeyExchangeResponse,
                    handle_connection_key_exchange_request
                )
            }
            _ => error!("handle_message: received unexpected Signaling Request Message"),
        },
        SignalingMessagePacketType::Response => match CURRENT_SIGNALING_CLIENT.load().as_ref() {
            Some(signaling_client) => {
                signaling_client.set_call_reply(packet.call_id, packet.message)
            }
            None => error!("handle_message: current signaling client not exists"),
        },
    }
}
