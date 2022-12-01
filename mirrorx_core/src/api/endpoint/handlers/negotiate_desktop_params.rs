use crate::{
    api::endpoint::{
        client::EndPointClient,
        message::{
            EndPointMessage, EndPointNegotiateDesktopParamsRequest,
            EndPointNegotiateDesktopParamsResponse, EndPointNegotiateVisitDesktopParams,
            VideoCodec,
        },
    },
    component::desktop::monitor::get_primary_monitor_params,
};
use std::sync::Arc;

pub fn handle_negotiate_desktop_params_request(
    client: Arc<EndPointClient>,
    req: EndPointNegotiateDesktopParamsRequest,
) {
    let resp = negotiate_media_params(req);

    if let Err(err) = client.send(&EndPointMessage::NegotiateDesktopParamsResponse(resp)) {
        tracing::error!(
            ?err,
            "handle_negotiate_desktop_params_request: reply failed"
        );
    }
}

fn negotiate_media_params(
    _req: EndPointNegotiateDesktopParamsRequest,
) -> EndPointNegotiateDesktopParamsResponse {
    // todo: check support video and audio properties

    let primary_monitor = match get_primary_monitor_params() {
        Ok(monitor) => monitor,
        Err(err) => {
            tracing::error!(?err, "get primary monitor params failed at negotiate stage");
            return EndPointNegotiateDesktopParamsResponse::MonitorError(err.to_string());
        }
    };

    let mut params = EndPointNegotiateVisitDesktopParams {
        video_codec: VideoCodec::H264,
        os_type: String::from(""),
        os_version: String::from(""),
        primary_monitor,
    };

    EndPointNegotiateDesktopParamsResponse::Params(params)
}
