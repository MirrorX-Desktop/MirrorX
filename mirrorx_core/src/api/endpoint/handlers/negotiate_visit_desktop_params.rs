use crate::{
    api::endpoint::{
        message::{
            AudioSampleFormat, EndPointMessage, EndPointNegotiateVisitDesktopParams,
            EndPointNegotiateVisitDesktopParamsRequest,
            EndPointNegotiateVisitDesktopParamsResponse, VideoCodec,
        },
        ENDPOINTS, RECV_MESSAGE_TIMEOUT, SEND_MESSAGE_TIMEOUT,
    },
    component::desktop::monitor::get_primary_monitor_params,
    core_error,
    error::{CoreError, CoreResult},
};
use cpal::traits::{DeviceTrait, HostTrait};
use dashmap::DashMap;
use once_cell::sync::Lazy;
use tokio::sync::{mpsc, oneshot};

static RESPONSE_CHANNELS: Lazy<
    DashMap<(i64, i64), oneshot::Sender<EndPointNegotiateVisitDesktopParamsResponse>>,
> = Lazy::new(DashMap::new);

pub struct NegotiateVisitDesktopParamsRequest {
    pub active_device_id: i64,
    pub passive_device_id: i64,
}

pub struct NegotiateVisitDesktopParamsResponse {
    pub video_codec: VideoCodec,
    pub audio_sample_rate: u32,
    pub audio_sample_format: AudioSampleFormat,
    pub audio_channels: u8,
    pub os_type: String,
    pub os_version: String,
    pub monitor_id: String,
    pub monitor_width: i16,
    pub monitor_height: i16,
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
        .send_timeout(Some(negotiate_req), SEND_MESSAGE_TIMEOUT)
        .await
    {
        RESPONSE_CHANNELS.remove(&(req.active_device_id, req.passive_device_id));
        return Err(core_error!(
            "negotiate_visit_desktop_params: message send failed ({})",
            err
        ));
    }

    let negotiate_resp = tokio::time::timeout(RECV_MESSAGE_TIMEOUT, resp_rx).await??;

    match negotiate_resp {
        EndPointNegotiateVisitDesktopParamsResponse::Error => {
            Err(core_error!("negotiate desktop params failed"))
        }
        EndPointNegotiateVisitDesktopParamsResponse::Params(params) => {
            Ok(NegotiateVisitDesktopParamsResponse {
                video_codec: params.video_codec,
                audio_sample_rate: params.audio_sample_rate,
                audio_sample_format: params.audio_sample_format,
                audio_channels: params.audio_channels,
                os_type: params.os_type,
                os_version: params.os_version,
                monitor_id: params.monitor_id,
                monitor_width: params.monitor_width as i16,
                monitor_height: params.monitor_height as i16,
            })
        }
    }
}

pub async fn handle_negotiate_visit_desktop_params_request(
    active_device_id: i64,
    passive_device_id: i64,
    req: EndPointNegotiateVisitDesktopParamsRequest,
    message_tx: mpsc::Sender<Option<EndPointMessage>>,
) {
    let resp = negotiate_media_params(req);

    if let Err(err) = message_tx
        .send_timeout(
            Some(EndPointMessage::NegotiateVisitDesktopParamsResponse(resp)),
            SEND_MESSAGE_TIMEOUT,
        )
        .await
    {
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

fn negotiate_media_params(
    req: EndPointNegotiateVisitDesktopParamsRequest,
) -> EndPointNegotiateVisitDesktopParamsResponse {
    let mut params = EndPointNegotiateVisitDesktopParams {
        video_codec: VideoCodec::H264,
        os_type: String::from(""),
        os_version: String::from(""),
        ..Default::default()
    };

    // todo: check support video and audio properties

    let host = cpal::default_host();
    let device = match host.default_output_device() {
        Some(device) => {
            tracing::info!(name = ?device.name(), "select default audio device");
            device
        }
        None => {
            tracing::error!("get default audio output device failed");
            return EndPointNegotiateVisitDesktopParamsResponse::Error;
        }
    };

    let default_output_config = match device.default_output_config() {
        Ok(config) => config,
        Err(err) => {
            tracing::error!(?err, "get default audio output config failed");
            return EndPointNegotiateVisitDesktopParamsResponse::Error;
        }
    };

    params.audio_sample_rate = default_output_config.sample_rate().0;
    params.audio_sample_format = match default_output_config.sample_format() {
        cpal::SampleFormat::I16 => AudioSampleFormat::I16,
        cpal::SampleFormat::U16 => AudioSampleFormat::U16,
        cpal::SampleFormat::F32 => AudioSampleFormat::F32,
    };
    params.audio_channels = default_output_config.channels() as u8;

    match get_primary_monitor_params() {
        Ok((monitor_id, monitor_width, monitor_height)) => {
            params.monitor_id = monitor_id;
            params.monitor_width = monitor_width;
            params.monitor_height = monitor_height;
        }
        Err(_) => return EndPointNegotiateVisitDesktopParamsResponse::Error,
    }

    EndPointNegotiateVisitDesktopParamsResponse::Params(params)
}
