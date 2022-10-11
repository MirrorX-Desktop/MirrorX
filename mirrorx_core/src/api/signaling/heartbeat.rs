use crate::error::CoreResult;
use tonic::transport::Channel;

pub async fn heartbeat(
    client: &mut signaling_proto::service::signaling_client::SignalingClient<Channel>,
    device_id: i64,
    timestamp: u32,
) -> CoreResult<u32> {
    let resp = client
        .heartbeat(signaling_proto::message::HeartbeatRequest {
            device_id,
            timestamp,
        })
        .await?
        .into_inner();

    Ok(resp.timestamp)
}
