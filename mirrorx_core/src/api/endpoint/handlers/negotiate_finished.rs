use crate::{
    api::endpoint::{
        message::{
            EndPointMessage,
            EndPointNegotiateFinishedRequest, //EndPointNegotiateFinishedResponse,
        },
        ENDPOINTS, RECV_MESSAGE_TIMEOUT, SEND_MESSAGE_TIMEOUT,
    },
    core_error,
    error::{CoreError, CoreResult},
};
use dashmap::DashMap;
use flutter_rust_bridge::{StreamSink, ZeroCopyBuffer};
use once_cell::sync::Lazy;
use tokio::sync::{mpsc, oneshot};

// static RESPONSE_CHANNELS: Lazy<
//     DashMap<(i64, i64), oneshot::Sender<EndPointNegotiateFinishedResponse>>,
// > = Lazy::new(|| DashMap::new());

pub struct NegotiateFinishedRequest {
    pub active_device_id: i64,
    pub passive_device_id: i64,
    // pub selected_monitor_id: String,
    pub expect_frame_rate: u8,
    pub texture_id: i64,
    pub video_texture_pointer: i64,
    pub update_frame_callback_pointer: i64,
}

pub async fn negotiate_finished(req: NegotiateFinishedRequest) -> CoreResult<()> {
    let message_tx = ENDPOINTS
        .get(&(req.active_device_id, req.passive_device_id))
        .ok_or(core_error!("endpoint not exists"))?;

    let negotiate_req =
        EndPointMessage::NegotiateFinishedRequest(EndPointNegotiateFinishedRequest {
            // selected_monitor_id: req.selected_monitor_id,
            expected_frame_rate: req.expect_frame_rate,
        });

    // let (resp_tx, resp_rx) = tokio::sync::oneshot::channel();
    // RESPONSE_CHANNELS.insert(
    //     (
    //         req.active_device_id.to_owned(),
    //         req.passive_device_id.to_owned(),
    //     ),
    //     resp_tx,
    // );

    if let Err(err) = message_tx
        .send_timeout(negotiate_req, SEND_MESSAGE_TIMEOUT)
        .await
    {
        // RESPONSE_CHANNELS.remove(&(req.active_device_id, req.passive_device_id));
        return Err(core_error!(
            "negotiate_finished: message send failed ({})",
            err
        ));
    }

    // let _ = tokio::time::timeout(RECV_MESSAGE_TIMEOUT, resp_rx).await??;

    Ok(())
}

pub async fn handle_negotiate_finished_request(
    active_device_id: i64,
    passive_device_id: i64,
    _: EndPointNegotiateFinishedRequest,
    message_tx: mpsc::Sender<EndPointMessage>,
) {
    // todo: launch video and audio
}

// pub async fn handle_negotiate_finished_response(
//     active_device_id: i64,
//     passive_device_id: i64,
//     resp: EndPointNegotiateFinishedResponse,
// ) {
//     if let Some((_, tx)) = RESPONSE_CHANNELS.remove(&(active_device_id, passive_device_id)) {
//         let _ = tx.send(resp);
//     }
// }
