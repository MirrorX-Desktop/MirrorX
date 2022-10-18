use crate::error::CoreResult;
use tonic::transport::Channel;

pub struct RegisterRequest {
    pub device_id: i64,
    pub device_finger_print: String,
}

pub struct RegisterResponse {
    pub device_id: i64,
}

pub async fn register(
    client: &mut signaling_proto::service::signaling_client::SignalingClient<Channel>,
    req: RegisterRequest,
) -> CoreResult<RegisterResponse> {
    let device_id = req.device_id;

    let resp = client
        .register(signaling_proto::message::RegisterRequest {
            device_id,
            device_finger_print: req.device_finger_print.clone(),
        })
        .await?
        .into_inner();

    Ok(RegisterResponse {
        device_id: resp.device_id,
    })
}
