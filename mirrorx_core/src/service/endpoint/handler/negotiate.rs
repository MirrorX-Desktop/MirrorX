use crate::{
    component::screen::display::Display,
    error::CoreResult,
    service::endpoint::message::{EndPointNegotiateReply, EndPointNegotiateRequest, VideoCodec},
};

pub async fn handle_negotiate_request(
    _req: EndPointNegotiateRequest,
) -> CoreResult<EndPointNegotiateReply> {
    let displays = Display::enum_all_available_displays()?;
    Ok(EndPointNegotiateReply {
        video_codec: VideoCodec::H264,
        os_type: String::default(),
        os_version: String::default(),
        displays,
    })
}
