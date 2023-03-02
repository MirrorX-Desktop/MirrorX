mod client;
mod id;

pub mod handler;
pub mod message;

pub use client::{ClientSendStream, EndPointClient};
pub use id::EndPointID;

use self::handler::{audio_frame::serve_audio_decode, video_frame::serve_video_decode};
use crate::{error::CoreResult, utility::nonce_value::NonceValue, DesktopDecodeFrame};
use ring::aead::{OpeningKey, SealingKey};
use std::net::SocketAddr;
use tokio::net::{TcpStream, UdpSocket};

pub enum EndPointStream {
    ActiveTCP(SocketAddr),
    ActiveUDP(SocketAddr),
    PassiveTCP(TcpStream),
    PassiveUDP {
        remote_addr: SocketAddr,
        socket: UdpSocket,
    },
}

pub async fn create_video_and_audio_endpoint_client(
    endpoint_id: EndPointID,
    key_pair: Option<(OpeningKey<NonceValue>, SealingKey<NonceValue>)>,
    stream: EndPointStream,
    visit_credentials: Option<Vec<u8>>,
) -> CoreResult<(
    EndPointClient,
    tokio::sync::mpsc::Receiver<DesktopDecodeFrame>,
)> {
    let (render_frame_tx, render_frame_rx) = tokio::sync::mpsc::channel(180);
    let (audio_frame_tx, audio_frame_rx) = tokio::sync::mpsc::channel(180);

    let video_frame_tx = serve_video_decode(endpoint_id, render_frame_tx);
    serve_audio_decode(endpoint_id, audio_frame_rx);

    tracing::info!("begin client");
    let client = EndPointClient::new_active_endpoint(
        endpoint_id,
        key_pair,
        stream,
        video_frame_tx,
        audio_frame_tx,
        visit_credentials,
    )
    .await?;
    tracing::info!("end client");
    Ok((client, render_frame_rx))
}

pub async fn create_endpoint_client(
    endpoint_id: EndPointID,
    key_pair: Option<(OpeningKey<NonceValue>, SealingKey<NonceValue>)>,
    stream: EndPointStream,
    visit_credentials: Option<Vec<u8>>,
) -> CoreResult<EndPointClient> {
    let client =
        EndPointClient::new_passive_endpoint(endpoint_id, key_pair, stream, visit_credentials)
            .await?;

    Ok(client)
}
