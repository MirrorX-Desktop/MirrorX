use super::key::MouseKey;
use crate::{component::desktop::monitor::Monitor, core_error, error::CoreResult};
use windows::Win32::{
    Foundation::GetLastError,
    UI::{Input::KeyboardAndMouse::*, WindowsAndMessaging::*},
};

pub fn mouse_up(monitor: &Monitor, key: &MouseKey, x: f32, y: f32) -> CoreResult<()> {
    let dw_flags = match key {
        MouseKey::None => return Err(core_error!("unsupport key")),
        MouseKey::Left => MOUSEEVENTF_LEFTUP | MOUSEEVENTF_ABSOLUTE | MOUSEEVENTF_VIRTUALDESK,
        MouseKey::Right => MOUSEEVENTF_RIGHTUP | MOUSEEVENTF_ABSOLUTE | MOUSEEVENTF_VIRTUALDESK,
        MouseKey::Wheel => MOUSEEVENTF_MIDDLEUP | MOUSEEVENTF_ABSOLUTE | MOUSEEVENTF_VIRTUALDESK,
        MouseKey::SideForward => MOUSEEVENTF_XUP | MOUSEEVENTF_ABSOLUTE | MOUSEEVENTF_VIRTUALDESK,
        MouseKey::SideBack => MOUSEEVENTF_XDOWN | MOUSEEVENTF_ABSOLUTE | MOUSEEVENTF_VIRTUALDESK,
    };

    let mouse_data = if MouseKey::SideForward == *key {
        VK_XBUTTON1.0 as i32
    } else if MouseKey::SideBack == *key {
        VK_XBUTTON2.0 as i32
    } else {
        0
    };

    unsafe { send_input(&[(mouse_data, dw_flags)], monitor.left, monitor.top, x, y) }
}

pub fn mouse_down(monitor: &Monitor, key: &MouseKey, x: f32, y: f32) -> CoreResult<()> {
    let dw_flags = match key {
        MouseKey::None => return Err(core_error!("unsupport key")),
        MouseKey::Left => MOUSEEVENTF_LEFTDOWN | MOUSEEVENTF_ABSOLUTE | MOUSEEVENTF_VIRTUALDESK,
        MouseKey::Right => MOUSEEVENTF_RIGHTDOWN | MOUSEEVENTF_ABSOLUTE | MOUSEEVENTF_VIRTUALDESK,
        MouseKey::Wheel => MOUSEEVENTF_MIDDLEDOWN | MOUSEEVENTF_ABSOLUTE | MOUSEEVENTF_VIRTUALDESK,
        MouseKey::SideForward => MOUSEEVENTF_XUP | MOUSEEVENTF_ABSOLUTE | MOUSEEVENTF_VIRTUALDESK,
        MouseKey::SideBack => MOUSEEVENTF_XDOWN | MOUSEEVENTF_ABSOLUTE | MOUSEEVENTF_VIRTUALDESK,
    };

    let mouse_data = if MouseKey::SideForward == *key {
        VK_XBUTTON1.0 as i32
    } else if MouseKey::SideBack == *key {
        VK_XBUTTON2.0 as i32
    } else {
        0
    };

    unsafe { send_input(&[(mouse_data, dw_flags)], monitor.left, monitor.top, x, y) }
}

pub fn mouse_double_click(monitor: &Monitor, key: &MouseKey, x: f32, y: f32) -> CoreResult<()> {
    let mut args = Vec::new();

    for _ in 0..2 {
        let down_flags = match key {
            MouseKey::None => return Err(core_error!("unsupport key")),
            MouseKey::Left => MOUSEEVENTF_LEFTDOWN | MOUSEEVENTF_ABSOLUTE | MOUSEEVENTF_VIRTUALDESK,
            MouseKey::Right => {
                MOUSEEVENTF_RIGHTDOWN | MOUSEEVENTF_ABSOLUTE | MOUSEEVENTF_VIRTUALDESK
            }
            MouseKey::Wheel => {
                MOUSEEVENTF_MIDDLEDOWN | MOUSEEVENTF_ABSOLUTE | MOUSEEVENTF_VIRTUALDESK
            }
            MouseKey::SideForward => {
                MOUSEEVENTF_XUP | MOUSEEVENTF_ABSOLUTE | MOUSEEVENTF_VIRTUALDESK
            }
            MouseKey::SideBack => {
                MOUSEEVENTF_XDOWN | MOUSEEVENTF_ABSOLUTE | MOUSEEVENTF_VIRTUALDESK
            }
        };

        let up_flags = match key {
            MouseKey::None => return Err(core_error!("unsupport key")),
            MouseKey::Left => MOUSEEVENTF_LEFTUP | MOUSEEVENTF_ABSOLUTE | MOUSEEVENTF_VIRTUALDESK,
            MouseKey::Right => MOUSEEVENTF_RIGHTUP | MOUSEEVENTF_ABSOLUTE | MOUSEEVENTF_VIRTUALDESK,
            MouseKey::Wheel => {
                MOUSEEVENTF_MIDDLEUP | MOUSEEVENTF_ABSOLUTE | MOUSEEVENTF_VIRTUALDESK
            }
            MouseKey::SideForward => {
                MOUSEEVENTF_XUP | MOUSEEVENTF_ABSOLUTE | MOUSEEVENTF_VIRTUALDESK
            }
            MouseKey::SideBack => {
                MOUSEEVENTF_XDOWN | MOUSEEVENTF_ABSOLUTE | MOUSEEVENTF_VIRTUALDESK
            }
        };

        let mouse_data = if MouseKey::SideForward == *key {
            VK_XBUTTON1.0 as i32
        } else if MouseKey::SideBack == *key {
            VK_XBUTTON2.0 as i32
        } else {
            0
        };

        args.push((mouse_data, down_flags));
        args.push((mouse_data, up_flags));
    }

    unsafe { send_input(&args, monitor.left, monitor.top, x, y) }
}

pub fn mouse_move(monitor: &Monitor, key: &MouseKey, x: f32, y: f32) -> CoreResult<()> {
    let dw_flags = match key {
        MouseKey::None => MOUSEEVENTF_MOVE | MOUSEEVENTF_ABSOLUTE | MOUSEEVENTF_VIRTUALDESK,
        MouseKey::Left => {
            MOUSEEVENTF_LEFTDOWN | MOUSEEVENTF_MOVE | MOUSEEVENTF_ABSOLUTE | MOUSEEVENTF_VIRTUALDESK
        }
        MouseKey::Right => {
            MOUSEEVENTF_RIGHTDOWN
                | MOUSEEVENTF_MOVE
                | MOUSEEVENTF_ABSOLUTE
                | MOUSEEVENTF_VIRTUALDESK
        }
        MouseKey::Wheel => {
            MOUSEEVENTF_MIDDLEDOWN
                | MOUSEEVENTF_MOVE
                | MOUSEEVENTF_ABSOLUTE
                | MOUSEEVENTF_VIRTUALDESK
        }
        MouseKey::SideForward => {
            MOUSEEVENTF_XUP | MOUSEEVENTF_MOVE | MOUSEEVENTF_ABSOLUTE | MOUSEEVENTF_VIRTUALDESK
        }
        MouseKey::SideBack => {
            MOUSEEVENTF_XDOWN | MOUSEEVENTF_MOVE | MOUSEEVENTF_ABSOLUTE | MOUSEEVENTF_VIRTUALDESK
        }
    };

    let mouse_data = if MouseKey::SideForward == *key {
        VK_XBUTTON1.0 as i32
    } else if MouseKey::SideBack == *key {
        VK_XBUTTON2.0 as i32
    } else {
        0
    };

    unsafe { send_input(&[(mouse_data, dw_flags)], monitor.left, monitor.top, x, y) }
}

pub fn mouse_scroll_wheel(monitor: &Monitor, delta: f32) -> CoreResult<()> {
    unsafe {
        send_input(
            &[(delta.round() as i32, MOUSEEVENTF_WHEEL)],
            monitor.left,
            monitor.top,
            0f32,
            0f32,
        )
    }
}

pub fn keyboard_up(key: &tao::keyboard::KeyCode) -> CoreResult<()> {
    unsafe { post_keyboard_event(key, false) }
}

pub fn keyboard_down(key: &tao::keyboard::KeyCode) -> CoreResult<()> {
    unsafe { post_keyboard_event(key, true) }
}

unsafe fn send_input(
    args: &[(i32, MOUSE_EVENT_FLAGS)],
    left: u16,
    top: u16,
    screen_coordinate_x: f32,
    screen_coordinate_y: f32,
) -> CoreResult<()> {
    let dx = ((left as f32 + screen_coordinate_x) * 65535f32
        / GetSystemMetrics(SM_CXVIRTUALSCREEN) as f32)
        .round() as i32;

    let dy = ((top as f32 + screen_coordinate_y) * 65535f32
        / GetSystemMetrics(SM_CYVIRTUALSCREEN) as f32)
        .round() as i32;

    let mut inputs = Vec::with_capacity(args.len());

    for (mouse_data, dw_flags) in args {
        inputs.push(INPUT {
            r#type: INPUT_MOUSE,
            Anonymous: windows::Win32::UI::Input::KeyboardAndMouse::INPUT_0 {
                mi: MOUSEINPUT {
                    dx,
                    dy,
                    mouseData: *mouse_data,
                    dwFlags: *dw_flags,
                    ..Default::default()
                },
            },
        });
    }

    inputs.set_len(args.len());

    if SendInput(&inputs, std::mem::size_of::<INPUT>() as i32) as usize == inputs.len() {
        Ok(())
    } else {
        Err(core_error!(
            "SendInput failed ({:?})",
            GetLastError().to_hresult()
        ))
    }
}

unsafe fn post_keyboard_event(key: &tao::keyboard::KeyCode, press: bool) -> CoreResult<()> {
    if let Some(vk_key) = map_key_code(key) {
        let mut flags: KEYBD_EVENT_FLAGS = KEYBD_EVENT_FLAGS(0);
        if is_extend_key(vk_key) {
            flags |= KEYEVENTF_EXTENDEDKEY;
        }

        if !press {
            flags |= KEYEVENTF_KEYUP;
        }

        let inputs = [INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: vk_key,
                    wScan: (MapVirtualKeyW(vk_key.0 as u32, MAPVK_VK_TO_VSC) & 0xFF) as u16,
                    dwFlags: flags,
                    ..Default::default()
                },
            },
        }];

        if SendInput(&inputs, std::mem::size_of::<INPUT>() as i32) as usize == inputs.len() {
            Ok(())
        } else {
            Err(core_error!(
                "SendInput failed ({:?})",
                GetLastError().to_hresult()
            ))
        }
    } else {
        Ok(())
    }
}

const fn is_extend_key(key: VIRTUAL_KEY) -> bool {
    matches!(
        key,
        VK_MENU
            | VK_LMENU
            | VK_RMENU
            | VK_CONTROL
            | VK_LCONTROL
            | VK_RCONTROL
            | VK_INSERT
            | VK_DELETE
            | VK_HOME
            | VK_END
            | VK_PRIOR
            | VK_NEXT
            | VK_LEFT
            | VK_RIGHT
            | VK_UP
            | VK_DOWN
            | VK_NUMLOCK
            | VK_CANCEL
            | VK_SNAPSHOT
            | VK_DIVIDE
    )
}

const fn map_key_code(key: &tao::keyboard::KeyCode) -> Option<VIRTUAL_KEY> {
    match key {
        tao::keyboard::KeyCode::Unidentified(_) => None,
        tao::keyboard::KeyCode::Backquote => Some(VK_OEM_3),
        tao::keyboard::KeyCode::Backslash => Some(VK_OEM_5),
        tao::keyboard::KeyCode::BracketLeft => Some(VK_OEM_4),
        tao::keyboard::KeyCode::BracketRight => Some(VK_OEM_6),
        tao::keyboard::KeyCode::Comma => Some(VK_OEM_COMMA),
        tao::keyboard::KeyCode::Digit0 => Some(VK_0),
        tao::keyboard::KeyCode::Digit1 => Some(VK_1),
        tao::keyboard::KeyCode::Digit2 => Some(VK_2),
        tao::keyboard::KeyCode::Digit3 => Some(VK_3),
        tao::keyboard::KeyCode::Digit4 => Some(VK_4),
        tao::keyboard::KeyCode::Digit5 => Some(VK_5),
        tao::keyboard::KeyCode::Digit6 => Some(VK_6),
        tao::keyboard::KeyCode::Digit7 => Some(VK_7),
        tao::keyboard::KeyCode::Digit8 => Some(VK_8),
        tao::keyboard::KeyCode::Digit9 => Some(VK_9),
        tao::keyboard::KeyCode::Equal => Some(VK_OEM_PLUS),
        tao::keyboard::KeyCode::IntlBackslash => Some(VK_OEM_5),
        tao::keyboard::KeyCode::IntlRo => None,
        tao::keyboard::KeyCode::IntlYen => None,
        tao::keyboard::KeyCode::KeyA => Some(VK_A),
        tao::keyboard::KeyCode::KeyB => Some(VK_B),
        tao::keyboard::KeyCode::KeyC => Some(VK_C),
        tao::keyboard::KeyCode::KeyD => Some(VK_D),
        tao::keyboard::KeyCode::KeyE => Some(VK_E),
        tao::keyboard::KeyCode::KeyF => Some(VK_F),
        tao::keyboard::KeyCode::KeyG => Some(VK_G),
        tao::keyboard::KeyCode::KeyH => Some(VK_H),
        tao::keyboard::KeyCode::KeyI => Some(VK_I),
        tao::keyboard::KeyCode::KeyJ => Some(VK_J),
        tao::keyboard::KeyCode::KeyK => Some(VK_K),
        tao::keyboard::KeyCode::KeyL => Some(VK_L),
        tao::keyboard::KeyCode::KeyM => Some(VK_M),
        tao::keyboard::KeyCode::KeyN => Some(VK_N),
        tao::keyboard::KeyCode::KeyO => Some(VK_O),
        tao::keyboard::KeyCode::KeyP => Some(VK_P),
        tao::keyboard::KeyCode::KeyQ => Some(VK_Q),
        tao::keyboard::KeyCode::KeyR => Some(VK_R),
        tao::keyboard::KeyCode::KeyS => Some(VK_S),
        tao::keyboard::KeyCode::KeyT => Some(VK_T),
        tao::keyboard::KeyCode::KeyU => Some(VK_U),
        tao::keyboard::KeyCode::KeyV => Some(VK_V),
        tao::keyboard::KeyCode::KeyW => Some(VK_W),
        tao::keyboard::KeyCode::KeyX => Some(VK_X),
        tao::keyboard::KeyCode::KeyY => Some(VK_Y),
        tao::keyboard::KeyCode::KeyZ => Some(VK_Z),
        tao::keyboard::KeyCode::Minus => Some(VK_OEM_MINUS),
        tao::keyboard::KeyCode::Plus => Some(VK_OEM_PLUS),
        tao::keyboard::KeyCode::Period => Some(VK_OEM_PERIOD),
        tao::keyboard::KeyCode::Quote => Some(VK_OEM_7),
        tao::keyboard::KeyCode::Semicolon => Some(VK_OEM_1),
        tao::keyboard::KeyCode::Slash => Some(VK_OEM_2),
        tao::keyboard::KeyCode::AltLeft => Some(VK_LMENU),
        tao::keyboard::KeyCode::AltRight => Some(VK_RMENU),
        tao::keyboard::KeyCode::Backspace => Some(VK_BACK),
        tao::keyboard::KeyCode::CapsLock => Some(VK_CAPITAL),
        tao::keyboard::KeyCode::ContextMenu => Some(VK_MENU),
        tao::keyboard::KeyCode::ControlLeft => Some(VK_LCONTROL),
        tao::keyboard::KeyCode::ControlRight => Some(VK_RCONTROL),
        tao::keyboard::KeyCode::Enter => Some(VK_RETURN),
        tao::keyboard::KeyCode::SuperLeft => Some(VK_LWIN),
        tao::keyboard::KeyCode::SuperRight => Some(VK_RWIN),
        tao::keyboard::KeyCode::ShiftLeft => Some(VK_LSHIFT),
        tao::keyboard::KeyCode::ShiftRight => Some(VK_RSHIFT),
        tao::keyboard::KeyCode::Space => Some(VK_SPACE),
        tao::keyboard::KeyCode::Tab => Some(VK_TAB),
        tao::keyboard::KeyCode::Convert => Some(VK_CONVERT),
        tao::keyboard::KeyCode::KanaMode => Some(VK_KANA),
        tao::keyboard::KeyCode::Lang1 => Some(VK_HANGUL),
        tao::keyboard::KeyCode::Lang2 => Some(VK_HANJA),
        tao::keyboard::KeyCode::Lang3 => None, // todo: https://bsakatu.net/doc/virtual-key-of-windows/
        tao::keyboard::KeyCode::Lang4 => None, // todo: https://bsakatu.net/doc/virtual-key-of-windows/
        tao::keyboard::KeyCode::Lang5 => None, // todo: https://bsakatu.net/doc/virtual-key-of-windows/
        tao::keyboard::KeyCode::NonConvert => Some(VK_NONCONVERT),
        tao::keyboard::KeyCode::Delete => Some(VK_DELETE),
        tao::keyboard::KeyCode::End => Some(VK_END),
        tao::keyboard::KeyCode::Help => Some(VK_HELP),
        tao::keyboard::KeyCode::Home => Some(VK_HOME),
        tao::keyboard::KeyCode::Insert => Some(VK_INSERT),
        tao::keyboard::KeyCode::PageDown => Some(VK_NEXT),
        tao::keyboard::KeyCode::PageUp => Some(VK_PRIOR),
        tao::keyboard::KeyCode::ArrowDown => Some(VK_DOWN),
        tao::keyboard::KeyCode::ArrowLeft => Some(VK_LEFT),
        tao::keyboard::KeyCode::ArrowRight => Some(VK_RIGHT),
        tao::keyboard::KeyCode::ArrowUp => Some(VK_UP),
        tao::keyboard::KeyCode::NumLock => Some(VK_NUMLOCK),
        tao::keyboard::KeyCode::Numpad0 => Some(VK_NUMPAD0),
        tao::keyboard::KeyCode::Numpad1 => Some(VK_NUMPAD1),
        tao::keyboard::KeyCode::Numpad2 => Some(VK_NUMPAD2),
        tao::keyboard::KeyCode::Numpad3 => Some(VK_NUMPAD3),
        tao::keyboard::KeyCode::Numpad4 => Some(VK_NUMPAD4),
        tao::keyboard::KeyCode::Numpad5 => Some(VK_NUMPAD5),
        tao::keyboard::KeyCode::Numpad6 => Some(VK_NUMPAD6),
        tao::keyboard::KeyCode::Numpad7 => Some(VK_NUMPAD7),
        tao::keyboard::KeyCode::Numpad8 => Some(VK_NUMPAD8),
        tao::keyboard::KeyCode::Numpad9 => Some(VK_NUMPAD9),
        tao::keyboard::KeyCode::NumpadAdd => Some(VK_ADD),
        tao::keyboard::KeyCode::NumpadBackspace => Some(VK_BACK),
        tao::keyboard::KeyCode::NumpadClear => Some(VK_CLEAR),
        tao::keyboard::KeyCode::NumpadClearEntry => None,
        tao::keyboard::KeyCode::NumpadComma => Some(VK_SEPARATOR),
        tao::keyboard::KeyCode::NumpadDecimal => Some(VK_DECIMAL),
        tao::keyboard::KeyCode::NumpadDivide => Some(VK_DIVIDE),
        tao::keyboard::KeyCode::NumpadEnter => Some(VK_RETURN),
        tao::keyboard::KeyCode::NumpadEqual => Some(VK_OEM_NEC_EQUAL),
        tao::keyboard::KeyCode::NumpadHash => None,
        tao::keyboard::KeyCode::NumpadMemoryAdd => None,
        tao::keyboard::KeyCode::NumpadMemoryClear => None,
        tao::keyboard::KeyCode::NumpadMemoryRecall => None,
        tao::keyboard::KeyCode::NumpadMemoryStore => None,
        tao::keyboard::KeyCode::NumpadMemorySubtract => None,
        tao::keyboard::KeyCode::NumpadMultiply => Some(VK_MULTIPLY),
        tao::keyboard::KeyCode::NumpadParenLeft => None,
        tao::keyboard::KeyCode::NumpadParenRight => None,
        tao::keyboard::KeyCode::NumpadStar => Some(VK_MULTIPLY),
        tao::keyboard::KeyCode::NumpadSubtract => Some(VK_SUBTRACT),
        tao::keyboard::KeyCode::Escape => Some(VK_ESCAPE),
        tao::keyboard::KeyCode::Fn => None,
        tao::keyboard::KeyCode::FnLock => None,
        tao::keyboard::KeyCode::PrintScreen => Some(VK_PRINT),
        tao::keyboard::KeyCode::ScrollLock => Some(VK_SCROLL),
        tao::keyboard::KeyCode::Pause => Some(VK_PAUSE),
        tao::keyboard::KeyCode::BrowserBack => Some(VK_BROWSER_BACK),
        tao::keyboard::KeyCode::BrowserFavorites => Some(VK_BROWSER_FAVORITES),
        tao::keyboard::KeyCode::BrowserForward => Some(VK_BROWSER_FORWARD),
        tao::keyboard::KeyCode::BrowserHome => Some(VK_BROWSER_HOME),
        tao::keyboard::KeyCode::BrowserRefresh => Some(VK_BROWSER_REFRESH),
        tao::keyboard::KeyCode::BrowserSearch => Some(VK_BROWSER_SEARCH),
        tao::keyboard::KeyCode::BrowserStop => Some(VK_BROWSER_STOP),
        tao::keyboard::KeyCode::Eject => None,
        tao::keyboard::KeyCode::LaunchApp1 => Some(VK_LAUNCH_APP1),
        tao::keyboard::KeyCode::LaunchApp2 => Some(VK_LAUNCH_APP2),
        tao::keyboard::KeyCode::LaunchMail => Some(VK_LAUNCH_MAIL),
        tao::keyboard::KeyCode::MediaPlayPause => Some(VK_MEDIA_PLAY_PAUSE),
        tao::keyboard::KeyCode::MediaSelect => Some(VK_LAUNCH_MEDIA_SELECT),
        tao::keyboard::KeyCode::MediaStop => Some(VK_MEDIA_STOP),
        tao::keyboard::KeyCode::MediaTrackNext => Some(VK_MEDIA_NEXT_TRACK),
        tao::keyboard::KeyCode::MediaTrackPrevious => Some(VK_MEDIA_PREV_TRACK),
        tao::keyboard::KeyCode::Power => None,
        tao::keyboard::KeyCode::Sleep => Some(VK_SLEEP),
        tao::keyboard::KeyCode::AudioVolumeDown => Some(VK_VOLUME_DOWN),
        tao::keyboard::KeyCode::AudioVolumeMute => Some(VK_VOLUME_MUTE),
        tao::keyboard::KeyCode::AudioVolumeUp => Some(VK_VOLUME_UP),
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
        tao::keyboard::KeyCode::F1 => Some(VK_F1),
        tao::keyboard::KeyCode::F2 => Some(VK_F2),
        tao::keyboard::KeyCode::F3 => Some(VK_F3),
        tao::keyboard::KeyCode::F4 => Some(VK_F4),
        tao::keyboard::KeyCode::F5 => Some(VK_F5),
        tao::keyboard::KeyCode::F6 => Some(VK_F6),
        tao::keyboard::KeyCode::F7 => Some(VK_F7),
        tao::keyboard::KeyCode::F8 => Some(VK_F8),
        tao::keyboard::KeyCode::F9 => Some(VK_F9),
        tao::keyboard::KeyCode::F10 => Some(VK_F10),
        tao::keyboard::KeyCode::F11 => Some(VK_F11),
        tao::keyboard::KeyCode::F12 => Some(VK_F12),
        tao::keyboard::KeyCode::F13 => Some(VK_F13),
        tao::keyboard::KeyCode::F14 => Some(VK_F14),
        tao::keyboard::KeyCode::F15 => Some(VK_F15),
        tao::keyboard::KeyCode::F16 => Some(VK_F16),
        tao::keyboard::KeyCode::F17 => Some(VK_F17),
        tao::keyboard::KeyCode::F18 => Some(VK_F18),
        tao::keyboard::KeyCode::F19 => Some(VK_F19),
        tao::keyboard::KeyCode::F20 => Some(VK_F20),
        tao::keyboard::KeyCode::F21 => Some(VK_F21),
        tao::keyboard::KeyCode::F22 => Some(VK_F22),
        tao::keyboard::KeyCode::F23 => Some(VK_F23),
        tao::keyboard::KeyCode::F24 => Some(VK_F24),
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
