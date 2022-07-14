use crate::{
    component::{self, monitor::Monitor},
    error::MirrorXError,
    service::endpoint::message::{MouseEventFrame, MouseKey},
};

pub fn mouse_up(
    monitor: &Monitor,
    key: MouseKey,
    position: (f32, f32),
) -> Result<(), MirrorXError> {
    component::mouse::mouse_up(monitor, key, position)
}

pub fn mouse_down(
    monitor: &Monitor,
    key: MouseKey,
    position: (f32, f32),
) -> Result<(), MirrorXError> {
    component::mouse::mouse_down(monitor, key, position)
}

pub fn mouse_move(
    monitor: &Monitor,
    key: MouseKey,
    position: (f32, f32),
) -> Result<(), MirrorXError> {
    component::mouse::mouse_move(monitor, key, position)
}

pub fn mouse_scroll_whell(
    monitor: &Monitor,
    delta: f32,
    position: (f32, f32),
) -> Result<(), MirrorXError> {
    component::mouse::mouse_scroll_whell(monitor, delta, position)
}
