use crate::{
    component::monitor::Monitor, error::MirrorXError, service::endpoint::message::MouseKey,
};
use core_graphics::{
    display::{CGDirectDisplayID, CGDisplayMoveCursorToPoint, CGPoint},
    event::{CGEvent, CGEventTapLocation, CGEventType, CGMouseButton, ScrollEventUnit},
    event_source::{CGEventSource, CGEventSourceStateID},
};

pub fn mouse_up(
    monitor: &Monitor,
    key: MouseKey,
    position: (f32, f32),
) -> Result<(), MirrorXError> {
    let display_id = monitor
        .id
        .parse::<u32>()
        .map_err(|err| MirrorXError::Other(anyhow::anyhow!(err)))?;

    let (event_type, mouse_button) = match key {
        MouseKey::None => return Err(MirrorXError::Other(anyhow::anyhow!("unsupport key"))),
        MouseKey::Left => (CGEventType::LeftMouseUp, CGMouseButton::Left),
        MouseKey::Right => (CGEventType::RightMouseUp, CGMouseButton::Right),
        MouseKey::Wheel => (CGEventType::ScrollWheel, CGMouseButton::Center),
    };

    unsafe {
        post_cg_event(
            display_id,
            position.0,
            position.1,
            move |event_source, point| {
                CGEvent::new_mouse_event(
                    event_source,
                    event_type,
                    CGPoint::new(position.0 as f64, position.1 as f64),
                    mouse_button,
                )
                .map_err(|_| MirrorXError::Other(anyhow::anyhow!("create CGEvent failed")))
            },
        )
    }
}

pub fn mouse_down(
    monitor: &Monitor,
    key: MouseKey,
    position: (f32, f32),
) -> Result<(), MirrorXError> {
    let display_id = monitor
        .id
        .parse::<u32>()
        .map_err(|err| MirrorXError::Other(anyhow::anyhow!(err)))?;

    let (event_type, mouse_button) = match key {
        MouseKey::None => return Err(MirrorXError::Other(anyhow::anyhow!("unsupport key"))),
        MouseKey::Left => (CGEventType::LeftMouseDown, CGMouseButton::Left),
        MouseKey::Right => (CGEventType::RightMouseDown, CGMouseButton::Right),
        MouseKey::Wheel => (CGEventType::ScrollWheel, CGMouseButton::Center),
    };

    unsafe {
        post_cg_event(
            display_id,
            position.0,
            position.1,
            move |event_source, point| {
                CGEvent::new_mouse_event(event_source, event_type, point, mouse_button)
                    .map_err(|_| MirrorXError::Other(anyhow::anyhow!("create CGEvent failed")))
            },
        )
    }
}

pub fn mouse_move(
    monitor: &Monitor,
    key: MouseKey,
    position: (f32, f32),
) -> Result<(), MirrorXError> {
    let display_id = monitor
        .id
        .parse::<u32>()
        .map_err(|err| MirrorXError::Other(anyhow::anyhow!(err)))?;

    let (event_type, mouse_button) = match key {
        MouseKey::None => (CGEventType::MouseMoved, CGMouseButton::Left),
        MouseKey::Left => (CGEventType::LeftMouseDragged, CGMouseButton::Left),
        MouseKey::Right => (CGEventType::RightMouseDragged, CGMouseButton::Right),
        MouseKey::Wheel => (CGEventType::OtherMouseDragged, CGMouseButton::Center),
    };

    unsafe {
        post_cg_event(
            display_id,
            position.0,
            position.1,
            move |event_source, point| {
                CGEvent::new_mouse_event(event_source, event_type, point, mouse_button)
                    .map_err(|_| MirrorXError::Other(anyhow::anyhow!("create CGEvent failed")))
            },
        )
    }
}

pub fn mouse_scroll_whell(
    monitor: &Monitor,
    delta: f32,
    position: (f32, f32),
) -> Result<(), MirrorXError> {
    let display_id = monitor
        .id
        .parse::<u32>()
        .map_err(|err| MirrorXError::Other(anyhow::anyhow!(err)))?;

    unsafe {
        post_cg_event(
            display_id,
            position.0,
            position.1,
            move |event_source, point| {
                CGEvent::new_scroll_event(
                    event_source,
                    ScrollEventUnit::PIXEL,
                    1,
                    delta.round() as i32,
                    0,
                    0,
                )
                .map_err(|_| MirrorXError::Other(anyhow::anyhow!("create CGEvent failed")))
            },
        )
    }
}

unsafe fn post_cg_event(
    display_id: CGDirectDisplayID,
    x: f32,
    y: f32,
    event_create_fn: impl Fn(CGEventSource, CGPoint) -> Result<CGEvent, MirrorXError> + 'static + Send,
) -> Result<(), MirrorXError> {
    // todo: use self created serial queue
    dispatch::Queue::global(dispatch::QueuePriority::High).barrier_async(move || {
        if let Ok(event_source) = CGEventSource::new(CGEventSourceStateID::HIDSystemState) {
            let point = CGPoint::new(x as f64, y as f64);

            if let Ok(event) = event_create_fn(event_source, point.clone()) {
                event.post(CGEventTapLocation::HID);
                let _ = CGDisplayMoveCursorToPoint(display_id, point);
            }
        }
    });

    Ok(())
}
