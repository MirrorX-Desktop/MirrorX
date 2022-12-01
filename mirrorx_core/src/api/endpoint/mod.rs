pub mod client;
pub mod handlers;
pub mod id;
pub mod message;

use self::{
    client::EndPointClient,
    handlers::{audio_frame::serve_audio_decode, video_frame::serve_video_decode},
    id::EndPointID,
};
use crate::{error::CoreResult, utility::nonce_value::NonceValue, DesktopDecodeFrame};
use ring::aead::{OpeningKey, SealingKey};
use std::{net::SocketAddr, sync::Arc};
use tokio::net::{TcpStream, UdpSocket};

pub enum EndPointStream {
    PublicTCP(SocketAddr),
    PublicUDP(SocketAddr),
    PrivateTCP(TcpStream),
    PrivateUDP {
        remote_addr: SocketAddr,
        socket: UdpSocket,
    },
}

pub async fn create_active_endpoint_client(
    endpoint_id: EndPointID,
    stream_key: Option<(OpeningKey<NonceValue>, SealingKey<NonceValue>)>,
    stream: EndPointStream,
) -> CoreResult<(
    Arc<EndPointClient>,
    tokio::sync::mpsc::Receiver<DesktopDecodeFrame>,
)> {
    let (render_frame_tx, render_frame_rx) = tokio::sync::mpsc::channel(1);
    let (audio_frame_tx, audio_frame_rx) = tokio::sync::mpsc::channel(1);

    let video_frame_tx = serve_video_decode(endpoint_id, render_frame_tx);
    serve_audio_decode(endpoint_id, audio_frame_rx);

    let client = EndPointClient::new_active(
        endpoint_id,
        stream_key,
        stream,
        video_frame_tx,
        audio_frame_tx,
    )
    .await?;

    Ok((client, render_frame_rx))
}

pub async fn create_passive_endpoint_client(
    endpoint_id: EndPointID,
    stream_key: Option<(OpeningKey<NonceValue>, SealingKey<NonceValue>)>,
    stream: EndPointStream,
) -> CoreResult<()> {
    EndPointClient::new_passive(endpoint_id, stream_key, stream).await?;
    Ok(())
}
