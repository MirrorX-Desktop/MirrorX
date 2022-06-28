use tracing::info;

use super::{
    endpoint::EndPoint,
    message::{MediaFrame, StartMediaTransmissionResponse},
};
use crate::{
    error::MirrorXError,
    socket::endpoint::{endpoint::ENDPOINTS, message::StartMediaTransmissionRequest},
};

pub async fn handle_start_media_transmission_request(
    endpoint: &EndPoint,
    req: StartMediaTransmissionRequest,
) -> Result<StartMediaTransmissionResponse, MirrorXError> {
    endpoint.begin_screen_capture()?;

    let reply = StartMediaTransmissionResponse {
        os_name: crate::constants::OS_NAME
            .get()
            .map(|v| v.clone())
            .unwrap_or(String::from("Unknown")),
        os_version: crate::constants::OS_VERSION
            .get()
            .map(|v| v.clone())
            .unwrap_or(String::from("Unknown")),
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
