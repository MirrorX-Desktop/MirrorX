use super::SignalingClientManager;
use crate::error::CoreResult;

pub struct RegisterRequest {
    pub local_device_id: Option<String>,
    pub device_finger_print: String,
}

pub struct RegisterResponse {
    pub device_id: String,
}

pub async fn register(req: RegisterRequest) -> CoreResult<RegisterResponse> {
    let resp = SignalingClientManager::get_client()
        .await?
        .register(signaling_proto::RegisterRequest {
            device_id: req.local_device_id,
            device_finger_print: req.device_finger_print,
        })
        .await?
        .into_inner();

    Ok(RegisterResponse {
        device_id: resp.device_id,
    })
}