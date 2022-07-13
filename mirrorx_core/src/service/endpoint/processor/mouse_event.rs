use crate::{
    component,
    error::MirrorXError,
    service::endpoint::message::{MouseEventFrame, MouseKey},
};
use core_graphics::display::CGDirectDisplayID;

pub fn mouse_up(key: MouseKey, position: (f32, f32)) -> Result<(), MirrorXError> {
    component::mouse_controller::mouse_up(key, position)
}

pub fn mouse_down(key: MouseKey, position: (f32, f32)) -> Result<(), MirrorXError> {
    component::mouse_controller::mouse_down(key, position)
}

pub fn mouse_move(
    display_id: CGDirectDisplayID,
    key: MouseKey,
    position: (f32, f32),
) -> Result<(), MirrorXError> {
    component::mouse_controller::mouse_move(display_id, key, position)
}

pub fn mouse_scroll_whell(delta: f32, position: (f32, f32)) -> Result<(), MirrorXError> {
    component::mouse_controller::mouse_scroll_whell(delta, position)
}
