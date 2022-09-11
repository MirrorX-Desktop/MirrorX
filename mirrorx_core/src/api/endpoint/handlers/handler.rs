use super::{
    endpoint::EndPoint,
    message::{
        AudioFrame, MonitorDescription, NegotiateSelectMonitorRequest,
        NegotiateSelectMonitorResponse, StartMediaTransmissionResponse, VideoFrame,
    },
    processor,
};
use crate::{
    core_error,
    error::{CoreError, CoreResult},
    service::endpoint::message::{
        Input,
        InputEvent::{Keyboard, Mouse},
        KeyboardEvent,
        MouseEvent::{MouseDown, MouseMove, MouseScrollWheel, MouseUp},
        StartMediaTransmissionRequest,
    },
};

pub async fn handle_get_display_info_request(
    endpoint: &EndPoint,
    req: NegotiateSelectMonitorRequest,
) -> CoreResult<NegotiateSelectMonitorResponse> {
    let monitors = crate::component::desktop::monitor::get_active_monitors()?;

    // todo: monitor and display_info has same memory layout, use memory block copy?
    let mut displays = Vec::new();

    for monitor in monitors {
        displays.push(MonitorDescription {
            id: monitor.id,
            name: monitor.name,
            frame_rate: monitor.refresh_rate,
            width: monitor.width,
            height: monitor.height,
            is_primary: monitor.is_primary,
            screen_shot: monitor.screen_shot,
        });
    }

    Ok(NegotiateSelectMonitorResponse { displays })
}

pub async fn handle_start_media_transmission_request(
    endpoint: &EndPoint,
    req: StartMediaTransmissionRequest,
) -> CoreResult<StartMediaTransmissionResponse> {
    // endpoint.start_audio_capture().await?;

    let monitor = endpoint
        .start_video_capture(&req.expect_display_id, req.expect_fps)
        .await?;

    let reply = StartMediaTransmissionResponse {
        os_type: crate::constants::os::OS_TYPE.clone(),
        os_version: crate::constants::os::OS_VERSION
            .get()
            .map(|v| v.clone())
            .unwrap_or(String::from("unknown")),
        screen_width: monitor.width,
        screen_height: monitor.height,
        video_type: String::from("todo"),
        audio_type: String::from("todo"),
    };

    Ok(reply)
}

pub async fn handle_input(endpoint: &EndPoint, input: Input) -> CoreResult<()> {
    match input.event {
        Mouse(event) => {
            if let Some(monitor) = endpoint.monitor() {
                match event {
                    MouseUp(key, x, y) => processor::input::mouse_up(monitor, key, x, y),
                    MouseDown(key, x, y) => processor::input::mouse_down(monitor, key, x, y),
                    MouseMove(key, x, y) => processor::input::mouse_move(monitor, key, x, y),
                    MouseScrollWheel(delta) => processor::input::mouse_scroll_whell(monitor, delta),
                }
            } else {
                Err(core_error!("no associate monitor with current session"))
            }
        }
        Keyboard(event) => match event {
            KeyboardEvent::KeyUp(key) => processor::input::keyboard_up(key),
            KeyboardEvent::KeyDown(key) => processor::input::keyboard_down(key),
        },
    }
}
