use super::{
    endpoint::EndPoint,
    message::{
        DisplayInfo, GetDisplayInfoRequest, GetDisplayInfoResponse, MediaFrame,
        StartMediaTransmissionResponse,
    },
};
use crate::{
    error::MirrorXError,
    socket::endpoint::{endpoint::ENDPOINTS, message::StartMediaTransmissionRequest},
};

pub async fn handle_get_display_info_request(
    endpoint: &EndPoint,
    req: GetDisplayInfoRequest,
) -> Result<GetDisplayInfoResponse, MirrorXError> {
    let displays = crate::component::monitor::get_active_displays()?;

    // todo: display and display_info has same memory layout, use memory block copy?
    let mut display_info_vec = Vec::with_capacity(displays.len());
    // for display in displays {
    //     display_info_vec.push(DisplayInfo {
    //         id: display.id,
    //         is_main: display.is_main,
    //         screen_shot: display.screen_shot,
    //     })
    // }

    Ok(GetDisplayInfoResponse {
        displays: display_info_vec,
    })
}

pub async fn handle_start_media_transmission_request(
    endpoint: &EndPoint,
    req: StartMediaTransmissionRequest,
) -> Result<StartMediaTransmissionResponse, MirrorXError> {
    let fps = req.expect_fps;

    endpoint.begin_screen_capture()?;

    let reply = StartMediaTransmissionResponse {
        os_name: crate::constants::OS_TYPE
            .get()
            .map(|v| v.clone())
            .unwrap_or(String::from("unknown")),
        os_version: crate::constants::OS_VERSION
            .get()
            .map(|v| v.clone())
            .unwrap_or(String::from("unknown")),
        screen_width: 1920,
        screen_height: 1080,
        video_type: String::from("todo"),
        audio_type: String::from("todo"),
    };

    Ok(reply)
}

pub async fn handle_media_transmission(
    remote_device_id: String,
    media_frame: MediaFrame,
) -> Result<(), MirrorXError> {
    // info!(
    //     data_length = media_frame.data.len(),
    //     timestamp = media_frame.timestamp,
    //     "handle_media_transmission",
    // );

    if let Some(endpoint) = ENDPOINTS.get(&remote_device_id) {
        endpoint.transfer_desktop_video_frame(media_frame.data);
    };

    Ok(())
}
