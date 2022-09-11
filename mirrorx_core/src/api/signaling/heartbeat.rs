use crate::{api::signaling::SignalingClientManager, error::CoreResult};

pub struct HeartbeatRequest {
    pub local_device_id: String,
    pub timestamp: u32,
}

pub struct HeartbeatResponse {
    pub timestamp: u32,
}

pub async fn heartbeat(req: HeartbeatRequest) -> CoreResult<HeartbeatResponse> {
    let resp = SignalingClientManager::get_client()
        .await?
        .heartbeat(signaling_proto::HeartbeatRequest {
            local_device_id: req.local_device_id,
            timestamp: req.timestamp,
        })
        .await?
        .into_inner();

    Ok(HeartbeatResponse {
        timestamp: resp.timestamp,
    })
}
