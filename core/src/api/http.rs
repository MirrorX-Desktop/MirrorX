use crate::provider::http::{HTTPProvider, RegisterReq, RegisterResp};

pub async fn device_register(device_id: Option<String>) -> anyhow::Result<RegisterResp> {
    HTTPProvider::current()?
        .device_register(RegisterReq { device_id })
        .await
}
