use super::{
    call_id_generator::CallIdGenerator,
    message::{
        client_to_client::ClientToClientMessage,
        client_to_server::{ClientToServerMessage, HeartBeatRequest},
        server_to_client::ServerToClientMessage,
    },
    packet::Packet,
};
use crate::{
    instance::{BINCODE_INSTANCE, RUNTIME_INSTANCE, SOCKET_ENDPOINT_MAP, STREAMER_INSTANCE},
    socket::{client_to_client_handler, endpoint::EndPoint},
};
use anyhow::bail;
use bincode::Options;
use bytes::Bytes;
use dashmap::DashMap;
use futures::{SinkExt, StreamExt};
use log::{error, info, warn};
use std::{sync::Arc, time::Duration};
use tokio::{
    net::{TcpStream, ToSocketAddrs},
    sync::mpsc,
};
use tokio_util::codec::LengthDelimitedCodec;

macro_rules! handle_client_to_client_message {
    ($endpoint:expr, $call_id:ident, $handler:path, $req:ident, $reply_type:path) => {
        RUNTIME_INSTANCE.spawn(async move {
            let res = $handler($endpoint, $req)
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

            if let Err(err) = STREAMER_INSTANCE
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
        });
    };
}

pub struct Streamer {
    tx: mpsc::Sender<Vec<u8>>,
    call_server_tx_map: DashMap<u16, mpsc::Sender<ServerToClientMessage>>,
    call_client_tx_map: DashMap<u16, mpsc::Sender<ClientToClientMessage>>,
    call_id_generator: CallIdGenerator,
}

impl Streamer {
    pub async fn connect<A>(addr: A) -> anyhow::Result<Arc<Self>>
    where
        A: ToSocketAddrs,
    {
        let stream =
            tokio::time::timeout(Duration::from_secs(10), TcpStream::connect(addr)).await??;

        let (tx, rx) = mpsc::channel(16);

        let client = Arc::new(Streamer {
            tx,
            call_server_tx_map: DashMap::new(),
            call_client_tx_map: DashMap::new(),
            call_id_generator: CallIdGenerator::new(),
        });

        serve_stream(stream, rx);
        serve_heart_beat(client.clone());

        Ok(client)
    }

    pub async fn call_client(
        &self,
        from_device_id: String,
        to_device_id: String,
        message: ClientToClientMessage,
        timeout: Duration,
    ) -> anyhow::Result<ClientToClientMessage> {
        let call_id = self.call_id_generator.next();

        let buf = BINCODE_INSTANCE.serialize(&message)?;

        let packet = Packet::ClientToClient(call_id, from_device_id, to_device_id, buf);

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
        let buf = BINCODE_INSTANCE.serialize(&message)?;
        let packet = Packet::ClientToClient(call_id, from_device_id, to_device_id, buf);
        self.send(packet).await
    }

    async fn send(&self, packet: Packet) -> anyhow::Result<()> {
        let buf = BINCODE_INSTANCE.serialize(&packet)?;
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

fn serve_stream(stream: TcpStream, mut client_rx: mpsc::Receiver<Vec<u8>>) {
    let framed_stream = LengthDelimitedCodec::builder()
        .little_endian()
        .max_frame_length(16 * 1024 * 1024)
        .new_framed(stream);

    let (mut sink, mut stream) = framed_stream.split();

    RUNTIME_INSTANCE.spawn(async move {
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
                            error!("serve_stream: stream_loop read packet error: {:?}", err);
                            continue;
                        }
                    },
                },
                None => break,
            };

            let packet = match BINCODE_INSTANCE.deserialize::<Packet>(&packet_bytes) {
                Ok(packet) => packet,
                Err(err) => {
                    error!(
                        "serve_stream: stream_loop deserialize packet error: {:?}",
                        err
                    );
                    continue;
                }
            };

            match packet {
                Packet::ClientToServer(_, message) => {
                    warn!(
                        "serve_stream: received unexpected client to server packet: {:?}",
                        message
                    );
                }
                Packet::ServerToClient(call_id, message) => {
                    if call_id == 0 {
                        // todo
                    }

                    STREAMER_INSTANCE.set_server_call_reply(call_id, message);
                }
                Packet::ClientToClient(call_id, from_device_id, to_device_id, message_bytes) => {
                    if !SOCKET_ENDPOINT_MAP.contains_key(&from_device_id) {
                        let ep = EndPoint::new(to_device_id.clone(), from_device_id.clone());
                        SOCKET_ENDPOINT_MAP.insert(from_device_id.clone(), ep);
                    }

                    let endpoint = match SOCKET_ENDPOINT_MAP.get(&from_device_id) {
                        Some(ep) => ep,
                        None => {
                            error!(
                                "serve_stream: get endpoint failed, from: {}",
                                &from_device_id
                            );
                            continue;
                        }
                    };

                    let message = match BINCODE_INSTANCE
                        .deserialize::<ClientToClientMessage>(&message_bytes)
                    {
                        Ok(message) => message,
                        Err(err) => {
                            error!("client: stream_loop deserialize message error: {:?}", err);
                            continue;
                        }
                    };

                    match message {
                        ClientToClientMessage::ConnectRequest(req) => {
                            handle_client_to_client_message!(
                                endpoint.value(),
                                call_id,
                                client_to_client_handler::connect,
                                req,
                                ClientToClientMessage::ConnectReply
                            );
                        }
                        ClientToClientMessage::KeyExchangeAndVerifyPasswordRequest(req) => {
                            handle_client_to_client_message!(
                                endpoint.value(),
                                call_id,
                                client_to_client_handler::key_exchange_and_verify_password,
                                req,
                                ClientToClientMessage::KeyExchangeAndVerifyPasswordReply
                            );
                        }
                        _ => {
                            STREAMER_INSTANCE.set_client_call_reply(call_id, message);
                        }
                    };
                }
            };
        }

        info!("client: stream_loop exit");
    });

    RUNTIME_INSTANCE.spawn(async move {
        loop {
            let buf = match client_rx.recv().await {
                Some(buf) => buf,
                None => break,
            };

            info!("client: send packet: {:?}", buf);

            if let Err(err) = sink.send(Bytes::from(buf)).await {
                error!("send error: {:?}", err);
            }
        }
    });
}

fn serve_heart_beat(client: Arc<Streamer>) {
    RUNTIME_INSTANCE.spawn(async move {
        let mut ticker = tokio::time::interval(Duration::from_secs(60));
        let mut timeout_counter = 0;

        loop {
            ticker.tick().await;

            if let Err(err) = client
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
}
