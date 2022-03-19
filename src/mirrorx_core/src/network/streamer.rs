use std::time::Duration;

use bytes::{Bytes, BytesMut};
use futures::stream::{SplitSink, SplitStream};
use futures::{SinkExt, StreamExt};
use log::{error, info};
use tokio::net::{TcpStream, ToSocketAddrs};
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};
use tokio::sync::oneshot::error::TryRecvError;
use tokio::sync::oneshot::{channel, Receiver, Sender};
use tokio::time::timeout;
use tokio_util::codec::{Framed, LengthDelimitedCodec};

pub async fn create_tcp_streamer<A>(
    addr: A,
) -> anyhow::Result<(UnboundedSender<Bytes>, UnboundedReceiver<BytesMut>)>
where
    A: ToSocketAddrs,
{
    let stream = timeout(Duration::from_secs(2), TcpStream::connect(addr)).await??;
    stream.set_nodelay(true)?;

    let mut codec = LengthDelimitedCodec::new();
    codec.set_max_frame_length(16 * 1024 * 1024);

    let framed_stream = Framed::new(stream, codec);
    let (write_sink, read_stream) = framed_stream.split();

    let (read_data_tx, read_data_rx) = unbounded_channel();
    let (write_data_tx, write_data_rx) = unbounded_channel();

    let (read_loop_exit_tx, read_loop_exit_rx) = channel();
    let (write_loop_exit_tx, write_loop_exit_rx) = channel();

    tokio::spawn(read_loop(
        read_stream,
        read_data_tx,
        read_loop_exit_tx,
        write_loop_exit_rx,
    ));

    tokio::spawn(write_loop(
        write_sink,
        write_data_rx,
        write_loop_exit_tx,
        read_loop_exit_rx,
    ));

    Ok((write_data_tx, read_data_rx))
}

async fn read_loop(
    mut read_stream: SplitStream<Framed<TcpStream, LengthDelimitedCodec>>,
    data_tx: UnboundedSender<BytesMut>,
    mut read_loop_exit_tx: Sender<()>,
    mut write_loop_exit_rx: Receiver<()>,
) {
    loop {
        match write_loop_exit_rx.try_recv() {
            Ok(_) => break,
            Err(err) => match err {
                TryRecvError::Empty => {}
                TryRecvError::Closed => break,
            },
        };

        let buf = match timeout(Duration::from_secs(1), read_stream.next()).await {
            Ok(stream_res) => match stream_res {
                Some(res) => match res {
                    Ok(buf) => buf,
                    Err(err) => {
                        error!("read_loop: {:?}", err);
                        break;
                    }
                },
                None => continue,
            },
            Err(err) => continue, // elapsed
        };

        if let Err(err) = data_tx.send(buf) {
            error!("read_loop: {:?}", err);
            break;
        }
    }

    let _ = timeout(Duration::from_secs(1), read_loop_exit_tx.closed()).await;
    info!("read_loop: exited");
}

async fn write_loop(
    mut write_sink: SplitSink<Framed<TcpStream, LengthDelimitedCodec>, Bytes>,
    mut data_rx: UnboundedReceiver<Bytes>,
    mut write_loop_exit_tx: Sender<()>,
    mut read_loop_exit_rx: Receiver<()>,
) {
    loop {
        match read_loop_exit_rx.try_recv() {
            Ok(_) => break,
            Err(err) => match err {
                TryRecvError::Empty => {}
                TryRecvError::Closed => break,
            },
        };

        let buf = match timeout(Duration::from_secs(1), data_rx.recv()).await {
            Ok(recv_res) => match recv_res {
                Some(buf) => buf,
                None => break, // no further values can be sent on the channel, usually means it had been closed
            },
            Err(_) => continue, // elapsed
        };

        if let Err(err) = write_sink.send(buf).await {
            error!("write_loop: {:?}", err);
            break;
        }
    }

    let _ = timeout(Duration::from_secs(1), write_loop_exit_tx.closed()).await;
    info!("write_loop: exited");
}
