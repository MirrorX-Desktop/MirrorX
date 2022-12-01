use crate::{api::endpoint::message::EndPointMessage, error::CoreResult};
use bytes::Bytes;
use futures::{
    stream::{SplitSink, SplitStream},
    AsyncWrite, Sink, SinkExt, Stream, StreamExt,
};
use std::net::SocketAddr;
use tokio::net::{TcpStream, UdpSocket};
use tokio_util::{
    codec::{Framed, LengthDelimitedCodec},
    udp::UdpFramed,
};

mod tcp;
mod udp;

// struct FramedSocket<S, Item>
// where
//     S: StreamExt + SinkExt<Item>,
// {
//     sink: SplitSink<S, Item>,
//     stream: SplitStream<S>,
// }

// impl FramedSocket<Framed<TcpStream, LengthDelimitedCodec>, Bytes> {
//     pub fn read(&mut self) {
//         self.stream.next();
//     }

//     pub fn write(&mut self) {
//         self.sink.send(bytes::Bytes::new());
//     }
// }

// impl FramedSocket<UdpFramed<LengthDelimitedCodec>, (Bytes, SocketAddr)> {
//     pub async fn read(&mut self) {
//         let a = self.stream.next().await;
//     }

//     pub fn write(&mut self) {
//         self.sink
//             .send((bytes::Bytes::new(), SocketAddr::V4(todo!())));
//     }
// }
