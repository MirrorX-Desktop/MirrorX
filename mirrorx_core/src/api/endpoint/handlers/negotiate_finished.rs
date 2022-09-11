use crate::{
    api::endpoint::{message::EndPointMessage, serve, stream_call, RESERVE_ENDPOINTS},
    core_error,
    error::{CoreError, CoreResult},
};
use tokio::sync::mpsc::Sender;

pub struct NegotiateFinishedRequest {
    pub active_device_id: String,
    pub passive_device_id: String,
    pub selected_monitor_id: String,
    pub expect_frame_rate: u8,
}

pub struct NegotiateFinishedResponse {}

pub async fn negotiate_finished(
    req: NegotiateFinishedRequest,
) -> CoreResult<NegotiateFinishedResponse> {
    let ((active_device_id, passive_device_id), (mut stream, opening_key, sealing_key)) =
        RESERVE_ENDPOINTS
            .remove(&(
                req.active_device_id.to_owned(),
                req.passive_device_id.to_owned(),
            ))
            .ok_or(core_error!("reserve endpoint bundle not exists"))?;

    let req = crate::api::endpoint::message::NegotiateFinishedRequest {
        selected_monitor_id: req.selected_monitor_id,
        expected_frame_rate: req.expect_frame_rate,
    };

    let resp: crate::api::endpoint::message::NegotiateFinishedResponse =
        stream_call(&mut stream, req).await?;

    // remove stream from RESERVE_ENDPOINTS and create endpoint to ENDPOINTS

    serve(
        active_device_id,
        passive_device_id,
        stream,
        opening_key,
        sealing_key,
    )?;

    Ok(NegotiateFinishedResponse {})
}

pub async fn handle_negotiate_finished_request(
    active_device_id: String,
    passive_device_id: String,
    req: crate::api::endpoint::message::NegotiateFinishedRequest,
    message_tx: Sender<EndPointMessage>,
) {
}

pub async fn handle_negotiate_finished_response(
    active_device_id: String,
    passive_device_id: String,
    resp: crate::api::endpoint::message::NegotiateFinishedResponse,
) {
}
