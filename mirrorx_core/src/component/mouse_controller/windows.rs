use crate::{error::MirrorXError, service::endpoint::message::MouseKey};
use windows::Win32::UI::Input::KeyboardAndMouse::*;

pub fn mouse_up(key: MouseKey, position: (f32, f32)) -> Result<(), MirrorXError> {
    let dw_flags = match key {
        MouseKey::None => return Err(MirrorXError::Other(anyhow::anyhow!("unsupport key"))),
        MouseKey::Left => MOUSEEVENTF_LEFTUP,
        MouseKey::Right => MOUSEEVENTF_RIGHTUP,
        MouseKey::Wheel => MOUSEEVENTF_MIDDLEUP,
    };

    let inputs = [INPUT {
        r#type: INPUT_MOUSE,
        Anonymous: windows::Win32::UI::Input::KeyboardAndMouse::INPUT_0 {
            mi: MOUSEINPUT {
                dx: position.0.round() as i32,
                dy: position.1.round() as i32,
                mouseData: 0,
                dwFlags: dw_flags,
                time: 0,
                dwExtraInfo: 0,
            },
        },
    }];

    unsafe {
        if SendInput(&inputs, inputs.len() as i32) > 0 {
            Ok(())
        } else {
            Err(MirrorXError::Other(anyhow::anyhow!("SendInput failed")))
        }
    }
}

pub fn mouse_down(key: MouseKey, position: (f32, f32)) -> Result<(), MirrorXError> {
    let dw_flags = match key {
        MouseKey::None => return Err(MirrorXError::Other(anyhow::anyhow!("unsupport key"))),
        MouseKey::Left => MOUSEEVENTF_LEFTDOWN,
        MouseKey::Right => MOUSEEVENTF_RIGHTDOWN,
        MouseKey::Wheel => MOUSEEVENTF_MIDDLEDOWN,
    };

    let inputs = [INPUT {
        r#type: INPUT_MOUSE,
        Anonymous: windows::Win32::UI::Input::KeyboardAndMouse::INPUT_0 {
            mi: MOUSEINPUT {
                dx: position.0.round() as i32,
                dy: position.1.round() as i32,
                mouseData: 0,
                dwFlags: dw_flags,
                time: 0,
                dwExtraInfo: 0,
            },
        },
    }];

    unsafe {
        if SendInput(&inputs, inputs.len() as i32) > 0 {
            Ok(())
        } else {
            Err(MirrorXError::Other(anyhow::anyhow!("SendInput failed")))
        }
    }
}

pub fn mouse_move(
    display_id: &str,
    key: MouseKey,
    position: (f32, f32),
) -> Result<(), MirrorXError> {
    let dw_flags = match key {
        MouseKey::None => MOUSEEVENTF_MOVE,
        MouseKey::Left => MOUSEEVENTF_LEFTDOWN | MOUSEEVENTF_MOVE,
        MouseKey::Right => MOUSEEVENTF_RIGHTDOWN | MOUSEEVENTF_MOVE,
        MouseKey::Wheel => MOUSEEVENTF_MIDDLEDOWN | MOUSEEVENTF_MOVE,
    };

    let inputs = [INPUT {
        r#type: INPUT_MOUSE,
        Anonymous: windows::Win32::UI::Input::KeyboardAndMouse::INPUT_0 {
            mi: MOUSEINPUT {
                dx: position.0.round() as i32,
                dy: position.1.round() as i32,
                mouseData: 0,
                dwFlags: dw_flags,
                time: 0,
                dwExtraInfo: 0,
            },
        },
    }];

    unsafe {
        if SendInput(&inputs, inputs.len() as i32) > 0 {
            Ok(())
        } else {
            Err(MirrorXError::Other(anyhow::anyhow!("SendInput failed")))
        }
    }
}

pub fn mouse_scroll_whell(delta: f32, position: (f32, f32)) -> Result<(), MirrorXError> {
    let inputs = [INPUT {
        r#type: INPUT_MOUSE,
        Anonymous: windows::Win32::UI::Input::KeyboardAndMouse::INPUT_0 {
            mi: MOUSEINPUT {
                dx: position.0.round() as i32,
                dy: position.1.round() as i32,
                mouseData: delta.round() as i32,
                dwFlags: MOUSEEVENTF_WHEEL,
                time: 0,
                dwExtraInfo: 0,
            },
        },
    }];

    unsafe {
        if SendInput(&inputs, inputs.len() as i32) > 0 {
            Ok(())
        } else {
            Err(MirrorXError::Other(anyhow::anyhow!("SendInput failed")))
        }
    }
}
