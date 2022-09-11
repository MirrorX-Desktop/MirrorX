use crate::{
    api::endpoint::{message::EndPointMessage, stream_call, RESERVE_ENDPOINTS},
    core_error,
    error::{CoreError, CoreResult},
};
use tokio::sync::mpsc::Sender;

pub struct NegotiateSelectMonitorRequest {
    pub active_device_id: String,
    pub passive_device_id: String,
}

pub struct NegotiateSelectMonitorResponse {}

pub async fn negotiate_select_monitor(
    req: NegotiateSelectMonitorRequest,
) -> CoreResult<NegotiateSelectMonitorResponse> {
    let mut entry = RESERVE_ENDPOINTS
        .get_mut(&(
            req.active_device_id.to_owned(),
            req.passive_device_id.to_owned(),
        ))
        .ok_or(core_error!("reserve endpoint bundle not exists"))?;

    let (stream, _, _) = entry.value_mut();

    let req = crate::api::endpoint::message::NegotiateSelectMonitorRequest {};

    let resp: crate::api::endpoint::message::NegotiateSelectMonitorResponse =
        stream_call(stream, req).await?;

    // todo: handle monitor descriptions

    Ok(NegotiateSelectMonitorResponse {})
}

pub async fn handle_negotiate_select_monitor_request(
    active_device_id: String,
    passive_device_id: String,
    req: crate::api::endpoint::message::NegotiateSelectMonitorRequest,
    message_tx: Sender<EndPointMessage>,
) {
}

pub async fn handle_negotiate_select_monitor_response(
    active_device_id: String,
    passive_device_id: String,
    resp: crate::api::endpoint::message::NegotiateSelectMonitorResponse,
) {
}
