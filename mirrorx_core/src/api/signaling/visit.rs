use crate::error::CoreResult;
use tonic::transport::Channel;

pub enum ResourceType {
    Desktop,
    Files,
}

pub struct VisitRequest {
    pub domain: String,
    pub local_device_id: i64,
    pub remote_device_id: i64,
    pub resource_type: ResourceType,
}

pub struct VisitResponse {
    pub allow: bool,
}

pub async fn visit(
    client: &mut signaling_proto::service::signaling_client::SignalingClient<Channel>,
    req: VisitRequest,
) -> CoreResult<VisitResponse> {
    let resource_type = match req.resource_type {
        ResourceType::Desktop => signaling_proto::message::ResourceType::Desktop,
        ResourceType::Files => signaling_proto::message::ResourceType::Files,
    };

    let resp = client
        .visit(signaling_proto::message::VisitRequest {
            domain: req.domain,
            active_device_id: req.local_device_id,
            passive_device_id: req.remote_device_id,
            resource_type: resource_type.into(),
        })
        .await?
        .into_inner();

    Ok(VisitResponse { allow: resp.allow })
}
