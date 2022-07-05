use super::{
    endpoint::EndPoint,
    message::{
        DisplayInfo, GetDisplayInfoRequest, GetDisplayInfoResponse, MediaUnit,
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
    let monitors = crate::component::monitor::get_active_monitors()?;

    // todo: monitor and display_info has same memory layout, use memory block copy?
    let mut displays = Vec::new();

    for monitor in monitors {
        displays.push(DisplayInfo {
            id: monitor.id,
            name: monitor.name,
            refresh_rate: monitor.refresh_rate,
            width: monitor.width,
            height: monitor.height,
            is_primary: monitor.is_primary,
            screen_shot: monitor.screen_shot,
        });
    }

    Ok(GetDisplayInfoResponse { displays })
}

pub async fn handle_start_media_transmission_request(
    endpoint: &EndPoint,
    req: StartMediaTransmissionRequest,
) -> Result<StartMediaTransmissionResponse, MirrorXError> {
    let fps = req.expect_fps;
    let display_id = req.expect_display_id;

    endpoint.start_audio_capture()?;
    endpoint.start_video_capture()?;

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
    media_frame: MediaUnit,
) -> Result<(), MirrorXError> {
    // info!(
    //     data_length = media_frame.data.len(),
    //     timestamp = media_frame.timestamp,
    //     "handle_media_transmission",
    // );

    if let Some(endpoint) = ENDPOINTS.get(&remote_device_id) {
        endpoint.enqueue_media_frame(media_frame);
    };

    Ok(())
}
