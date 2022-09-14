use crate::{
    api::endpoint::{
        message::{
            AudioSampleFormat, AudioSampleRate, EndPointMessage,
            EndPointNegotiateVisitDesktopParamsRequest,
            EndPointNegotiateVisitDesktopParamsResponse, VideoCodec,
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
    DashMap<(i64, i64), oneshot::Sender<EndPointNegotiateVisitDesktopParamsResponse>>,
> = Lazy::new(|| DashMap::new());

pub struct NegotiateVisitDesktopParamsRequest {
    pub active_device_id: i64,
    pub passive_device_id: i64,
}

pub struct NegotiateVisitDesktopParamsResponse {
    pub video_codec: VideoCodec,
    pub audio_sample_rate: AudioSampleRate,
    pub audio_sample_format: AudioSampleFormat,
    pub audio_dual_channel: bool,
    pub os_type: String,
    pub os_version: String,
}

pub async fn negotiate_visit_desktop_params(
    req: NegotiateVisitDesktopParamsRequest,
) -> CoreResult<NegotiateVisitDesktopParamsResponse> {
    let message_tx = ENDPOINTS
        .get(&(
            req.active_device_id.to_owned(),
            req.passive_device_id.to_owned(),
        ))
        .ok_or(core_error!("endpoint not exists"))?;

    // todo: check local machine support video and audio properties
    let negotiate_req = EndPointMessage::NegotiateVisitDesktopParamsRequest(
        EndPointNegotiateVisitDesktopParamsRequest {
            video_codecs: vec![VideoCodec::H264],
            audio_max_sample_rate: AudioSampleRate::HZ480000,
            audio_sample_formats: vec![AudioSampleFormat::F32],
            audio_dual_channel: true,
        },
    );

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
            "negotiate_visit_desktop_params: message send failed ({})",
            err
        ));
    }

    let negotiate_resp = tokio::time::timeout(RECV_MESSAGE_TIMEOUT, resp_rx).await??;

    Ok(NegotiateVisitDesktopParamsResponse {
        video_codec: negotiate_resp.video_codec,
        audio_sample_rate: negotiate_resp.audio_sample_rate,
        audio_sample_format: negotiate_resp.audio_sample_format,
        audio_dual_channel: negotiate_resp.audio_dual_channel,
        os_type: negotiate_resp.os_type,
        os_version: negotiate_resp.os_version,
    })
}

pub async fn handle_negotiate_visit_desktop_params_request(
    active_device_id: i64,
    passive_device_id: i64,
    req: EndPointNegotiateVisitDesktopParamsRequest,
    message_tx: mpsc::Sender<EndPointMessage>,
) {
    // todo: check support video and audio properties

    let resp = EndPointMessage::NegotiateVisitDesktopParamsResponse(
        EndPointNegotiateVisitDesktopParamsResponse {
            video_codec: VideoCodec::H264,
            audio_sample_rate: req.audio_max_sample_rate,
            audio_sample_format: AudioSampleFormat::F32,
            audio_dual_channel: req.audio_dual_channel,
            os_type: String::from(""),
            os_version: String::from(""),
        },
    );

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

pub async fn handle_negotiate_visit_desktop_params_response(
    active_device_id: i64,
    passive_device_id: i64,
    resp: EndPointNegotiateVisitDesktopParamsResponse,
) {
    if let Some((_, tx)) = RESPONSE_CHANNELS.remove(&(active_device_id, passive_device_id)) {
        let _ = tx.send(resp);
    }
}
