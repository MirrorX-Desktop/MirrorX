use crate::{
    error::MirrorXError,
    service::endpoint::message::{MouseEvent, MouseEventFrame, MouseKey},
};
use core_graphics::{
    display::{CGDirectDisplayID, CGDisplayMoveCursorToPoint, CGPoint},
    event::{CGEvent, CGEventTapLocation, CGEventType, CGMouseButton, ScrollEventUnit},
    event_source::{CGEventSource, CGEventSourceStateID},
};

pub fn mouse_up(key: MouseKey, position: (f32, f32)) -> Result<(), MirrorXError> {
    let (mouse_type, mouse_button) = match key {
        MouseKey::None => return Err(MirrorXError::Other(anyhow::anyhow!("unsupport key"))),
        MouseKey::Left => (CGEventType::LeftMouseUp, CGMouseButton::Left),
        MouseKey::Right => (CGEventType::RightMouseUp, CGMouseButton::Right),
        MouseKey::Wheel => (CGEventType::ScrollWheel, CGMouseButton::Center),
    };

    let event_source = CGEventSource::new(CGEventSourceStateID::HIDSystemState)
        .map_err(|err| MirrorXError::Other(anyhow::anyhow!("create CGEventSource failed")))?;

    let event = CGEvent::new_mouse_event(
        event_source,
        mouse_type,
        CGPoint::new(position.0 as f64, position.1 as f64),
        mouse_button,
    )
    .map_err(|err| MirrorXError::Other(anyhow::anyhow!("create CGEvent failed")))?;

    event.post(CGEventTapLocation::HID);

    Ok(())
}

pub fn mouse_down(key: MouseKey, position: (f32, f32)) -> Result<(), MirrorXError> {
    let (mouse_type, mouse_button) = match key {
        MouseKey::None => return Err(MirrorXError::Other(anyhow::anyhow!("unsupport key"))),
        MouseKey::Left => (CGEventType::LeftMouseDown, CGMouseButton::Left),
        MouseKey::Right => (CGEventType::RightMouseDown, CGMouseButton::Right),
        MouseKey::Wheel => (CGEventType::ScrollWheel, CGMouseButton::Center),
    };

    let event_source = CGEventSource::new(CGEventSourceStateID::HIDSystemState)
        .map_err(|err| MirrorXError::Other(anyhow::anyhow!("create CGEventSource failed")))?;

    let event = CGEvent::new_mouse_event(
        event_source,
        mouse_type,
        CGPoint::new(position.0 as f64, position.1 as f64),
        mouse_button,
    )
    .map_err(|err| MirrorXError::Other(anyhow::anyhow!("create CGEvent failed")))?;

    event.post(CGEventTapLocation::HID);

    Ok(())
}

pub fn mouse_move(
    display_id: &str,
    key: MouseKey,
    position: (f32, f32),
) -> Result<(), MirrorXError> {
    let display_id = display_id
        .parse::<u32>()
        .map_err(|err| MirrorXError::Other(anyhow::anyhow!(err)))?;

    let (mouse_type, mouse_button) = match key {
        MouseKey::None => (CGEventType::MouseMoved, CGMouseButton::Left),
        MouseKey::Left => (CGEventType::LeftMouseDragged, CGMouseButton::Left),
        MouseKey::Right => (CGEventType::RightMouseDragged, CGMouseButton::Right),
        MouseKey::Wheel => (CGEventType::OtherMouseDragged, CGMouseButton::Center),
    };

    let event_source = CGEventSource::new(CGEventSourceStateID::HIDSystemState)
        .map_err(|err| MirrorXError::Other(anyhow::anyhow!("create CGEventSource failed")))?;

    let point = CGPoint::new(position.0 as f64, position.1 as f64);

    let event =
        CGEvent::new_mouse_event(event_source, CGEventType::MouseMoved, point, mouse_button)
            .map_err(|err| MirrorXError::Other(anyhow::anyhow!("create CGEvent failed")))?;

    event.post(CGEventTapLocation::HID);

    unsafe {
        if CGDisplayMoveCursorToPoint(display_id, point) != 0 {
            return Err(MirrorXError::Other(anyhow::anyhow!(
                "CGDisplayMoveCursorToPoint failed"
            )));
        }
    }

    Ok(())
}

pub fn mouse_scroll_whell(delta: f32, position: (f32, f32)) -> Result<(), MirrorXError> {
    let event_source = CGEventSource::new(CGEventSourceStateID::HIDSystemState)
        .map_err(|err| MirrorXError::Other(anyhow::anyhow!("create CGEventSource failed")))?;

    let event = CGEvent::new_scroll_event(
        event_source,
        ScrollEventUnit::PIXEL,
        1,
        delta.round() as i32,
        0,
        0,
    )
    .map_err(|err| MirrorXError::Other(anyhow::anyhow!("create CGEvent failed")))?;

    event.post(CGEventTapLocation::HID);

    Ok(())
}
