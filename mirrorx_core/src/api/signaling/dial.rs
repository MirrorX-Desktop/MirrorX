use super::SignalingClientManager;
use crate::{
    core_error,
    error::{CoreError, CoreResult},
};
use signaling_proto::service::signaling_client::SignalingClient;
use std::{convert::TryFrom, time::Duration};
use tonic::{
    codegen::CompressionEncoding,
    transport::{Channel, Uri},
};

pub struct DialRequest {
    pub uri: String,
}

pub async fn dial(req: DialRequest) -> CoreResult<()> {
    let uri = Uri::try_from(req.uri).map_err(|_| core_error!("invalid uri format"))?;
    let domain = uri
        .host()
        .ok_or_else(|| core_error!("invalid uri format"))?
        .to_string();

    let channel = Channel::builder(uri)
        .tcp_nodelay(true)
        .keep_alive_timeout(Duration::from_secs(60))
        .rate_limit(5, Duration::from_secs(1))
        .connect_timeout(Duration::from_secs(10))
        .keep_alive_while_idle(true)
        .initial_connection_window_size(256 * 1024 * 1024)
        .initial_stream_window_size(32 * 1024 * 1024);

    let client = SignalingClient::connect(channel).await?;
    let client = client
        .accept_compressed(CompressionEncoding::Gzip)
        .send_compressed(CompressionEncoding::Gzip);

    SignalingClientManager::set_client(Some(client)).await;

    Ok(())
}

#[tokio::test]
async fn test_dial() {
    dial(DialRequest {
        uri: String::from("mirrorx.cloud"),
    })
    .await
    .unwrap();
}
