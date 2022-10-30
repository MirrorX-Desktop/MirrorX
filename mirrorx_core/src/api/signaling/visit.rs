use crate::error::CoreResult;
use serde::Serialize;
use tonic::transport::Channel;

#[derive(Debug, Clone, Serialize)]
pub enum ResourceType {
    Desktop,
    Files,
}

#[derive(Clone)]
pub struct VisitRequest {
    pub local_device_id: i64,
    pub remote_device_id: i64,
    pub resource_type: ResourceType,
}

#[derive(Debug, Clone)]
pub struct VisitResponse {
    pub allow: bool,
}

pub async fn visit(
    mut client: signaling_proto::service::signaling_client::SignalingClient<Channel>,
    req: VisitRequest,
) -> CoreResult<VisitResponse> {
    let resource_type = match req.resource_type {
        ResourceType::Desktop => signaling_proto::message::ResourceType::Desktop,
        ResourceType::Files => signaling_proto::message::ResourceType::Files,
    };

    let resp = client
        .visit(signaling_proto::message::VisitRequest {
            active_device_id: req.local_device_id,
            passive_device_id: req.remote_device_id,
            resource_type: resource_type.into(),
        })
        .await?
        .into_inner();

    Ok(VisitResponse { allow: resp.allow })
}
