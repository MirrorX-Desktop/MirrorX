mod key_code;

use self::key_code::*;
use super::key::{KeyboardKey, MouseKey};
use crate::{
    component::desktop::monitor::Monitor,
    core_error,
    error::{CoreError, CoreResult},
};
use core_graphics::{
    display::{CGDirectDisplayID, CGDisplayMoveCursorToPoint, CGPoint},
    event::{
        CGEvent, CGEventTapLocation, CGEventType, CGKeyCode, CGMouseButton, EventField,
        ScrollEventUnit,
    },
    event_source::{CGEventSource, CGEventSourceStateID},
};

// https://developer.mozilla.org/en-US/docs/Web/API/UI_Events/Keyboard_event_code_values

pub fn mouse_up(monitor: &Monitor, key: MouseKey, x: f32, y: f32) -> CoreResult<()> {
    let display_id = monitor.id.parse::<u32>()?;

    let (event_type, mouse_button) = match key {
        MouseKey::None => return Err(core_error!("unsupport key")),
        MouseKey::Left => (CGEventType::LeftMouseUp, CGMouseButton::Left),
        MouseKey::Right => (CGEventType::RightMouseUp, CGMouseButton::Right),
        MouseKey::Wheel => (CGEventType::ScrollWheel, CGMouseButton::Center),
        MouseKey::SideForward => (CGEventType::OtherMouseUp, CGMouseButton::Center),
        MouseKey::SideBack => (CGEventType::OtherMouseUp, CGMouseButton::Center),
    };

    unsafe {
        post_mouse_event(display_id, x, y, move |event_source, point| {
            let event = CGEvent::new_mouse_event(event_source, event_type, point, mouse_button)
                .map_err(|_| core_error!("create mouse CGEvent failed"))?;

            match event_type {
                CGEventType::OtherMouseUp => {
                    if key == MouseKey::SideForward {
                        event.set_integer_value_field(EventField::MOUSE_EVENT_BUTTON_NUMBER, 0x08);
                    } else if key == MouseKey::SideBack {
                        event.set_integer_value_field(EventField::MOUSE_EVENT_BUTTON_NUMBER, 0x16);
                    }
                }
                _ => {}
            }

            Ok(event)
        })
    }
}

pub fn mouse_down(monitor: &Monitor, key: MouseKey, x: f32, y: f32) -> CoreResult<()> {
    let display_id = monitor.id.parse::<u32>()?;

    let (event_type, mouse_button) = match key {
        MouseKey::None => return Err(core_error!("unsupport key")),
        MouseKey::Left => (CGEventType::LeftMouseDown, CGMouseButton::Left),
        MouseKey::Right => (CGEventType::RightMouseDown, CGMouseButton::Right),
        MouseKey::Wheel => (CGEventType::ScrollWheel, CGMouseButton::Center),
        MouseKey::SideForward => (CGEventType::OtherMouseDown, CGMouseButton::Center),
        MouseKey::SideBack => (CGEventType::OtherMouseDown, CGMouseButton::Center),
    };

    unsafe {
        post_mouse_event(display_id, x, y, move |event_source, point| {
            let event = CGEvent::new_mouse_event(event_source, event_type, point, mouse_button)
                .map_err(|_| core_error!("create mouse CGEvent failed"))?;

            match event_type {
                CGEventType::OtherMouseDown => {
                    if key == MouseKey::SideForward {
                        event.set_integer_value_field(EventField::MOUSE_EVENT_BUTTON_NUMBER, 0x08);
                    } else if key == MouseKey::SideBack {
                        event.set_integer_value_field(EventField::MOUSE_EVENT_BUTTON_NUMBER, 0x16);
                    }
                }
                _ => {}
            }

            Ok(event)
        })
    }
}

pub fn mouse_move(monitor: &Monitor, key: MouseKey, x: f32, y: f32) -> CoreResult<()> {
    let display_id = monitor.id.parse::<u32>()?;

    let (event_type, mouse_button) = match key {
        MouseKey::None => (CGEventType::MouseMoved, CGMouseButton::Left),
        MouseKey::Left => (CGEventType::LeftMouseDragged, CGMouseButton::Left),
        MouseKey::Right => (CGEventType::RightMouseDragged, CGMouseButton::Right),
        MouseKey::Wheel => (CGEventType::OtherMouseDragged, CGMouseButton::Center),
        MouseKey::SideForward => (CGEventType::OtherMouseDragged, CGMouseButton::Center),
        MouseKey::SideBack => (CGEventType::OtherMouseDragged, CGMouseButton::Center),
    };

    unsafe {
        post_mouse_event(display_id, x, y, move |event_source, point| {
            let event = CGEvent::new_mouse_event(event_source, event_type, point, mouse_button)
                .map_err(|_| core_error!("create mouse CGEvent failed"))?;

            match event_type {
                CGEventType::OtherMouseDragged => {
                    if key == MouseKey::SideForward {
                        event.set_integer_value_field(EventField::MOUSE_EVENT_BUTTON_NUMBER, 0x08);
                    } else if key == MouseKey::SideBack {
                        event.set_integer_value_field(EventField::MOUSE_EVENT_BUTTON_NUMBER, 0x16);
                    }
                }
                _ => {}
            }

            Ok(event)
        })
    }
}

pub fn mouse_scroll_wheel(monitor: &Monitor, delta: f32) -> CoreResult<()> {
    let display_id = monitor.id.parse::<u32>()?;

    unsafe {
        post_mouse_event(display_id, 0f32, 0f32, move |event_source, _| {
            CGEvent::new_scroll_event(
                event_source,
                ScrollEventUnit::PIXEL,
                1,
                delta.round() as i32,
                0,
                0,
            )
            .map_err(|_| core_error!("create scroll CGEvent failed"))
        })
    }
}

pub fn keyboard_up(key: KeyboardKey) -> CoreResult<()> {
    post_keyboard_event(key, false)
}

pub fn keyboard_down(key: KeyboardKey) -> CoreResult<()> {
    post_keyboard_event(key, true)
}

unsafe fn post_mouse_event(
    display_id: CGDirectDisplayID,
    x: f32,
    y: f32,
    event_create_fn: impl Fn(CGEventSource, CGPoint) -> CoreResult<CGEvent> + 'static + Send,
) -> CoreResult<()> {
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

fn post_keyboard_event(key: KeyboardKey, press: bool) -> CoreResult<()> {
    let vk_key = map_key_code(key);

    if let Ok(source) = CGEventSource::new(CGEventSourceStateID::HIDSystemState) {
        if let Ok(event) = CGEvent::new_keyboard_event(source, vk_key, press) {
            event.post(CGEventTapLocation::HID);
            Ok(())
        } else {
            Err(core_error!("create keyboard CGEvent failed"))
        }
    } else {
        Err(core_error!("create CGEventSource failed"))
    }
}

const fn map_key_code(key: KeyboardKey) -> CGKeyCode {
    match key {
        KeyboardKey::A => kVK_ANSI_A,
        KeyboardKey::B => kVK_ANSI_B,
        KeyboardKey::C => kVK_ANSI_C,
        KeyboardKey::D => kVK_ANSI_D,
        KeyboardKey::E => kVK_ANSI_E,
        KeyboardKey::F => kVK_ANSI_F,
        KeyboardKey::G => kVK_ANSI_G,
        KeyboardKey::H => kVK_ANSI_H,
        KeyboardKey::I => kVK_ANSI_I,
        KeyboardKey::J => kVK_ANSI_J,
        KeyboardKey::K => kVK_ANSI_K,
        KeyboardKey::L => kVK_ANSI_L,
        KeyboardKey::M => kVK_ANSI_M,
        KeyboardKey::N => kVK_ANSI_N,
        KeyboardKey::O => kVK_ANSI_O,
        KeyboardKey::P => kVK_ANSI_P,
        KeyboardKey::Q => kVK_ANSI_Q,
        KeyboardKey::R => kVK_ANSI_R,
        KeyboardKey::S => kVK_ANSI_S,
        KeyboardKey::T => kVK_ANSI_T,
        KeyboardKey::U => kVK_ANSI_U,
        KeyboardKey::V => kVK_ANSI_V,
        KeyboardKey::W => kVK_ANSI_W,
        KeyboardKey::X => kVK_ANSI_X,
        KeyboardKey::Y => kVK_ANSI_Y,
        KeyboardKey::Z => kVK_ANSI_Z,
        KeyboardKey::BackQuote => kVK_ANSI_Grave, // todo: if ISO keyboard layout -> kVK_ISO_Section
        KeyboardKey::Digit0 => kVK_ANSI_0,
        KeyboardKey::Digit1 => kVK_ANSI_1,
        KeyboardKey::Digit2 => kVK_ANSI_2,
        KeyboardKey::Digit3 => kVK_ANSI_3,
        KeyboardKey::Digit4 => kVK_ANSI_4,
        KeyboardKey::Digit5 => kVK_ANSI_5,
        KeyboardKey::Digit6 => kVK_ANSI_6,
        KeyboardKey::Digit7 => kVK_ANSI_7,
        KeyboardKey::Digit8 => kVK_ANSI_8,
        KeyboardKey::Digit9 => kVK_ANSI_9,
        KeyboardKey::Minus => kVK_ANSI_Minus,
        KeyboardKey::Equal => kVK_ANSI_Equal,
        KeyboardKey::Tab => kVK_Tab,
        KeyboardKey::CapsLock => kVK_CapsLock,
        KeyboardKey::LeftShift => kVK_Shift,
        KeyboardKey::LeftControl => kVK_Control,
        KeyboardKey::LeftAlt => kVK_Command,
        KeyboardKey::LeftMeta => kVK_Option,
        KeyboardKey::Space => kVK_Space,
        KeyboardKey::RightMeta => kVK_RightOption,
        KeyboardKey::RightControl => kVK_RightControl,
        KeyboardKey::RightAlt => kVK_RightCommand,
        KeyboardKey::RightShift => kVK_RightShift,
        KeyboardKey::Comma => kVK_ANSI_Comma,
        KeyboardKey::Period => kVK_ANSI_Period,
        KeyboardKey::Slash => kVK_ANSI_Slash,
        KeyboardKey::Semicolon => kVK_ANSI_Semicolon,
        KeyboardKey::QuoteSingle => kVK_ANSI_Quote,
        KeyboardKey::Enter => kVK_Return,
        KeyboardKey::BracketLeft => kVK_ANSI_LeftBracket,
        KeyboardKey::BracketRight => kVK_ANSI_RightBracket,
        KeyboardKey::BackSlash => kVK_ANSI_Backslash,
        KeyboardKey::Backspace => kVK_Delete,
        KeyboardKey::NumLock => kVK_ANSI_KeypadClear,
        KeyboardKey::NumpadEquals => kVK_ANSI_KeypadEquals,
        KeyboardKey::NumpadDivide => kVK_ANSI_KeypadDivide,
        KeyboardKey::NumpadMultiply => kVK_ANSI_KeypadMultiply,
        KeyboardKey::NumpadSubtract => kVK_ANSI_KeypadMinus,
        KeyboardKey::NumpadAdd => kVK_ANSI_KeypadPlus,
        KeyboardKey::NumpadEnter => kVK_ANSI_KeypadEnter,
        KeyboardKey::Numpad0 => kVK_ANSI_Keypad0,
        KeyboardKey::Numpad1 => kVK_ANSI_Keypad1,
        KeyboardKey::Numpad2 => kVK_ANSI_Keypad2,
        KeyboardKey::Numpad3 => kVK_ANSI_Keypad3,
        KeyboardKey::Numpad4 => kVK_ANSI_Keypad4,
        KeyboardKey::Numpad5 => kVK_ANSI_Keypad5,
        KeyboardKey::Numpad6 => kVK_ANSI_Keypad6,
        KeyboardKey::Numpad7 => kVK_ANSI_Keypad7,
        KeyboardKey::Numpad8 => kVK_ANSI_Keypad8,
        KeyboardKey::Numpad9 => kVK_ANSI_Keypad9,
        KeyboardKey::NumpadDecimal => kVK_ANSI_KeypadDecimal,
        KeyboardKey::ArrowLeft => kVK_LeftArrow,
        KeyboardKey::ArrowUp => kVK_UpArrow,
        KeyboardKey::ArrowRight => kVK_RightArrow,
        KeyboardKey::ArrowDown => kVK_DownArrow,
        KeyboardKey::Escape => kVK_Escape,
        KeyboardKey::PrintScreen => kVK_F13,
        KeyboardKey::ScrollLock => kVK_F14,
        KeyboardKey::Pause => kVK_F15,
        KeyboardKey::Insert => kVK_Help,
        KeyboardKey::Delete => kVK_ForwardDelete,
        KeyboardKey::Home => kVK_Home,
        KeyboardKey::End => kVK_End,
        KeyboardKey::PageUp => kVK_PageUp,
        KeyboardKey::PageDown => kVK_PageDown,
        KeyboardKey::F1 => kVK_F1,
        KeyboardKey::F2 => kVK_F2,
        KeyboardKey::F3 => kVK_F3,
        KeyboardKey::F4 => kVK_F4,
        KeyboardKey::F5 => kVK_F5,
        KeyboardKey::F6 => kVK_F6,
        KeyboardKey::F7 => kVK_F7,
        KeyboardKey::F8 => kVK_F8,
        KeyboardKey::F9 => kVK_F9,
        KeyboardKey::F10 => kVK_F10,
        KeyboardKey::F11 => kVK_F11,
        KeyboardKey::F12 => kVK_F12,
        KeyboardKey::Fn => kVK_Function,
    }
}
