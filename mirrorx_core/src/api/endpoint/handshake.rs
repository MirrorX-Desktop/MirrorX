use super::{stream_call, RESERVE_ENDPOINTS};
use crate::{
    core_error,
    error::{CoreError, CoreResult},
};

pub struct HandshakeRequest {
    pub active_device_id: String,
    pub passive_device_id: String,
    pub visit_credentials: String,
}

pub struct HandshakeResponse {}

pub async fn handshake(req: HandshakeRequest) -> CoreResult<HandshakeResponse> {
    let mut entry = RESERVE_ENDPOINTS
        .get_mut(&(
            req.active_device_id.to_owned(),
            req.passive_device_id.to_owned(),
        ))
        .ok_or(core_error!("reserve endpoint bundle not exists"))?;

    let (stream, _, _) = entry.value_mut();

    let req = super::message::EndPointHandshakeRequest {
        active_device_id: req.active_device_id,
        passive_device_id: req.passive_device_id,
        visit_credentials: req.visit_credentials,
    };

    let resp: super::message::EndPointHandshakeResponse = stream_call(stream, req).await?;

    Ok(HandshakeResponse {})
}
