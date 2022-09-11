use crate::{
    api::endpoint::{
        message::{AudioSampleFormat, AudioSampleRate, EndPointMessage, VideoCodec},
        stream_call, RESERVE_ENDPOINTS,
    },
    core_error,
    error::{CoreError, CoreResult},
};
use tokio::sync::mpsc::Sender;

pub struct NegotiateVisitDesktopParamsRequest {
    pub active_device_id: String,
    pub passive_device_id: String,
}

pub struct NegotiateVisitDesktopParamsResponse {}

pub async fn negotiate_visit_desktop_params(
    req: NegotiateVisitDesktopParamsRequest,
) -> CoreResult<NegotiateVisitDesktopParamsResponse> {
    let mut entry = RESERVE_ENDPOINTS
        .get_mut(&(
            req.active_device_id.to_owned(),
            req.passive_device_id.to_owned(),
        ))
        .ok_or(core_error!("reserve endpoint bundle not exists"))?;

    let (stream, _, _) = entry.value_mut();

    // todo: check local machine support video and audio properties
    let req = crate::api::endpoint::message::NegotiateVisitDesktopParamsRequest {
        video_codecs: vec![VideoCodec::H264],
        audio_max_sample_rate: AudioSampleRate::HZ480000,
        audio_sample_formats: vec![AudioSampleFormat::F32],
        audio_dual_channel: true,
    };

    let resp: crate::api::endpoint::message::NegotiateFinishedResponse =
        stream_call(stream, req).await?;

    // todo: handle response params

    Ok(NegotiateVisitDesktopParamsResponse {})
}

pub async fn handle_negotiate_visit_desktop_params_request(
    active_device_id: String,
    passive_device_id: String,
    req: crate::api::endpoint::message::NegotiateVisitDesktopParamsRequest,
    message_tx: Sender<EndPointMessage>,
) {
}

pub async fn handle_negotiate_visit_desktop_params_response(
    active_device_id: String,
    passive_device_id: String,
    resp: crate::api::endpoint::message::NegotiateVisitDesktopParamsResponse,
) {
}
