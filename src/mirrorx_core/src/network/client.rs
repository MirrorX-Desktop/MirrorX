use super::{message::Message, packet::Packet};
use crate::network::handler::process_message;
use bincode::{
    config::{LittleEndian, VarintEncoding, WithOtherEndian, WithOtherIntEncoding},
    DefaultOptions, Options,
};
use bytes::Bytes;
use futures::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use lazy_static::lazy_static;
use log::{error, info};
use std::{
    collections::HashMap,
    sync::{
        atomic::{self, AtomicU8},
        Arc,
    },
    time::Duration,
};
use tokio::{
    io::{AsyncRead, AsyncWrite},
    sync::{
        mpsc::{channel, Receiver, Sender},
        Mutex,
    },
    time::timeout,
};
use tokio_util::codec::{Framed, LengthDelimitedCodec};

lazy_static! {
    static ref BIN_CODER: WithOtherIntEncoding<WithOtherEndian<DefaultOptions, LittleEndian>, VarintEncoding> =
        DefaultOptions::new()
            .with_little_endian()
            .with_varint_encoding();
}

pub struct Client {
    caller_tx_register: Arc<Mutex<HashMap<u8, Sender<Message>>>>,
    call_id: AtomicU8,
    sink_tx: Sender<Bytes>,
}

impl Client {
    pub async fn new<T>(stream: T) -> anyhow::Result<Arc<Client>>
    where
        T: AsyncRead + AsyncWrite + Send + 'static,
    {
        let framed_stream = LengthDelimitedCodec::builder()
            .little_endian()
            .max_frame_length(16 * 1024 * 1024)
            .new_framed(stream);

        let (sink, stream) = framed_stream.split();

        let caller_tx_register = Arc::new(Mutex::new(HashMap::new()));

        let (sink_tx, sink_rx) = channel(32);

        let call_id = AtomicU8::new(0);
        let client = Arc::new(Client {
            caller_tx_register,
            call_id,
            sink_tx,
        });

        tokio::spawn(Client::sink_loop(sink, sink_rx));
        tokio::spawn(Client::stream_loop(client.clone(), stream));

        Ok(client)
    }

    pub async fn send(&self, message: Message) -> anyhow::Result<()> {
        self.inner_send(0, message).await
    }

    pub async fn call(
        &self,
        message: Message,
        time_out_duration: Duration,
    ) -> anyhow::Result<Message> {
        if time_out_duration.is_zero() {
            return Err(anyhow::anyhow!(
                "call: every call must have a non-zero timeout"
            ));
        }

        let (tx, mut rx) = channel(1);

        let call_id = self.new_call_id();

        self.register_call(call_id, tx).await;

        if let Err(err) = self.inner_send(call_id, message).await {
            self.remove_call(&call_id).await;
            return Err(anyhow::anyhow!(err));
        }

        match timeout(time_out_duration, rx.recv()).await? {
            Some(message) => Ok(message),
            None => Err(anyhow::anyhow!("call: sender closed")),
        }
    }

    async fn inner_send(&self, call_id: u8, message: Message) -> anyhow::Result<()> {
        let packet = Packet { call_id, message };
        let buf = BIN_CODER.serialize(&packet)?;
        self.sink_tx.send(Bytes::from(buf)).await?;
        Ok(())
    }

    fn new_call_id(&self) -> u8 {
        let mut call_id = self.call_id.fetch_add(1, atomic::Ordering::SeqCst);

        if call_id == 0 {
            call_id = self.call_id.fetch_add(1, atomic::Ordering::SeqCst);
        }

        call_id
    }

    async fn register_call(&self, call_id: u8, tx: Sender<Message>) {
        let mut register = self.caller_tx_register.lock().await;
        register.insert(call_id, tx);
    }

    async fn remove_call(&self, call_id: &u8) -> Option<Sender<Message>> {
        let mut register = self.caller_tx_register.lock().await;
        register.remove(call_id)
    }

    async fn stream_loop<T>(
        client: Arc<Client>,
        mut stream: SplitStream<Framed<T, LengthDelimitedCodec>>,
    ) where
        T: AsyncRead + AsyncWrite + Send + 'static,
    {
        loop {
            let packet_bytes = match stream.next().await {
                Some(Ok(packet)) => packet,
                Some(Err(err)) => {
                    error!("stream_loop: read packet error: {:?}", err);
                    continue;
                }
                None => break,
            };

            let packet = match BIN_CODER.deserialize::<Packet>(&packet_bytes) {
                Ok(packet) => packet,
                Err(err) => {
                    error!("stream_loop: deserialize packet error: {:?}", err);
                    continue;
                }
            };

            match client.remove_call(&packet.call_id).await {
                Some(sender) => {
                    if let Err(err) = sender.send(packet.message).await {
                        error!("stream_loop: send packet to call receiver error: {:?}", err);
                    }
                }
                None => {
                    let client = client.clone();

                    tokio::spawn(async move {
                        match process_message(client.as_ref(), packet.message).await {
                            Ok(Some(message)) => {
                                if let Err(err) = client.inner_send(packet.call_id, message).await {
                                    error!(
                                        "stream_loop: send call response message error: {:?}",
                                        err
                                    );
                                }
                            }
                            Ok(None) => {}
                            Err(_) => {
                                // normally, we should send an error message to caller (remote
                                // machine), but considering current stage of project, to simplify
                                // the process, we just ignore it temporarily.
                            }
                        }
                    });
                }
            };
        }

        info!("stream_loop: exited");
    }

    async fn sink_loop<T>(
        mut sink: SplitSink<Framed<T, LengthDelimitedCodec>, Bytes>,
        mut sink_rx: Receiver<Bytes>,
    ) where
        T: AsyncRead + AsyncWrite + Send + 'static,
    {
        loop {
            let buf = match sink_rx.recv().await {
                Some(buf) => buf,
                None => break,
            };

            if let Err(err) = sink.send(buf).await {
                error!("sink_loop: write buf to sink, {:?}", err);
                break;
            }
        }

        info!("sink_loop: exited");
    }
}
