use super::SignalingClientManager;
use crate::error::CoreResult;

pub struct VisitReplyRequest {
    pub domain: String,
    pub active_device_id: i64,
    pub passive_device_id: i64,
    pub allow: bool,
}

pub async fn visit_reply(req: VisitReplyRequest) -> CoreResult<()> {
    let _ = SignalingClientManager::get_client()
        .await?
        .visit_reply(signaling_proto::message::VisitReplyRequest {
            domain: req.domain,
            active_device_id: req.active_device_id,
            passive_device_id: req.passive_device_id,
            allow: req.allow,
        })
        .await?
        .into_inner();

    Ok(())
}
