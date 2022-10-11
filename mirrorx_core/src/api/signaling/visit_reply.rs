use crate::error::CoreResult;
use tonic::transport::Channel;

pub struct VisitReplyRequest {
    pub domain: String,
    pub active_device_id: i64,
    pub passive_device_id: i64,
    pub allow: bool,
}

pub async fn visit_reply(
    client: &mut signaling_proto::service::signaling_client::SignalingClient<Channel>,
    req: VisitReplyRequest,
) -> CoreResult<()> {
    let _ = client
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
