use crate::{
    component::{
        self,
        desktop::monitor::Monitor,
        input::key::{KeyboardKey, MouseKey},
    },
    error::CoreError,
};

pub fn mouse_up(monitor: &Monitor, key: MouseKey, x: f32, y: f32) -> Result<(), CoreError> {
    component::input::mouse_up(monitor, key, x, y)
}

pub fn mouse_down(monitor: &Monitor, key: MouseKey, x: f32, y: f32) -> Result<(), CoreError> {
    component::input::mouse_down(monitor, key, x, y)
}

pub fn mouse_move(monitor: &Monitor, key: MouseKey, x: f32, y: f32) -> Result<(), CoreError> {
    component::input::mouse_move(monitor, key, x, y)
}

pub fn mouse_scroll_whell(monitor: &Monitor, delta: f32) -> Result<(), CoreError> {
    component::input::mouse_scroll_wheel(monitor, delta)
}

pub fn keyboard_up(key: KeyboardKey) -> Result<(), CoreError> {
    component::input::keyboard_up(key)
}

pub fn keyboard_down(key: KeyboardKey) -> Result<(), CoreError> {
    component::input::keyboard_down(key)
}
