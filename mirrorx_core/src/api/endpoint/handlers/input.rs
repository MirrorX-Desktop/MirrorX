use crate::{
    api::endpoint::{
        message::{EndPointInput, EndPointMessage, InputEvent},
        ENDPOINTS, SEND_MESSAGE_TIMEOUT,
    },
    core_error,
    error::{CoreError, CoreResult},
};

pub struct InputRequest {
    pub active_device_id: i64,
    pub passive_device_id: i64,
    pub event: Box<InputEvent>,
}

// pub struct InputResponse {}

pub async fn input(req: InputRequest) -> CoreResult<()> {
    let message_tx = ENDPOINTS
        .get(&(req.active_device_id, req.passive_device_id))
        .ok_or(core_error!("endpoint not exists"))?;

    let req = EndPointMessage::Input(EndPointInput { event: *req.event });

    if let Err(err) = message_tx.send_timeout(req, SEND_MESSAGE_TIMEOUT).await {
        return Err(core_error!(
            "negotiate_finished: message send failed ({})",
            err
        ));
    }

    Ok(())
}

pub async fn handle_input(active_device_id: i64, passive_device_id: i64, input: EndPointInput) {
    // match input.event {
    //     InputEvent::Mouse(event) => {
    //         if let Some(monitor) = endpoint.monitor() {
    //             match event {
    //                 MouseUp(key, x, y) => processor::input::mouse_up(monitor, key, x, y),
    //                 MouseDown(key, x, y) => processor::input::mouse_down(monitor, key, x, y),
    //                 MouseMove(key, x, y) => processor::input::mouse_move(monitor, key, x, y),
    //                 MouseScrollWheel(delta) => processor::input::mouse_scroll_wheel(monitor, delta),
    //             }
    //         } else {
    //             Err(core_error!("no associate monitor with current session"))
    //         }
    //     }
    //     InputEvent::Keyboard(event) => match event {
    //         KeyboardEvent::KeyUp(key) => processor::input::keyboard_up(key),
    //         KeyboardEvent::KeyDown(key) => processor::input::keyboard_down(key),
    //     },
    // }
}
