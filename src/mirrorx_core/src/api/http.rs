use crate::provider::http::{RegisterReq, RegisterResp};

pub async fn device_register(device_id: Option<String>) -> anyhow::Result<RegisterResp> {
    crate::instance::HTTP_INSTANCE
        .device_register(RegisterReq { device_id })
        .await
}
