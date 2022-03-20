use crate::network::call_future::CallFuture;
use crate::network::handler::process_handler;
use crate::network::util::{BytesReader, BytesWriter};
use crate::network::{
    proto::{factory::create_proto_message, ProtoMessage},
    streamer::create_tcp_streamer,
};
use bytes::{Bytes, BytesMut};
use log::error;
use std::{
    collections::HashMap,
    sync::{atomic, Arc},
    time::Duration,
};
use tokio::net::TcpStream;
use tokio::sync::{
    mpsc::{UnboundedReceiver, UnboundedSender},
    oneshot::{channel, Sender},
    Mutex,
};

pub struct Client {
    callback_register: Arc<Mutex<HashMap<u32, Sender<Box<dyn ProtoMessage>>>>>,
    call_id: atomic::AtomicU32,
    tx: UnboundedSender<Bytes>,
}

impl Client {
    pub async fn new(stream: TcpStream) -> anyhow::Result<Arc<Client>> {
        let (tx, rx) = create_tcp_streamer(stream).await?;

        let callback_register = Arc::new(Mutex::new(HashMap::new()));

        let client = Arc::new(Client {
            callback_register: callback_register.clone(),
            call_id: atomic::AtomicU32::new(0),
            tx: tx,
        });

        tokio::spawn(handle_packet(client.clone(), rx));

        Ok(client)
    }

    pub fn send<S>(&self, message: &S) -> anyhow::Result<()>
    where
        S: ProtoMessage,
    {
        self.inner_send(0, message)
    }

    pub async fn call<S, R>(&self, message: &S, time_out_duration: Duration) -> anyhow::Result<R>
    where
        S: ProtoMessage,
        R: ProtoMessage,
    {
        if time_out_duration.is_zero() {
            return Err(anyhow::anyhow!("call: time out duration can't be zero"));
        }

        let (callback_tx, callback_rx) = channel();

        let call_id = self.new_call_id();

        self.register_call(call_id, callback_tx).await;

        if let Err(err) = self.inner_send(call_id, message) {
            self.remove_call(&call_id).await;
            return Err(anyhow::anyhow!(err));
        }

        let proto_message =
            match tokio::time::timeout(time_out_duration, CallFuture::new(callback_rx)).await {
                Ok(res) => match res {
                    Ok(message) => message,
                    Err(err) => {
                        self.remove_call(&call_id).await;
                        return Err(anyhow::anyhow!(err));
                    }
                },
                Err(err) => {
                    self.remove_call(&call_id).await;
                    return Err(anyhow::anyhow!(err));
                }
            };

        let resp_message = match proto_message.downcast::<R>() {
            Ok(res) => res,
            Err(_) => {
                return Err(anyhow::anyhow!("not matched response message type"));
            }
        };

        Ok(*resp_message)
    }

    fn inner_send<S>(&self, call_id: u32, message: &S) -> anyhow::Result<()>
    where
        S: ProtoMessage,
    {
        let mut buf = BytesMut::new();

        encode_packet(&mut buf, call_id, message);

        let bytes = buf.freeze();

        self.tx.send(bytes).or_else(|err| Err(anyhow::anyhow!(err)))
    }

    fn inner_send_call_resp(
        &self,
        call_id: u32,
        message: Box<dyn ProtoMessage>,
    ) -> anyhow::Result<()> {
        let mut buf = BytesMut::new();

        encode_packet(&mut buf, call_id, message.as_ref());

        let bytes = buf.freeze();

        self.tx.send(bytes).or_else(|err| Err(anyhow::anyhow!(err)))
    }

    fn new_call_id(&self) -> u32 {
        let mut call_id = self.call_id.fetch_add(1, atomic::Ordering::SeqCst);

        if call_id == 0 {
            call_id = self.call_id.fetch_add(1, atomic::Ordering::SeqCst);
        }

        call_id
    }

    async fn register_call(&self, call_id: u32, callback_tx: Sender<Box<dyn ProtoMessage>>) {
        let mut register = self.callback_register.lock().await;
        register.insert(call_id, callback_tx);
    }

    async fn remove_call(&self, call_id: &u32) -> Option<Sender<Box<dyn ProtoMessage>>> {
        let mut register = self.callback_register.lock().await;
        register.remove(call_id)
    }
}

async fn handle_packet(client: Arc<Client>, mut read_data_rx: UnboundedReceiver<BytesMut>) {
    loop {
        let packet = match read_data_rx.recv().await {
            Some(packet) => packet,
            None => continue,
        };

        let (call_id, opcode, body) = match decode_packet(&packet) {
            Ok(res) => res,
            Err(err) => {
                error!("{}", err);
                continue;
            }
        };

        let proto_message = match create_proto_message(opcode) {
            Some(mut message) => {
                if let Err(err) = message.decode(body) {
                    error!("handle_packet: decode failed: {:?}", err);
                    continue;
                }
                message
            }
            None => continue,
        };

        if let Some(tx) = client.remove_call(&call_id).await {
            // local machine is the sync_call sender
            if let Err(_) = tx.send(proto_message) {
                error!("process_data send proto_message failed, drop it");
            }
        } else {
            // local machine is not the sync_call sender, so the received data maybe represent
            // it is the sync_call request (it means local machine need to handle it) or
            // normally one-way packet

            let transporter_clone = client.clone();

            tokio::spawn(async move {
                let resp_message =
                    match process_handler(transporter_clone.as_ref(), proto_message).await {
                        Some(res) => res,
                        None => return,
                    };

                if let Err(err) = transporter_clone.inner_send_call_resp(call_id, resp_message) {
                    error!("handle_packet: send resp message failed: {:?}", err);
                }
            });
        }
    }
}

fn encode_packet(mut buf: &mut BytesMut, seq_id: u32, message: &dyn ProtoMessage) {
    let mut writer = BytesWriter::new(&mut buf);
    writer.write_bool(seq_id != 0);

    if seq_id != 0 {
        writer.write_u32(seq_id);
    }

    writer.write_u16(message.opcode());
    message.encode(&mut writer);
}

fn decode_packet(data: &BytesMut) -> anyhow::Result<(u32, u16, &[u8])> {
    let mut reader = BytesReader::new(&data);
    let is_sync_call = reader.read_bool()?;

    let mut seq_id = 0;
    if is_sync_call {
        seq_id = reader.read_u32()?;
    }

    let opcode = reader.read_u16()?;
    let body = reader.read_remaining_bytes()?;

    Ok((seq_id, opcode, body))
}
