use crate::{
    api::endpoint::{
        message::{EndPointInput, EndPointMessage, InputEvent, KeyboardEvent, MouseEvent},
        ENDPOINTS, ENDPOINTS_MONITOR, SEND_MESSAGE_TIMEOUT,
    },
    component, core_error,
    error::{CoreError, CoreResult},
};

pub struct InputRequest {
    pub active_device_id: i64,
    pub passive_device_id: i64,
    pub event: Box<InputEvent>,
}

// pub struct InputResponse {}

pub fn input(req: InputRequest) -> CoreResult<()> {
    let message_tx = ENDPOINTS
        .get(&(req.active_device_id, req.passive_device_id))
        .ok_or(core_error!("endpoint not exists"))?;

    let req = EndPointMessage::Input(EndPointInput { event: *req.event });

    if let Err(err) = message_tx.blocking_send(req) {
        return Err(core_error!(
            "negotiate_finished: message send failed ({})",
            err
        ));
    }

    Ok(())
}

pub async fn handle_input(active_device_id: i64, passive_device_id: i64, input: EndPointInput) {
    match input.event {
        InputEvent::Mouse(event) => {
            if let Some(monitor) = ENDPOINTS_MONITOR.get(&(active_device_id, passive_device_id)) {
                match event {
                    MouseEvent::MouseUp(key, x, y) => {
                        let _ = component::input::mouse_up(&monitor, key, x, y);
                    }
                    MouseEvent::MouseDown(key, x, y) => {
                        let _ = component::input::mouse_down(&monitor, key, x, y);
                    }
                    MouseEvent::MouseMove(key, x, y) => {
                        let _ = component::input::mouse_move(&monitor, key, x, y);
                    }
                    MouseEvent::MouseScrollWheel(delta) => {
                        let _ = component::input::mouse_scroll_wheel(&monitor, delta);
                    }
                }
            } else {
                tracing::warn!(
                    ?active_device_id,
                    ?passive_device_id,
                    "monitor id is not exists"
                )
            };
        }
        InputEvent::Keyboard(event) => match event {
            KeyboardEvent::KeyUp(key) => {
                let _ = component::input::keyboard_up(key);
            }
            KeyboardEvent::KeyDown(key) => {
                let _ = component::input::keyboard_down(key);
            }
        },
    };
}
