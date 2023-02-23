mod key_code;

use self::key_code::*;
use super::key::MouseKey;
use crate::{component::desktop::monitor::Monitor, core_error, error::CoreResult};
use core_graphics::{
    display::{CGDirectDisplayID, CGDisplayMoveCursorToPoint, CGPoint},
    event::{
        CGEvent, CGEventTapLocation, CGEventType, CGKeyCode, CGMouseButton, EventField,
        ScrollEventUnit,
    },
    event_source::{CGEventSource, CGEventSourceStateID},
};

// https://developer.mozilla.org/en-US/docs/Web/API/UI_Events/Keyboard_event_code_values

pub fn mouse_up(monitor: &Monitor, key: &MouseKey, x: f32, y: f32) -> CoreResult<()> {
    let display_id = monitor.id.parse::<u32>()?;

    let (event_type, mouse_button) = match key {
        MouseKey::None => return Err(core_error!("unsupport key")),
        MouseKey::Left => (CGEventType::LeftMouseUp, CGMouseButton::Left),
        MouseKey::Right => (CGEventType::RightMouseUp, CGMouseButton::Right),
        MouseKey::Wheel => (CGEventType::ScrollWheel, CGMouseButton::Center),
        MouseKey::SideForward => (CGEventType::OtherMouseUp, CGMouseButton::Center),
        MouseKey::SideBack => (CGEventType::OtherMouseUp, CGMouseButton::Center),
    };

    let extra_value = if let CGEventType::OtherMouseDown = event_type {
        if *key == MouseKey::SideForward {
            Some((EventField::MOUSE_EVENT_BUTTON_NUMBER, 0x08))
        } else if *key == MouseKey::SideBack {
            Some((EventField::MOUSE_EVENT_BUTTON_NUMBER, 0x16))
        } else {
            None
        }
    } else {
        None
    };

    unsafe {
        post_mouse_event(display_id, x, y, move |event_source, point| {
            let event = CGEvent::new_mouse_event(event_source, event_type, point, mouse_button)
                .map_err(|_| core_error!("create mouse CGEvent failed"))?;

            if let Some((field, value)) = extra_value {
                event.set_integer_value_field(field, value);
            }

            Ok(vec![event])
        })
    }
}

pub fn mouse_down(monitor: &Monitor, key: &MouseKey, x: f32, y: f32) -> CoreResult<()> {
    let display_id = monitor.id.parse::<u32>()?;

    let (event_type, mouse_button) = match key {
        MouseKey::None => return Err(core_error!("unsupport key")),
        MouseKey::Left => (CGEventType::LeftMouseDown, CGMouseButton::Left),
        MouseKey::Right => (CGEventType::RightMouseDown, CGMouseButton::Right),
        MouseKey::Wheel => (CGEventType::ScrollWheel, CGMouseButton::Center),
        MouseKey::SideForward => (CGEventType::OtherMouseDown, CGMouseButton::Center),
        MouseKey::SideBack => (CGEventType::OtherMouseDown, CGMouseButton::Center),
    };

    let extra_value = if let CGEventType::OtherMouseDown = event_type {
        if *key == MouseKey::SideForward {
            Some((EventField::MOUSE_EVENT_BUTTON_NUMBER, 0x08))
        } else if *key == MouseKey::SideBack {
            Some((EventField::MOUSE_EVENT_BUTTON_NUMBER, 0x16))
        } else {
            None
        }
    } else {
        None
    };

    unsafe {
        post_mouse_event(display_id, x, y, move |event_source, point| {
            let event = CGEvent::new_mouse_event(event_source, event_type, point, mouse_button)
                .map_err(|_| core_error!("create mouse CGEvent failed"))?;

            if let Some((field, value)) = extra_value {
                event.set_integer_value_field(field, value);
            }

            Ok(vec![event])
        })
    }
}

pub fn mouse_move(monitor: &Monitor, key: &MouseKey, x: f32, y: f32) -> CoreResult<()> {
    let display_id = monitor.id.parse::<u32>()?;

    let (event_type, mouse_button) = match key {
        MouseKey::None => (CGEventType::MouseMoved, CGMouseButton::Left),
        MouseKey::Left => (CGEventType::LeftMouseDragged, CGMouseButton::Left),
        MouseKey::Right => (CGEventType::RightMouseDragged, CGMouseButton::Right),
        MouseKey::Wheel => (CGEventType::OtherMouseDragged, CGMouseButton::Center),
        MouseKey::SideForward => (CGEventType::OtherMouseDragged, CGMouseButton::Center),
        MouseKey::SideBack => (CGEventType::OtherMouseDragged, CGMouseButton::Center),
    };

    let extra_value = if let CGEventType::OtherMouseDown = event_type {
        if *key == MouseKey::SideForward {
            Some((EventField::MOUSE_EVENT_BUTTON_NUMBER, 0x08))
        } else if *key == MouseKey::SideBack {
            Some((EventField::MOUSE_EVENT_BUTTON_NUMBER, 0x16))
        } else {
            None
        }
    } else {
        None
    };

    unsafe {
        post_mouse_event(display_id, x, y, move |event_source, point| {
            let event = CGEvent::new_mouse_event(event_source, event_type, point, mouse_button)
                .map_err(|_| core_error!("create mouse CGEvent failed"))?;

            if let Some((field, value)) = extra_value {
                event.set_integer_value_field(field, value);
            }

            Ok(vec![event])
        })
    }
}

pub fn mouse_scroll_wheel(monitor: &Monitor, delta: f32) -> CoreResult<()> {
    let display_id = monitor.id.parse::<u32>()?;

    unsafe {
        post_mouse_event(display_id, 0f32, 0f32, move |event_source, _| {
            if let Ok(event) = CGEvent::new_scroll_event(
                event_source,
                ScrollEventUnit::PIXEL,
                1,
                delta.round() as i32,
                0,
                0,
            ) {
                Ok(vec![event])
            } else {
                Err(core_error!("create scroll CGEvent failed"))
            }
        })
    }
}

pub fn mouse_double_click(monitor: &Monitor, key: &MouseKey, x: f32, y: f32) -> CoreResult<()> {
    let display_id = monitor.id.parse::<u32>()?;

    let (event_type, mouse_button) = match key {
        MouseKey::None => return Err(core_error!("unsupport key")),
        MouseKey::Left => (CGEventType::LeftMouseDown, CGMouseButton::Left),
        MouseKey::Right => (CGEventType::RightMouseDown, CGMouseButton::Right),
        MouseKey::Wheel => (CGEventType::ScrollWheel, CGMouseButton::Center),
        MouseKey::SideForward => (CGEventType::OtherMouseDown, CGMouseButton::Center),
        MouseKey::SideBack => (CGEventType::OtherMouseDown, CGMouseButton::Center),
    };

    let extra_value = if let CGEventType::OtherMouseDown = event_type {
        if *key == MouseKey::SideForward {
            Some((EventField::MOUSE_EVENT_BUTTON_NUMBER, 0x08))
        } else if *key == MouseKey::SideBack {
            Some((EventField::MOUSE_EVENT_BUTTON_NUMBER, 0x16))
        } else {
            None
        }
    } else {
        None
    };

    unsafe {
        post_mouse_event(display_id, x, y, move |event_source, point| {
            let event = CGEvent::new_mouse_event(event_source, event_type, point, mouse_button)
                .map_err(|_| core_error!("create mouse CGEvent failed"))?;

            event.set_integer_value_field(EventField::MOUSE_EVENT_CLICK_STATE, 2);

            if let Some((field, value)) = extra_value {
                event.set_integer_value_field(field, value);
            }

            Ok(vec![event])
        })
    }
}

pub fn keyboard_up(key: &tao::keyboard::KeyCode) -> CoreResult<()> {
    post_keyboard_event(key, false)
}

pub fn keyboard_down(key: &tao::keyboard::KeyCode) -> CoreResult<()> {
    post_keyboard_event(key, true)
}

unsafe fn post_mouse_event(
    display_id: CGDirectDisplayID,
    x: f32,
    y: f32,
    event_create_fn: impl Fn(CGEventSource, CGPoint) -> CoreResult<Vec<CGEvent>> + 'static + Send,
) -> CoreResult<()> {
    // todo: use self created serial queue
    dispatch::Queue::global(dispatch::QueuePriority::High).barrier_async(move || {
        if let Ok(event_source) = CGEventSource::new(CGEventSourceStateID::HIDSystemState) {
            let point = CGPoint::new(x as f64, y as f64);

            if let Ok(events) = event_create_fn(event_source, point) {
                for event in events.iter() {
                    event.post(CGEventTapLocation::HID);
                }

                let _ = CGDisplayMoveCursorToPoint(display_id, point);
            }
        }
    });

    Ok(())
}

fn post_keyboard_event(key: &tao::keyboard::KeyCode, press: bool) -> CoreResult<()> {
    if let Some(vk_key) = map_key_code(key) {
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
    } else {
        Ok(())
    }
}

const fn map_key_code(key: &tao::keyboard::KeyCode) -> Option<CGKeyCode> {
    match key {
        tao::keyboard::KeyCode::Unidentified(_) => None,
        tao::keyboard::KeyCode::Backquote => Some(kVK_ANSI_Grave),
        tao::keyboard::KeyCode::Backslash => Some(kVK_ANSI_Backslash),
        tao::keyboard::KeyCode::BracketLeft => Some(kVK_ANSI_LeftBracket),
        tao::keyboard::KeyCode::BracketRight => Some(kVK_ANSI_RightBracket),
        tao::keyboard::KeyCode::Comma => Some(kVK_ANSI_Comma),
        tao::keyboard::KeyCode::Digit0 => Some(kVK_ANSI_0),
        tao::keyboard::KeyCode::Digit1 => Some(kVK_ANSI_1),
        tao::keyboard::KeyCode::Digit2 => Some(kVK_ANSI_2),
        tao::keyboard::KeyCode::Digit3 => Some(kVK_ANSI_3),
        tao::keyboard::KeyCode::Digit4 => Some(kVK_ANSI_4),
        tao::keyboard::KeyCode::Digit5 => Some(kVK_ANSI_5),
        tao::keyboard::KeyCode::Digit6 => Some(kVK_ANSI_6),
        tao::keyboard::KeyCode::Digit7 => Some(kVK_ANSI_7),
        tao::keyboard::KeyCode::Digit8 => Some(kVK_ANSI_8),
        tao::keyboard::KeyCode::Digit9 => Some(kVK_ANSI_9),
        tao::keyboard::KeyCode::Equal => Some(kVK_ANSI_Equal),
        tao::keyboard::KeyCode::IntlBackslash => Some(kVK_ANSI_Backslash),
        tao::keyboard::KeyCode::IntlRo => None,
        tao::keyboard::KeyCode::IntlYen => Some(kVK_JIS_Yen),
        tao::keyboard::KeyCode::KeyA => Some(kVK_ANSI_A),
        tao::keyboard::KeyCode::KeyB => Some(kVK_ANSI_B),
        tao::keyboard::KeyCode::KeyC => Some(kVK_ANSI_C),
        tao::keyboard::KeyCode::KeyD => Some(kVK_ANSI_D),
        tao::keyboard::KeyCode::KeyE => Some(kVK_ANSI_E),
        tao::keyboard::KeyCode::KeyF => Some(kVK_ANSI_F),
        tao::keyboard::KeyCode::KeyG => Some(kVK_ANSI_G),
        tao::keyboard::KeyCode::KeyH => Some(kVK_ANSI_H),
        tao::keyboard::KeyCode::KeyI => Some(kVK_ANSI_I),
        tao::keyboard::KeyCode::KeyJ => Some(kVK_ANSI_J),
        tao::keyboard::KeyCode::KeyK => Some(kVK_ANSI_K),
        tao::keyboard::KeyCode::KeyL => Some(kVK_ANSI_L),
        tao::keyboard::KeyCode::KeyM => Some(kVK_ANSI_M),
        tao::keyboard::KeyCode::KeyN => Some(kVK_ANSI_N),
        tao::keyboard::KeyCode::KeyO => Some(kVK_ANSI_O),
        tao::keyboard::KeyCode::KeyP => Some(kVK_ANSI_P),
        tao::keyboard::KeyCode::KeyQ => Some(kVK_ANSI_Q),
        tao::keyboard::KeyCode::KeyR => Some(kVK_ANSI_R),
        tao::keyboard::KeyCode::KeyS => Some(kVK_ANSI_S),
        tao::keyboard::KeyCode::KeyT => Some(kVK_ANSI_T),
        tao::keyboard::KeyCode::KeyU => Some(kVK_ANSI_U),
        tao::keyboard::KeyCode::KeyV => Some(kVK_ANSI_V),
        tao::keyboard::KeyCode::KeyW => Some(kVK_ANSI_W),
        tao::keyboard::KeyCode::KeyX => Some(kVK_ANSI_X),
        tao::keyboard::KeyCode::KeyY => Some(kVK_ANSI_Y),
        tao::keyboard::KeyCode::KeyZ => Some(kVK_ANSI_Z),
        tao::keyboard::KeyCode::Minus => Some(kVK_ANSI_Minus),
        tao::keyboard::KeyCode::Plus => None,
        tao::keyboard::KeyCode::Period => Some(kVK_ANSI_Period),
        tao::keyboard::KeyCode::Quote => Some(kVK_ANSI_Quote),
        tao::keyboard::KeyCode::Semicolon => Some(kVK_ANSI_Semicolon),
        tao::keyboard::KeyCode::Slash => Some(kVK_ANSI_Slash),
        tao::keyboard::KeyCode::AltLeft => Some(kVK_Option),
        tao::keyboard::KeyCode::AltRight => Some(kVK_RightOption),
        tao::keyboard::KeyCode::Backspace => Some(kVK_Delete),
        tao::keyboard::KeyCode::CapsLock => Some(kVK_CapsLock),
        tao::keyboard::KeyCode::ContextMenu => None, // Windows only
        tao::keyboard::KeyCode::ControlLeft => Some(kVK_Control),
        tao::keyboard::KeyCode::ControlRight => Some(kVK_RightControl),
        tao::keyboard::KeyCode::Enter => Some(kVK_Return),
        tao::keyboard::KeyCode::SuperLeft => Some(kVK_Command),
        tao::keyboard::KeyCode::SuperRight => Some(kVK_RightCommand),
        tao::keyboard::KeyCode::ShiftLeft => Some(kVK_Shift),
        tao::keyboard::KeyCode::ShiftRight => Some(kVK_RightShift),
        tao::keyboard::KeyCode::Space => Some(kVK_Space),
        tao::keyboard::KeyCode::Tab => Some(kVK_Tab),
        tao::keyboard::KeyCode::Convert => None,
        tao::keyboard::KeyCode::KanaMode => Some(kVK_JIS_Kana),
        tao::keyboard::KeyCode::Lang1 => None,
        tao::keyboard::KeyCode::Lang2 => None,
        tao::keyboard::KeyCode::Lang3 => None,
        tao::keyboard::KeyCode::Lang4 => None,
        tao::keyboard::KeyCode::Lang5 => None,
        tao::keyboard::KeyCode::NonConvert => None,
        tao::keyboard::KeyCode::Delete => Some(kVK_ForwardDelete),
        tao::keyboard::KeyCode::End => Some(kVK_End),
        tao::keyboard::KeyCode::Help => Some(kVK_Help),
        tao::keyboard::KeyCode::Home => Some(kVK_Home),
        tao::keyboard::KeyCode::Insert => None, // Same as Scroll lock
        tao::keyboard::KeyCode::PageDown => Some(kVK_PageDown),
        tao::keyboard::KeyCode::PageUp => Some(kVK_PageUp),
        tao::keyboard::KeyCode::ArrowDown => Some(kVK_DownArrow),
        tao::keyboard::KeyCode::ArrowLeft => Some(kVK_LeftArrow),
        tao::keyboard::KeyCode::ArrowRight => Some(kVK_RightArrow),
        tao::keyboard::KeyCode::ArrowUp => Some(kVK_UpArrow),
        tao::keyboard::KeyCode::NumLock => None,
        tao::keyboard::KeyCode::Numpad0 => Some(kVK_ANSI_Keypad0),
        tao::keyboard::KeyCode::Numpad1 => Some(kVK_ANSI_Keypad1),
        tao::keyboard::KeyCode::Numpad2 => Some(kVK_ANSI_Keypad2),
        tao::keyboard::KeyCode::Numpad3 => Some(kVK_ANSI_Keypad3),
        tao::keyboard::KeyCode::Numpad4 => Some(kVK_ANSI_Keypad4),
        tao::keyboard::KeyCode::Numpad5 => Some(kVK_ANSI_Keypad5),
        tao::keyboard::KeyCode::Numpad6 => Some(kVK_ANSI_Keypad6),
        tao::keyboard::KeyCode::Numpad7 => Some(kVK_ANSI_Keypad7),
        tao::keyboard::KeyCode::Numpad8 => Some(kVK_ANSI_Keypad8),
        tao::keyboard::KeyCode::Numpad9 => Some(kVK_ANSI_Keypad9),
        tao::keyboard::KeyCode::NumpadAdd => Some(kVK_ANSI_KeypadPlus),
        tao::keyboard::KeyCode::NumpadBackspace => None,
        tao::keyboard::KeyCode::NumpadClear => Some(kVK_ANSI_KeypadClear),
        tao::keyboard::KeyCode::NumpadClearEntry => None,
        tao::keyboard::KeyCode::NumpadComma => None,
        tao::keyboard::KeyCode::NumpadDecimal => Some(kVK_ANSI_KeypadDecimal),
        tao::keyboard::KeyCode::NumpadDivide => Some(kVK_ANSI_KeypadDivide),
        tao::keyboard::KeyCode::NumpadEnter => Some(kVK_ANSI_KeypadEnter),
        tao::keyboard::KeyCode::NumpadEqual => Some(kVK_ANSI_KeypadEquals),
        tao::keyboard::KeyCode::NumpadHash => None,
        tao::keyboard::KeyCode::NumpadMemoryAdd => None,
        tao::keyboard::KeyCode::NumpadMemoryClear => None,
        tao::keyboard::KeyCode::NumpadMemoryRecall => None,
        tao::keyboard::KeyCode::NumpadMemoryStore => None,
        tao::keyboard::KeyCode::NumpadMemorySubtract => None,
        tao::keyboard::KeyCode::NumpadMultiply => Some(kVK_ANSI_KeypadMultiply),
        tao::keyboard::KeyCode::NumpadParenLeft => None,
        tao::keyboard::KeyCode::NumpadParenRight => None,
        tao::keyboard::KeyCode::NumpadStar => Some(kVK_ANSI_KeypadMultiply),
        tao::keyboard::KeyCode::NumpadSubtract => Some(kVK_ANSI_KeypadMinus),
        tao::keyboard::KeyCode::Escape => Some(kVK_Escape),
        tao::keyboard::KeyCode::Fn => Some(kVK_Function),
        tao::keyboard::KeyCode::FnLock => None,
        tao::keyboard::KeyCode::PrintScreen => Some(kVK_F13),
        tao::keyboard::KeyCode::ScrollLock => None, // Same as Insert
        tao::keyboard::KeyCode::Pause => None,
        tao::keyboard::KeyCode::BrowserBack => None,
        tao::keyboard::KeyCode::BrowserFavorites => None,
        tao::keyboard::KeyCode::BrowserForward => None,
        tao::keyboard::KeyCode::BrowserHome => None,
        tao::keyboard::KeyCode::BrowserRefresh => None,
        tao::keyboard::KeyCode::BrowserSearch => None,
        tao::keyboard::KeyCode::BrowserStop => None,
        tao::keyboard::KeyCode::Eject => None,
        tao::keyboard::KeyCode::LaunchApp1 => None,
        tao::keyboard::KeyCode::LaunchApp2 => None,
        tao::keyboard::KeyCode::LaunchMail => None,
        tao::keyboard::KeyCode::MediaPlayPause => None,
        tao::keyboard::KeyCode::MediaSelect => None,
        tao::keyboard::KeyCode::MediaStop => None,
        tao::keyboard::KeyCode::MediaTrackNext => None,
        tao::keyboard::KeyCode::MediaTrackPrevious => None,
        tao::keyboard::KeyCode::Power => None,
        tao::keyboard::KeyCode::Sleep => None,
        tao::keyboard::KeyCode::AudioVolumeDown => Some(kVK_VolumeDown),
        tao::keyboard::KeyCode::AudioVolumeMute => Some(kVK_Mute),
        tao::keyboard::KeyCode::AudioVolumeUp => Some(kVK_VolumeUp),
        tao::keyboard::KeyCode::WakeUp => None,
        tao::keyboard::KeyCode::Hyper => None,
        tao::keyboard::KeyCode::Turbo => None,
        tao::keyboard::KeyCode::Abort => None,
        tao::keyboard::KeyCode::Resume => None,
        tao::keyboard::KeyCode::Suspend => None,
        tao::keyboard::KeyCode::Again => None,
        tao::keyboard::KeyCode::Copy => None,
        tao::keyboard::KeyCode::Cut => None,
        tao::keyboard::KeyCode::Find => None,
        tao::keyboard::KeyCode::Open => None,
        tao::keyboard::KeyCode::Paste => None,
        tao::keyboard::KeyCode::Props => None,
        tao::keyboard::KeyCode::Select => None,
        tao::keyboard::KeyCode::Undo => None,
        tao::keyboard::KeyCode::Hiragana => None,
        tao::keyboard::KeyCode::Katakana => None,
        tao::keyboard::KeyCode::F1 => Some(kVK_F1),
        tao::keyboard::KeyCode::F2 => Some(kVK_F2),
        tao::keyboard::KeyCode::F3 => Some(kVK_F3),
        tao::keyboard::KeyCode::F4 => Some(kVK_F4),
        tao::keyboard::KeyCode::F5 => Some(kVK_F5),
        tao::keyboard::KeyCode::F6 => Some(kVK_F6),
        tao::keyboard::KeyCode::F7 => Some(kVK_F7),
        tao::keyboard::KeyCode::F8 => Some(kVK_F8),
        tao::keyboard::KeyCode::F9 => Some(kVK_F9),
        tao::keyboard::KeyCode::F10 => Some(kVK_F10),
        tao::keyboard::KeyCode::F11 => Some(kVK_F11),
        tao::keyboard::KeyCode::F12 => Some(kVK_F12),
        tao::keyboard::KeyCode::F13 => Some(kVK_F13),
        tao::keyboard::KeyCode::F14 => Some(kVK_F14),
        tao::keyboard::KeyCode::F15 => Some(kVK_F15),
        tao::keyboard::KeyCode::F16 => Some(kVK_F16),
        tao::keyboard::KeyCode::F17 => Some(kVK_F17),
        tao::keyboard::KeyCode::F18 => Some(kVK_F18),
        tao::keyboard::KeyCode::F19 => Some(kVK_F19),
        tao::keyboard::KeyCode::F20 => Some(kVK_F20),
        tao::keyboard::KeyCode::F21 => None,
        tao::keyboard::KeyCode::F22 => None,
        tao::keyboard::KeyCode::F23 => None,
        tao::keyboard::KeyCode::F24 => None,
        tao::keyboard::KeyCode::F25 => None,
        tao::keyboard::KeyCode::F26 => None,
        tao::keyboard::KeyCode::F27 => None,
        tao::keyboard::KeyCode::F28 => None,
        tao::keyboard::KeyCode::F29 => None,
        tao::keyboard::KeyCode::F30 => None,
        tao::keyboard::KeyCode::F31 => None,
        tao::keyboard::KeyCode::F32 => None,
        tao::keyboard::KeyCode::F33 => None,
        tao::keyboard::KeyCode::F34 => None,
        tao::keyboard::KeyCode::F35 => None,
        _ => None,
    }
}
