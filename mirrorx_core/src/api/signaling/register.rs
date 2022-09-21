use super::SignalingClientManager;
use crate::error::CoreResult;
use rand::RngCore;
use rsa::rand_core::OsRng;

pub struct RegisterRequest {
    pub device_id: Option<i64>,
    pub device_finger_print: Option<String>,
}

pub struct RegisterResponse {
    pub domain: String,
    pub device_id: i64,
    pub device_finger_print: String,
}

pub async fn register(req: RegisterRequest) -> CoreResult<RegisterResponse> {
    let device_id = req.device_id;
    let device_finger_print = req.device_finger_print.unwrap_or_else(|| {
        let mut finger_print_buffer = [0u8; 64];
        OsRng.fill_bytes(&mut finger_print_buffer);
        hex::encode_upper(finger_print_buffer)
    });

    let resp = SignalingClientManager::get_client()
        .await?
        .register(signaling_proto::message::RegisterRequest {
            device_id,
            device_finger_print: device_finger_print.to_owned(),
        })
        .await?
        .into_inner();

    Ok(RegisterResponse {
        domain: resp.domain,
        device_id: resp.device_id,
        device_finger_print,
    })
}
