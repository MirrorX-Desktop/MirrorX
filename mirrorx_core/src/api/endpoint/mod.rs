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
    ActiveTCP(SocketAddr),
    ActiveUDP(SocketAddr),
    PassiveTCP(TcpStream),
    PassiveUDP {
        remote_addr: SocketAddr,
        socket: UdpSocket,
    },
}

pub async fn create_desktop_active_endpoint_client(
    endpoint_id: EndPointID,
    key_pair: Option<(OpeningKey<NonceValue>, SealingKey<NonceValue>)>,
    stream: EndPointStream,
    visit_credentials: Option<Vec<u8>>,
) -> CoreResult<(
    Arc<EndPointClient>,
    tokio::sync::mpsc::Receiver<DesktopDecodeFrame>,
)> {
    let (render_frame_tx, render_frame_rx) = tokio::sync::mpsc::channel(180);
    let (audio_frame_tx, audio_frame_rx) = tokio::sync::mpsc::channel(180);

    let video_frame_tx = serve_video_decode(endpoint_id, render_frame_tx);
    serve_audio_decode(endpoint_id, audio_frame_rx);

    let client = EndPointClient::new_desktop_active(
        endpoint_id,
        key_pair,
        stream,
        video_frame_tx,
        audio_frame_tx,
        visit_credentials,
    )
    .await?;

    Ok((client, render_frame_rx))
}

pub async fn create_file_manager_active_endpoint_client(
    endpoint_id: EndPointID,
    key_pair: Option<(OpeningKey<NonceValue>, SealingKey<NonceValue>)>,
    stream: EndPointStream,
    visit_credentials: Option<Vec<u8>>,
) -> CoreResult<Arc<EndPointClient>> {
    let client =
        EndPointClient::new_file_manager_active(endpoint_id, key_pair, stream, visit_credentials)
            .await?;

    Ok(client)
}

pub async fn create_passive_endpoint_client(
    endpoint_id: EndPointID,
    key_pair: Option<(OpeningKey<NonceValue>, SealingKey<NonceValue>)>,
    stream: EndPointStream,
    visit_credentials: Option<Vec<u8>>,
) -> CoreResult<()> {
    EndPointClient::new_passive(endpoint_id, key_pair, stream, visit_credentials).await?;
    Ok(())
}
