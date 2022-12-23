use crate::{
    api::endpoint::{
        client::EndPointClient,
        message::{EndPointInput, InputEvent, KeyboardEvent, MouseEvent},
    },
    component::{self, desktop::monitor::Monitor, input::key::MouseKey},
};
use std::sync::Arc;

pub async fn handle_input(client: Arc<EndPointClient>, input_event: EndPointInput) {
    for event in input_event.events {
        match event {
            InputEvent::Mouse(event) => {
                if let Some(monitor) = client.monitor().await {
                    handle_mouse(&event, &monitor);
                }
            }
            InputEvent::Keyboard(event) => handle_keyboard(&event),
        }
    }
}

pub fn handle_mouse(event: &MouseEvent, monitor: &Monitor) {
    match event {
        MouseEvent::Up(key, x, y) => {
            let _ = component::input::mouse_up(monitor, key, *x, *y);
        }
        MouseEvent::Down(key, x, y) => {
            let _ = component::input::mouse_down(monitor, key, *x, *y);
        }
        MouseEvent::Move(key, x, y) => {
            let _ = component::input::mouse_move(monitor, key, *x, *y);
        }
        MouseEvent::ScrollWheel(delta) => {
            let _ = component::input::mouse_scroll_wheel(monitor, *delta);
        }
    }
}

pub fn handle_mouse_double_click(key: &MouseKey, x: f32, y: f32, monitor: &Monitor) {
    let _ = component::input::mouse_double_click(monitor, key, x, y);
}

pub fn handle_keyboard(event: &KeyboardEvent) {
    match event {
        KeyboardEvent::KeyUp(key) => {
            let _ = component::input::keyboard_up(key);
        }
        KeyboardEvent::KeyDown(key) => {
            let _ = component::input::keyboard_down(key);
        }
    }
}
