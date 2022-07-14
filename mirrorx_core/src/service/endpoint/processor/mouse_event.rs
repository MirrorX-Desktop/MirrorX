use crate::{
    component::{self, monitor::Monitor},
    error::MirrorXError,
    service::endpoint::message::{MouseEventFrame, MouseKey},
};

pub fn mouse_up(key: MouseKey, position: (f32, f32)) -> Result<(), MirrorXError> {
    component::mouse_controller::mouse_up(key, position)
}

pub fn mouse_down(key: MouseKey, position: (f32, f32)) -> Result<(), MirrorXError> {
    component::mouse_controller::mouse_down(key, position)
}

pub fn mouse_move(
    monitor: &Monitor,
    key: MouseKey,
    position: (f32, f32),
) -> Result<(), MirrorXError> {
    component::mouse_controller::mouse_move(monitor, key, position)
}

pub fn mouse_scroll_whell(delta: f32, position: (f32, f32)) -> Result<(), MirrorXError> {
    component::mouse_controller::mouse_scroll_whell(delta, position)
}
