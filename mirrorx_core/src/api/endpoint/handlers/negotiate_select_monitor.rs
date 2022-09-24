use crate::{
    api::endpoint::{
        message::{
            EndPointMessage, EndPointNegotiateSelectMonitorRequest,
            EndPointNegotiateSelectMonitorResponse, MonitorDescription,
        },
        ENDPOINTS, RECV_MESSAGE_TIMEOUT, SEND_MESSAGE_TIMEOUT,
    },
    core_error,
    error::{CoreError, CoreResult},
};
use dashmap::DashMap;
use once_cell::sync::Lazy;
use tokio::sync::{mpsc, oneshot};

static RESPONSE_CHANNELS: Lazy<
    DashMap<(i64, i64), oneshot::Sender<EndPointNegotiateSelectMonitorResponse>>,
> = Lazy::new(DashMap::new);

pub struct NegotiateSelectMonitorRequest {
    pub active_device_id: i64,
    pub passive_device_id: i64,
}

pub struct NegotiateSelectMonitorResponse {
    pub monitor_descriptions: Vec<MonitorDescription>,
}

pub async fn negotiate_select_monitor(
    req: NegotiateSelectMonitorRequest,
) -> CoreResult<NegotiateSelectMonitorResponse> {
    let message_tx = ENDPOINTS
        .get(&(req.active_device_id, req.passive_device_id))
        .ok_or(core_error!("endpoint not exists"))?;

    let negotiate_req =
        EndPointMessage::NegotiateSelectMonitorRequest(EndPointNegotiateSelectMonitorRequest {});

    let (resp_tx, resp_rx) = tokio::sync::oneshot::channel();
    RESPONSE_CHANNELS.insert(
        (
            req.active_device_id.to_owned(),
            req.passive_device_id.to_owned(),
        ),
        resp_tx,
    );

    if let Err(err) = message_tx
        .send_timeout(negotiate_req, SEND_MESSAGE_TIMEOUT)
        .await
    {
        RESPONSE_CHANNELS.remove(&(req.active_device_id, req.passive_device_id));
        return Err(core_error!(
            "negotiate_select_monitor: message send failed ({})",
            err
        ));
    }

    let negotiate_resp = tokio::time::timeout(RECV_MESSAGE_TIMEOUT, resp_rx).await??;

    Ok(NegotiateSelectMonitorResponse {
        monitor_descriptions: negotiate_resp.monitor_descriptions,
    })
}

pub async fn handle_negotiate_select_monitor_request(
    active_device_id: i64,
    passive_device_id: i64,
    _: EndPointNegotiateSelectMonitorRequest,
    message_tx: mpsc::Sender<EndPointMessage>,
) {
    let resp = match crate::component::desktop::monitor::get_active_monitors(true) {
        Ok(monitors) => {
            let mut displays = Vec::new();

            for monitor in monitors {
                displays.push(MonitorDescription {
                    id: monitor.id,
                    name: monitor.name,
                    frame_rate: monitor.refresh_rate,
                    width: monitor.width,
                    height: monitor.height,
                    is_primary: monitor.is_primary,
                    screen_shot: monitor.screen_shot,
                });
            }

            EndPointMessage::NegotiateSelectMonitorResponse(
                EndPointNegotiateSelectMonitorResponse {
                    monitor_descriptions: displays,
                },
            )
        }
        Err(err) => {
            tracing::error!(?err, "get active monitors failed");
            EndPointMessage::Error
        }
    };

    if let Err(err) = message_tx.send_timeout(resp, SEND_MESSAGE_TIMEOUT).await {
        tracing::error!(
            ?active_device_id,
            ?passive_device_id,
            handler = "handle_negotiate_visit_desktop_params_request",
            ?err,
            "message send timeout"
        )
    }
}

pub async fn handle_negotiate_select_monitor_response(
    active_device_id: i64,
    passive_device_id: i64,
    resp: crate::api::endpoint::message::EndPointNegotiateSelectMonitorResponse,
) {
    if let Some((_, tx)) = RESPONSE_CHANNELS.remove(&(active_device_id, passive_device_id)) {
        let _ = tx.send(resp);
    }
}
