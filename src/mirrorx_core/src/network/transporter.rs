use super::{
    handler::MESSAGE_HANDLERS, proto::opcode::Opcode, transporter_future::TransporterFuture,
};
use crate::{
    network::{
        proto::{factory::create_proto_message, ProtoMessage},
        streamer::create_tcp_streamer,
    },
    util::{BytesReader, BytesWriter},
};
use bytes::{Bytes, BytesMut};
use log::error;
use std::{
    collections::HashMap,
    sync::{atomic, Arc},
    time::Duration,
};
use tokio::sync::{
    mpsc::{UnboundedReceiver, UnboundedSender},
    oneshot::{channel, Sender},
    Mutex,
};

pub struct Transporter {
    callback_register: Arc<Mutex<HashMap<u32, Sender<(u16, Vec<u8>)>>>>,
    call_seq_id: atomic::AtomicU32,
    tx: UnboundedSender<Bytes>,
}

impl Transporter {
    pub async fn new() -> anyhow::Result<Transporter> {
        let (tx, rx) = create_tcp_streamer("127.0.0.1:45555").await?;

        let callback_register = Arc::new(Mutex::new(HashMap::new()));

        tokio::spawn(handle_packet(rx, callback_register.clone(), tx.clone()));

        Ok(Transporter {
            callback_register,
            call_seq_id: atomic::AtomicU32::new(0),
            tx,
        })
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
        let (tx, rx) = channel::<(u16, Vec<u8>)>();

        let mut seq_id = self.call_seq_id.fetch_add(1, atomic::Ordering::SeqCst);
        if seq_id == 0 {
            seq_id = self.call_seq_id.fetch_add(1, atomic::Ordering::SeqCst);
        }

        {
            let mut register = self.callback_register.lock().await;
            register.insert(seq_id, tx);
        }

        if let Err(err) = self.inner_send(seq_id, message) {
            let mut register = self.callback_register.lock().await;
            register.remove(&seq_id);
            return Err(anyhow::anyhow!(err));
        }

        let res = if time_out_duration.is_zero() {
            TransporterFuture::new(rx).await
        } else {
            match tokio::time::timeout(time_out_duration, TransporterFuture::new(rx)).await {
                Ok(r) => r,
                Err(err) => {
                    let mut register = self.callback_register.lock().await;
                    register.remove(&seq_id);
                    return Err(anyhow::anyhow!(err));
                }
            }
        };

        let (opcode, message_buffer) = res?;
        let mut resp_message = R::default();
        if resp_message.opcode() != opcode {
            return Err(anyhow::anyhow!("not matched response message type"));
        }

        resp_message.decode(&message_buffer)?;

        Ok(resp_message)
    }

    fn inner_send<S>(&self, seq_id: u32, message: &S) -> anyhow::Result<()>
    where
        S: ProtoMessage,
    {
        let mut buf = BytesMut::new();

        encode_packet(&mut buf, seq_id, message);

        let bytes = buf.freeze();

        self.tx.send(bytes).or_else(|err| Err(anyhow::anyhow!(err)))
    }
}

async fn handle_packet(
    mut read_data_rx: UnboundedReceiver<BytesMut>,
    callback_register: Arc<Mutex<HashMap<u32, Sender<(u16, Vec<u8>)>>>>,
    resp_tx: UnboundedSender<Bytes>,
) {
    loop {
        let packet = match read_data_rx.recv().await {
            Some(packet) => packet,
            None => continue,
        };

        let (seq_id, opcode, body) = match decode_packet(&packet) {
            Ok(res) => res,
            Err(err) => {
                error!("{}", err);
                continue;
            }
        };

        let find_callback_result = {
            let mut register = callback_register.lock().await;
            register.remove(&seq_id)
        };

        if let Some(tx) = find_callback_result {
            // local machine is the sync_call sender
            if let Err(_) = tx.send((opcode, body.to_vec())) {
                error!("process_data send proto_message failed, drop it");
            }
        } else {
            // local machine is not the sync_call sender, so the received data maybe represent
            // it is the sync_call request (it means local machine need to handle it) or
            // normally one-way packet

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

            let resp_tx_clone = resp_tx.clone();

            tokio::spawn(async move {
                let opcode_enum = match Opcode::try_from(proto_message.opcode()) {
                    Ok(res) => res,
                    Err(_) => {
                        error!("handle_packet: unknown opcode: {}", proto_message.opcode());
                        return;
                    }
                };

                if let Some(handler) = MESSAGE_HANDLERS.get(&opcode_enum) {
                    let resp_message = match handler(proto_message).await {
                        Ok(res) => res,
                        Err(err) => {
                            error!("handle_packet: message handler returns error: {:?}", err);
                            return;
                        }
                    };

                    let mut buf = BytesMut::new();

                    encode_packet(&mut buf, seq_id, resp_message.as_ref());

                    let bytes = buf.freeze();

                    if let Err(err) = resp_tx_clone.send(bytes) {
                        error!("handle_packet: send resp message failed: {:?}", err);
                    }
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
