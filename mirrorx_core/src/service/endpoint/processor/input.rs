use crate::{
    component::{
        self,
        input::key::{KeyboardKey, MouseKey},
        monitor::Monitor,
    },
    error::MirrorXError,
};

pub fn mouse_up(monitor: &Monitor, key: MouseKey, x: f32, y: f32) -> Result<(), MirrorXError> {
    component::input::mouse_up(monitor, key, x, y)
}

pub fn mouse_down(monitor: &Monitor, key: MouseKey, x: f32, y: f32) -> Result<(), MirrorXError> {
    component::input::mouse_down(monitor, key, x, y)
}

pub fn mouse_move(monitor: &Monitor, key: MouseKey, x: f32, y: f32) -> Result<(), MirrorXError> {
    component::input::mouse_move(monitor, key, x, y)
}

pub fn mouse_scroll_whell(monitor: &Monitor, delta: f32) -> Result<(), MirrorXError> {
    component::input::mouse_scroll_wheel(monitor, delta)
}

pub fn keyboard_up(key: KeyboardKey) -> Result<(), MirrorXError> {
    component::input::keyboard_up(key)
}

pub fn keyboard_down(key: KeyboardKey) -> Result<(), MirrorXError> {
    component::input::keyboard_down(key)
}
