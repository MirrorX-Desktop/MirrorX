use tokio::process;

use super::{
    endpoint::EndPoint,
    message::{
        AudioFrame, DisplayInfo, GetDisplayInfoRequest, GetDisplayInfoResponse, MouseEvent,
        MouseEventFrame, StartMediaTransmissionResponse, VideoFrame,
    },
    processor,
};
use crate::{error::MirrorXError, service::endpoint::message::StartMediaTransmissionRequest};

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
    // endpoint.start_audio_capture().await?;

    endpoint
        .start_video_capture(&req.expect_display_id, req.expect_fps)
        .await?;

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

pub async fn handle_video_frame(
    endpoint: &EndPoint,
    video_frame: VideoFrame,
) -> Result<(), MirrorXError> {
    endpoint.enqueue_video_frame(video_frame);
    Ok(())
}

pub async fn handle_audio_frame(
    endpoint: &EndPoint,
    audio_frame: AudioFrame,
) -> Result<(), MirrorXError> {
    endpoint.enqueue_audio_frame(audio_frame);
    Ok(())
}

pub async fn handle_mouse_event_frame(
    endpoint: &EndPoint,
    mouse_event_frame: MouseEventFrame,
) -> Result<(), MirrorXError> {
    match mouse_event_frame.event {
        MouseEvent::Up(key) => processor::mouse_event::mouse_up(key, mouse_event_frame.position),
        MouseEvent::Down(key) => {
            processor::mouse_event::mouse_down(key, mouse_event_frame.position)
        }
        MouseEvent::Move(key) => {
            if let Some(display_id) = endpoint.display_id() {
                processor::mouse_event::mouse_move(&display_id, key, mouse_event_frame.position)
            } else {
                Err(MirrorXError::Other(anyhow::anyhow!(
                    "no associate display id to endpoint"
                )))
            }
        }
        MouseEvent::ScrollWheel(delta) => {
            processor::mouse_event::mouse_scroll_whell(delta, mouse_event_frame.position)
        }
    }
}
