use super::runtime::RuntimeProvider;
use crate::{
    provider::endpoint::EndPointProvider,
    socket::{
        client_to_client_handler,
        endpoint::EndPoint,
        message::{
            client_to_client::ClientToClientMessage,
            client_to_server::{ClientToServerMessage, HandshakeRequest, HeartBeatRequest},
            server_to_client::{HandshakeStatus, ServerToClientMessage},
        },
        packet::Packet,
    },
    utility::{call_id_generator::CallIdGenerator, serializer::BINCODE_SERIALIZER},
};
use anyhow::bail;
use bincode::Options;
use bytes::Bytes;
use dashmap::DashMap;
use futures::{SinkExt, StreamExt};
use log::{error, info, trace, warn};
use once_cell::sync::OnceCell;
use std::{sync::Arc, time::Duration};
use tokio::{
    net::{TcpStream, ToSocketAddrs},
    sync::mpsc,
    time::timeout,
};
use tokio_util::codec::LengthDelimitedCodec;

static CURRENT_SOCKET_PROVIDER: OnceCell<Arc<SocketProvider>> = OnceCell::new();

macro_rules! handle_client_to_client_message {
    ($endpoint:expr, $call_id:ident, $handler:path, $req:ident, $reply_type:path) => {
        RuntimeProvider::current()?.spawn(async move {
            let res = $handler($endpoint.clone(), $req)
                .await
                .map(|reply| $reply_type(reply));

            let message = match res {
                Ok(message) => message,
                Err(err) => {
                    error!(
                        "handle_client_to_client_message: handle message error: {:?}",
                        err
                    );
                    ClientToClientMessage::Error
                }
            };

            match SocketProvider::current() {
                Ok(provider) => {
                    if let Err(err) = provider
                        .reply(
                            $call_id,
                            $endpoint.local_device_id().to_string(),
                            $endpoint.remote_device_id().to_string(),
                            message,
                        )
                        .await
                    {
                        error!("handle_client_to_client_message: reply error: {:?}", err);
                    }
                }
                Err(_) => {
                    error!("handle_client_to_client_message: socket provider uninitialized");
                }
            }
        });
    };
}

pub struct SocketProvider {
    tx: mpsc::Sender<Vec<u8>>,
    call_server_tx_map: DashMap<u16, mpsc::Sender<ServerToClientMessage>>,
    call_client_tx_map: DashMap<u16, mpsc::Sender<ClientToClientMessage>>,
    call_id_generator: CallIdGenerator,
}

impl SocketProvider {
    pub fn current() -> anyhow::Result<Arc<SocketProvider>> {
        CURRENT_SOCKET_PROVIDER
            .get()
            .map(|v| v.clone())
            .ok_or_else(|| anyhow::anyhow!("SocketProvider: uninitialized"))
    }

    pub fn make_current<A>(addr: A, token: &str) -> anyhow::Result<()>
    where
        A: ToSocketAddrs,
    {
        match CURRENT_SOCKET_PROVIDER.get_or_try_init(|| -> anyhow::Result<Arc<SocketProvider>> {
            RuntimeProvider::current()?.block_on(async move {
                let stream = timeout(Duration::from_secs(10), TcpStream::connect(addr)).await??;

                let (tx, rx) = mpsc::channel(16);

                let client = Arc::new(SocketProvider {
                    tx,
                    call_server_tx_map: DashMap::new(),
                    call_client_tx_map: DashMap::new(),
                    call_id_generator: CallIdGenerator::new(),
                });

                serve_stream(stream, rx)?;
                Ok(client)
            })
        }) {
            Ok(provider) => {
                handshake(provider.clone(), token)?;
                serve_heart_beat(provider.clone())?;
                Ok(())
            }
            Err(err) => bail!("SocketProvider: make current failed: {}", err),
        }
    }

    pub async fn call_client(
        &self,
        from_device_id: String,
        to_device_id: String,
        message: ClientToClientMessage,
        timeout: Duration,
    ) -> anyhow::Result<ClientToClientMessage> {
        let call_id = self.call_id_generator.next();

        let buf = BINCODE_SERIALIZER.serialize(&message)?;

        let packet = Packet::ClientToClient(call_id, from_device_id, to_device_id, false, buf);

        let mut rx = self.register_client_call(call_id);

        if let Err(err) = self.send(packet).await {
            self.remove_client_call(call_id);
            bail!("call_client: send packet failed: {}", err);
        }

        match tokio::time::timeout(timeout, rx.recv()).await {
            Ok(res) => match res {
                Some(message) => Ok(message),
                None => bail!("call_client: socket closed"),
            },
            Err(_) => {
                self.remove_client_call(call_id);
                bail!("call_client: timeout");
            }
        }
    }

    pub async fn call_server(
        &self,
        message: ClientToServerMessage,
        timeout: Duration,
    ) -> anyhow::Result<ServerToClientMessage> {
        let call_id = self.call_id_generator.next();

        let packet = Packet::ClientToServer(call_id, message);

        let mut rx = self.register_server_call(call_id);

        tokio::time::timeout(timeout, async move {
            if let Err(err) = self.send(packet).await {
                self.remove_server_call(call_id);
                bail!("call_server: send packet failed: {}", err);
            };

            rx.recv()
                .await
                .ok_or(anyhow::anyhow!("call_server: call tx closed"))
        })
        .await
        .map_err(|err| {
            self.remove_server_call(call_id);
            anyhow::anyhow!("call_server: {}", err)
        })?
    }

    pub async fn reply(
        &self,
        call_id: u16,
        from_device_id: String,
        to_device_id: String,
        message: ClientToClientMessage,
    ) -> anyhow::Result<()> {
        let buf = BINCODE_SERIALIZER.serialize(&message)?;
        let packet = Packet::ClientToClient(call_id, from_device_id, to_device_id, false, buf);
        self.send(packet).await
    }

    pub async fn send(&self, packet: Packet) -> anyhow::Result<()> {
        let buf = BINCODE_SERIALIZER.serialize(&packet)?;
        self.tx.send(buf).await?;
        Ok(())
    }

    fn set_client_call_reply(&self, call_id: u16, message: ClientToClientMessage) {
        self.remove_client_call(call_id).map(|tx| {
            if let Err(err) = tx.try_send(message) {
                error!("set_client_call_reply: reply failed: {}", err)
            }
        });
    }

    fn set_server_call_reply(&self, call_id: u16, message: ServerToClientMessage) {
        self.remove_server_call(call_id).map(|tx| {
            if let Err(err) = tx.try_send(message) {
                error!("set_server_call_reply: reply failed: {}", err)
            }
        });
    }

    fn register_server_call(&self, call_id: u16) -> mpsc::Receiver<ServerToClientMessage> {
        let (tx, rx) = mpsc::channel(1);
        self.call_server_tx_map.insert(call_id, tx);
        rx
    }

    fn remove_server_call(&self, call_id: u16) -> Option<mpsc::Sender<ServerToClientMessage>> {
        self.call_server_tx_map
            .remove(&call_id)
            .map(|entry| entry.1)
    }

    fn register_client_call(&self, call_id: u16) -> mpsc::Receiver<ClientToClientMessage> {
        let (tx, rx) = mpsc::channel(1);
        self.call_client_tx_map.insert(call_id, tx);
        rx
    }

    fn remove_client_call(&self, call_id: u16) -> Option<mpsc::Sender<ClientToClientMessage>> {
        self.call_client_tx_map
            .remove(&call_id)
            .map(|entry| entry.1)
    }
}

fn serve_stream(stream: TcpStream, mut rx: mpsc::Receiver<Vec<u8>>) -> anyhow::Result<()> {
    let framed_stream = LengthDelimitedCodec::builder()
        .little_endian()
        .max_frame_length(16 * 1024 * 1024)
        .new_framed(stream);

    let (mut sink, mut stream) = framed_stream.split();

    RuntimeProvider::current()?.spawn(async move {
        loop {
            let packet_bytes = match stream.next().await {
                Some(res) => match res {
                    Ok(packet_bytes) => packet_bytes,
                    Err(err) => match err.kind() {
                        std::io::ErrorKind::UnexpectedEof => {
                            info!("serve_stream: disconnected, exit");
                            break;
                        }
                        std::io::ErrorKind::ConnectionReset => {
                            info!("serve_stream: connection reset, exit");
                            break;
                        }
                        _ => {
                            error!("serve_stream: read packet error: {:?}", err);
                            continue;
                        }
                    },
                },
                None => break,
            };

            let packet = match BINCODE_SERIALIZER.deserialize::<Packet>(&packet_bytes) {
                Ok(packet) => packet,
                Err(err) => {
                    error!("serve_stream: deserialize packet error: {:?}", err);
                    continue;
                }
            };

            match packet {
                Packet::ClientToServer(_, message) => {
                    warn!(
                        "serve_stream: received unexpected client to server packet: {}",
                        message
                    );
                }
                Packet::ServerToClient(call_id, message) => {
                    if call_id == 0 {
                        // todo
                    }

                    if let Ok(socket_provider) = SocketProvider::current() {
                        socket_provider.set_server_call_reply(call_id, message);
                    } else {
                        error!("serve_stream: socket provider uninitialized");
                    }
                }
                Packet::ClientToClient(
                    call_id,
                    from_device_id,
                    to_device_id,
                    is_secure,
                    mut message_bytes,
                ) => {
                    let endpoint = match select_endpoint(to_device_id, from_device_id) {
                        Ok(ep) => ep,
                        Err(err) => {
                            error!("{}", err);
                            continue;
                        }
                    };

                    if is_secure {
                        if let Err(err) = endpoint.secure_open(&mut message_bytes).await {
                            error!("{}", err);
                            continue;
                        }
                    }

                    let message = match BINCODE_SERIALIZER
                        .deserialize::<ClientToClientMessage>(&message_bytes)
                    {
                        Ok(message) => message,
                        Err(err) => {
                            error!(
                                "serve_stream: deserialize client to client message failed: {}",
                                err
                            );
                            continue;
                        }
                    };

                    if let Err(err) = handle_client_to_client_message(endpoint, call_id, message) {
                        error!("{}", err);
                    }
                }
            };
        }

        info!("serve_stream: stream_loop exit");
    });

    RuntimeProvider::current()?.spawn(async move {
        loop {
            let buf = match rx.recv().await {
                Some(buf) => buf,
                None => break,
            };

            trace!("serve_stream: send packet: {:02X?}", buf);

            if let Err(err) = sink.send(Bytes::from(buf)).await {
                error!("serve_stream: send failed: {}", err);
            }
        }

        info!("serve_stream: sink_loop exit");
    });

    Ok(())
}

fn handshake(provider: Arc<SocketProvider>, token: &str) -> anyhow::Result<()> {
    RuntimeProvider::current()?.block_on(async move {
        let reply = provider
            .call_server(
                ClientToServerMessage::HandshakeRequest(HandshakeRequest {
                    token: token.to_owned(),
                }),
                Duration::from_secs(5),
            )
            .await?;

        if let ServerToClientMessage::HandshakeReply(message) = reply {
            match message.status {
                HandshakeStatus::Accepted => Ok(()),
                HandshakeStatus::Repeated => bail!("handshake: repeated"),
            }
        } else {
            bail!("handshake: mismatched reply message");
        }
    })
}

fn serve_heart_beat(provider: Arc<SocketProvider>) -> anyhow::Result<()> {
    RuntimeProvider::current()?.spawn(async move {
        let mut ticker = tokio::time::interval(Duration::from_secs(60));
        let mut timeout_counter = 0;

        loop {
            ticker.tick().await;

            if let Err(err) = provider
                .call_server(
                    ClientToServerMessage::HeartBeatRequest(HeartBeatRequest {
                        time_stamp: chrono::Utc::now().timestamp() as u32,
                    }),
                    Duration::from_secs(30),
                )
                .await
            {
                error!("serve_heart_beat: timeout: {}", err);
                timeout_counter += 1;
            }

            if timeout_counter >= 3 {
                todo!()
            }
        }
    });

    Ok(())
}

fn select_endpoint(
    local_device_id: String,
    remote_device_id: String,
) -> anyhow::Result<Arc<EndPoint>> {
    if !EndPointProvider::current()?.contains(&remote_device_id) {
        let ep = Arc::new(EndPoint::new(local_device_id, remote_device_id.to_owned()));
        EndPointProvider::current()?.insert(remote_device_id.to_owned(), ep);
    }

    EndPointProvider::current()?
        .get(&remote_device_id)
        .ok_or_else(|| anyhow::anyhow!("select_endpoint: endpoint not found"))
}

fn handle_client_to_client_message(
    endpoint: Arc<EndPoint>,
    call_id: u16,
    message: ClientToClientMessage,
) -> anyhow::Result<()> {
    match message {
        ClientToClientMessage::ConnectRequest(req) => {
            handle_client_to_client_message!(
                endpoint,
                call_id,
                client_to_client_handler::connect,
                req,
                ClientToClientMessage::ConnectReply
            );
        }
        ClientToClientMessage::KeyExchangeAndVerifyPasswordRequest(req) => {
            handle_client_to_client_message!(
                endpoint,
                call_id,
                client_to_client_handler::key_exchange_and_verify_password,
                req,
                ClientToClientMessage::KeyExchangeAndVerifyPasswordReply
            );
        }
        ClientToClientMessage::StartMediaTransmissionRequest(req) => {
            handle_client_to_client_message!(
                endpoint,
                call_id,
                client_to_client_handler::start_media_transmission,
                req,
                ClientToClientMessage::StartMediaTransmissionReply
            );
        }
        _ => {
            SocketProvider::current()?.set_client_call_reply(call_id, message);
        }
    };

    Ok(())
}
