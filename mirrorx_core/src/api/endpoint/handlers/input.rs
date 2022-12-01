use crate::{
    api::endpoint::{
        client::EndPointClient,
        message::{EndPointInput, InputEvent, KeyboardEvent, MouseEvent},
    },
    component::{self, desktop::monitor::Monitor, input::key::MouseKey},
};
use std::sync::Arc;

pub async fn handle_input(client: Arc<EndPointClient>, input_event: EndPointInput) {
    let mut i = 0;
    while i < input_event.events.len() {
        // if input_event.events.len() - i >= 4 {
        //     if let InputEvent::Mouse(MouseEvent::Up(key1, x1, y1)) = &input_event.events[i] {
        //         if let InputEvent::Mouse(MouseEvent::Up(key2, x2, y2)) = &input_event.events[i + 1]
        //         {
        //             if let InputEvent::Mouse(MouseEvent::Up(key3, x3, y3)) =
        //                 &input_event.events[i + 2]
        //             {
        //                 if let InputEvent::Mouse(MouseEvent::Up(key4, x4, y4)) =
        //                     &input_event.events[i + 3]
        //                 {
        //                     if (key1 == key2 && key2 == key3 && key3 == key4)
        //                         && (x1.max(*x2).max(*x3).max(*x4) - x1.min(*x2).min(*x3).min(*x4))
        //                             < 5.0
        //                         && (y1.max(*y2).max(*y3).max(*y4) - y1.min(*y2).min(*y3).min(*y4))
        //                             < 5.0
        //                     {
        //                         if let Some(entry) = PASSIVE_ENDPOINTS_MONITORS.get(&client.id) {
        //                             handle_mouse_double_click(
        //                                 &key1,
        //                                 (x1 + x2 + x3 + x4) / 4.0,
        //                                 (y1 + y2 + y3 + y4) / 4.0,
        //                                 entry.value(),
        //                             );
        //                         }
        //                         i += 4;
        //                         continue;
        //                     }
        //                 }
        //             }
        //         }
        //     }
        // }

        match &input_event.events[i] {
            InputEvent::Mouse(event) => {
                if let Some(monitor) = client.monitor().await {
                    handle_mouse(event, &monitor);
                }
            }
            InputEvent::Keyboard(event) => handle_keyboard(event),
        }

        i += 1;
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
