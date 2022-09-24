use super::key::{KeyboardKey, MouseKey};
use crate::{
    component::desktop::monitor::Monitor,
    core_error,
    error::{CoreError, CoreResult},
};
use windows::Win32::{
    Foundation::GetLastError,
    UI::{Input::KeyboardAndMouse::*, WindowsAndMessaging::*},
};

pub fn mouse_up(monitor: &Monitor, key: MouseKey, x: f32, y: f32) -> CoreResult<()> {
    let dw_flags = match key {
        MouseKey::None => return Err(core_error!("unsupport key")),
        MouseKey::Left => MOUSEEVENTF_LEFTUP | MOUSEEVENTF_ABSOLUTE | MOUSEEVENTF_VIRTUALDESK,
        MouseKey::Right => MOUSEEVENTF_RIGHTUP | MOUSEEVENTF_ABSOLUTE | MOUSEEVENTF_VIRTUALDESK,
        MouseKey::Wheel => MOUSEEVENTF_MIDDLEUP | MOUSEEVENTF_ABSOLUTE | MOUSEEVENTF_VIRTUALDESK,
        MouseKey::SideForward => MOUSEEVENTF_XUP | MOUSEEVENTF_ABSOLUTE | MOUSEEVENTF_VIRTUALDESK,
        MouseKey::SideBack => MOUSEEVENTF_XDOWN | MOUSEEVENTF_ABSOLUTE | MOUSEEVENTF_VIRTUALDESK,
    };

    let mouse_data = if MouseKey::SideForward == key {
        VK_XBUTTON1.0 as i32
    } else if MouseKey::SideBack == key {
        VK_XBUTTON2.0 as i32
    } else {
        0
    };

    unsafe { send_input(&[(mouse_data, dw_flags)], monitor.left, monitor.top, x, y) }
}

pub fn mouse_down(monitor: &Monitor, key: MouseKey, x: f32, y: f32) -> CoreResult<()> {
    let dw_flags = match key {
        MouseKey::None => return Err(core_error!("unsupport key")),
        MouseKey::Left => MOUSEEVENTF_LEFTDOWN | MOUSEEVENTF_ABSOLUTE | MOUSEEVENTF_VIRTUALDESK,
        MouseKey::Right => MOUSEEVENTF_RIGHTDOWN | MOUSEEVENTF_ABSOLUTE | MOUSEEVENTF_VIRTUALDESK,
        MouseKey::Wheel => MOUSEEVENTF_MIDDLEDOWN | MOUSEEVENTF_ABSOLUTE | MOUSEEVENTF_VIRTUALDESK,
        MouseKey::SideForward => MOUSEEVENTF_XUP | MOUSEEVENTF_ABSOLUTE | MOUSEEVENTF_VIRTUALDESK,
        MouseKey::SideBack => MOUSEEVENTF_XDOWN | MOUSEEVENTF_ABSOLUTE | MOUSEEVENTF_VIRTUALDESK,
    };

    let mouse_data = if MouseKey::SideForward == key {
        VK_XBUTTON1.0 as i32
    } else if MouseKey::SideBack == key {
        VK_XBUTTON2.0 as i32
    } else {
        0
    };

    unsafe { send_input(&[(mouse_data, dw_flags)], monitor.left, monitor.top, x, y) }
}

pub fn mouse_move(monitor: &Monitor, key: MouseKey, x: f32, y: f32) -> CoreResult<()> {
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

    let mouse_data = if MouseKey::SideForward == key {
        VK_XBUTTON1.0 as i32
    } else if MouseKey::SideBack == key {
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

pub fn keyboard_up(key: KeyboardKey) -> CoreResult<()> {
    unsafe { post_keyboard_event(key, false) }
}

pub fn keyboard_down(key: KeyboardKey) -> CoreResult<()> {
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

unsafe fn post_keyboard_event(key: KeyboardKey, press: bool) -> CoreResult<()> {
    let vk_key = map_key_code(key);

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

const fn map_key_code(key: KeyboardKey) -> VIRTUAL_KEY {
    match key {
        KeyboardKey::A => VK_A,
        KeyboardKey::B => VK_B,
        KeyboardKey::C => VK_C,
        KeyboardKey::D => VK_D,
        KeyboardKey::E => VK_E,
        KeyboardKey::F => VK_F,
        KeyboardKey::G => VK_G,
        KeyboardKey::H => VK_H,
        KeyboardKey::I => VK_I,
        KeyboardKey::J => VK_J,
        KeyboardKey::K => VK_K,
        KeyboardKey::L => VK_L,
        KeyboardKey::M => VK_M,
        KeyboardKey::N => VK_N,
        KeyboardKey::O => VK_O,
        KeyboardKey::P => VK_P,
        KeyboardKey::Q => VK_Q,
        KeyboardKey::R => VK_R,
        KeyboardKey::S => VK_S,
        KeyboardKey::T => VK_T,
        KeyboardKey::U => VK_U,
        KeyboardKey::V => VK_V,
        KeyboardKey::W => VK_W,
        KeyboardKey::X => VK_X,
        KeyboardKey::Y => VK_Y,
        KeyboardKey::Z => VK_Z,
        KeyboardKey::BackQuote => VK_OEM_3,
        KeyboardKey::Digit0 => VK_0,
        KeyboardKey::Digit1 => VK_1,
        KeyboardKey::Digit2 => VK_2,
        KeyboardKey::Digit3 => VK_3,
        KeyboardKey::Digit4 => VK_4,
        KeyboardKey::Digit5 => VK_5,
        KeyboardKey::Digit6 => VK_6,
        KeyboardKey::Digit7 => VK_7,
        KeyboardKey::Digit8 => VK_8,
        KeyboardKey::Digit9 => VK_9,
        KeyboardKey::Minus => VK_OEM_MINUS,
        KeyboardKey::Equal => VK_OEM_PLUS,
        KeyboardKey::Tab => VK_TAB,
        KeyboardKey::CapsLock => VK_CAPITAL,
        KeyboardKey::LeftShift => VK_LSHIFT,
        KeyboardKey::LeftControl => VK_LCONTROL,
        KeyboardKey::LeftAlt => VK_LMENU,
        KeyboardKey::LeftMeta => VK_LWIN,
        KeyboardKey::Space => VK_SPACE,
        KeyboardKey::RightMeta => VK_RWIN,
        KeyboardKey::RightControl => VK_RCONTROL,
        KeyboardKey::RightAlt => VK_RMENU,
        KeyboardKey::RightShift => VK_RSHIFT,
        KeyboardKey::Comma => VK_OEM_COMMA,
        KeyboardKey::Period => VK_OEM_PERIOD,
        KeyboardKey::Slash => VK_OEM_2,
        KeyboardKey::Semicolon => VK_OEM_1,
        KeyboardKey::QuoteSingle => VK_OEM_7,
        KeyboardKey::Enter => VK_RETURN,
        KeyboardKey::BracketLeft => VK_OEM_4,
        KeyboardKey::BracketRight => VK_OEM_6,
        KeyboardKey::BackSlash => VK_OEM_5,
        KeyboardKey::Backspace => VK_BACK,
        KeyboardKey::NumLock => VK_NUMLOCK,
        KeyboardKey::NumpadEquals => VK_OEM_NEC_EQUAL, // todo: check it
        KeyboardKey::NumpadDivide => VK_DIVIDE,
        KeyboardKey::NumpadMultiply => VK_MULTIPLY,
        KeyboardKey::NumpadSubtract => VK_SUBTRACT,
        KeyboardKey::NumpadAdd => VK_ADD,
        KeyboardKey::NumpadEnter => VK_RETURN, // Windows doesn't have NumPad Enter
        KeyboardKey::Numpad0 => VK_NUMPAD0,
        KeyboardKey::Numpad1 => VK_NUMPAD1,
        KeyboardKey::Numpad2 => VK_NUMPAD2,
        KeyboardKey::Numpad3 => VK_NUMPAD3,
        KeyboardKey::Numpad4 => VK_NUMPAD4,
        KeyboardKey::Numpad5 => VK_NUMPAD5,
        KeyboardKey::Numpad6 => VK_NUMPAD6,
        KeyboardKey::Numpad7 => VK_NUMPAD7,
        KeyboardKey::Numpad8 => VK_NUMPAD8,
        KeyboardKey::Numpad9 => VK_NUMPAD9,
        KeyboardKey::NumpadDecimal => VK_DECIMAL,
        KeyboardKey::ArrowLeft => VK_LEFT,
        KeyboardKey::ArrowUp => VK_UP,
        KeyboardKey::ArrowRight => VK_RIGHT,
        KeyboardKey::ArrowDown => VK_DOWN,
        KeyboardKey::Escape => VK_ESCAPE,
        KeyboardKey::PrintScreen => VK_SNAPSHOT,
        KeyboardKey::ScrollLock => VK_SCROLL,
        KeyboardKey::Pause => VK_PAUSE,
        KeyboardKey::Insert => VK_INSERT,
        KeyboardKey::Delete => VK_DELETE,
        KeyboardKey::Home => VK_HOME,
        KeyboardKey::End => VK_END,
        KeyboardKey::PageUp => VK_PRIOR,
        KeyboardKey::PageDown => VK_NEXT,
        KeyboardKey::F1 => VK_F1,
        KeyboardKey::F2 => VK_F2,
        KeyboardKey::F3 => VK_F3,
        KeyboardKey::F4 => VK_F4,
        KeyboardKey::F5 => VK_F5,
        KeyboardKey::F6 => VK_F6,
        KeyboardKey::F7 => VK_F7,
        KeyboardKey::F8 => VK_F8,
        KeyboardKey::F9 => VK_F9,
        KeyboardKey::F10 => VK_F10,
        KeyboardKey::F11 => VK_F11,
        KeyboardKey::F12 => VK_F12,
        KeyboardKey::Fn => todo!(),
    }
}
