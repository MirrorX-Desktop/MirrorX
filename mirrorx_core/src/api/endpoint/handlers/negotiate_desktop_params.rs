use crate::{
    api::endpoint::{
        message::{
            AudioSampleFormat, EndPointMessage, EndPointNegotiateDesktopParamsRequest,
            EndPointNegotiateDesktopParamsResponse, EndPointNegotiateVisitDesktopParams,
            VideoCodec,
        },
        EndPointClient,
    },
    component::desktop::monitor::get_primary_monitor_params,
};
use cpal::traits::{DeviceTrait, HostTrait};

pub fn handle_negotiate_desktop_params_request(
    client: EndPointClient,
    req: EndPointNegotiateDesktopParamsRequest,
) {
    let resp = negotiate_media_params(req);

    if let Err(err) = client.send_message(EndPointMessage::NegotiateDesktopParamsResponse(resp)) {
        tracing::error!(
            ?err,
            "handle_negotiate_desktop_params_request: reply failed"
        );
    }
}

fn negotiate_media_params(
    _req: EndPointNegotiateDesktopParamsRequest,
) -> EndPointNegotiateDesktopParamsResponse {
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
            return EndPointNegotiateDesktopParamsResponse::Error;
        }
    };

    let default_output_config = match device.default_output_config() {
        Ok(config) => config,
        Err(err) => {
            tracing::error!(?err, "get default audio output config failed");
            return EndPointNegotiateDesktopParamsResponse::Error;
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
        Err(_) => return EndPointNegotiateDesktopParamsResponse::Error,
    }

    EndPointNegotiateDesktopParamsResponse::Params(params)
}
