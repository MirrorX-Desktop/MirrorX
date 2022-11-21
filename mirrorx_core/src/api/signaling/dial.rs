use crate::{core_error, error::CoreResult};
use signaling_proto::service::signaling_client::SignalingClient;
use std::time::Duration;
use tonic::{
    codegen::CompressionEncoding,
    transport::{Channel, Uri},
};

pub async fn dial<U>(uri: U) -> CoreResult<SignalingClient<Channel>>
where
    U: TryInto<Uri>,
{
    let uri = uri
        .try_into()
        .map_err(|_| core_error!("invalid uri format"))?;

    let channel = Channel::builder(uri)
        .tcp_nodelay(true)
        .keep_alive_timeout(Duration::from_secs(60))
        .rate_limit(5, Duration::from_secs(1))
        .connect_timeout(Duration::from_secs(10))
        .keep_alive_while_idle(true)
        .initial_connection_window_size(256 * 1024 * 1024)
        .initial_stream_window_size(32 * 1024 * 1024)
        .connect_timeout(Duration::from_secs(5));

    let client = SignalingClient::connect(channel).await?;
    let client = client
        .accept_compressed(CompressionEncoding::Gzip)
        .send_compressed(CompressionEncoding::Gzip);

    Ok(client)
}
