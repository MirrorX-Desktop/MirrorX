use crate::{api::signaling::SignalingClientManager, error::CoreResult};

pub struct HeartbeatRequest {
    pub device_id: i64,
    pub timestamp: u32,
}

pub struct HeartbeatResponse {
    pub timestamp: u32,
}

pub async fn heartbeat(req: HeartbeatRequest) -> CoreResult<HeartbeatResponse> {
    let resp = SignalingClientManager::get_client()
        .await?
        .heartbeat(signaling_proto::message::HeartbeatRequest {
            device_id: req.device_id,
            timestamp: req.timestamp,
        })
        .await?
        .into_inner();

    Ok(HeartbeatResponse {
        timestamp: resp.timestamp,
    })
}
