use crate::{error::MirrorXError, service::endpoint::message::MouseKey};
use windows::{
    core::HRESULT,
    Win32::{
        Foundation::GetLastError,
        UI::{
            Input::KeyboardAndMouse::*,
            WindowsAndMessaging::{GetSystemMetrics, SM_CXSCREEN, SM_CYSCREEN},
        },
    },
};

pub fn mouse_up(key: MouseKey, position: (f32, f32)) -> Result<(), MirrorXError> {
    let dw_flags = match key {
        MouseKey::None => return Err(MirrorXError::Other(anyhow::anyhow!("unsupport key"))),
        MouseKey::Left => MOUSEEVENTF_LEFTUP | MOUSEEVENTF_ABSOLUTE,
        MouseKey::Right => MOUSEEVENTF_RIGHTUP | MOUSEEVENTF_ABSOLUTE,
        MouseKey::Wheel => MOUSEEVENTF_MIDDLEUP | MOUSEEVENTF_ABSOLUTE,
    };

    unsafe {
        let inputs = [INPUT {
            r#type: INPUT_MOUSE,
            Anonymous: windows::Win32::UI::Input::KeyboardAndMouse::INPUT_0 {
                mi: MOUSEINPUT {
                    dx: (position.0.round() * (65536f32 / GetSystemMetrics(SM_CXSCREEN) as f32))
                        .round() as i32,
                    dy: (position.1.round() * (65536f32 / GetSystemMetrics(SM_CYSCREEN) as f32))
                        .round() as i32,
                    mouseData: 0,
                    dwFlags: dw_flags,
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        }];

        let input_len = inputs.len();

        if SendInput(&inputs, input_len as i32) != (input_len as u32) {
            Ok(())
        } else {
            Err(MirrorXError::Other(anyhow::anyhow!(
                "SendInput failed ({:?})",
                GetLastError().to_hresult()
            )))
        }
    }
}

pub fn mouse_down(key: MouseKey, position: (f32, f32)) -> Result<(), MirrorXError> {
    let dw_flags = match key {
        MouseKey::None => return Err(MirrorXError::Other(anyhow::anyhow!("unsupport key"))),
        MouseKey::Left => MOUSEEVENTF_LEFTDOWN | MOUSEEVENTF_ABSOLUTE,
        MouseKey::Right => MOUSEEVENTF_RIGHTDOWN | MOUSEEVENTF_ABSOLUTE,
        MouseKey::Wheel => MOUSEEVENTF_MIDDLEDOWN | MOUSEEVENTF_ABSOLUTE,
    };

    unsafe {
        let inputs = [INPUT {
            r#type: INPUT_MOUSE,
            Anonymous: windows::Win32::UI::Input::KeyboardAndMouse::INPUT_0 {
                mi: MOUSEINPUT {
                    dx: (position.0.round() * (65536f32 / GetSystemMetrics(SM_CXSCREEN) as f32))
                        .round() as i32,
                    dy: (position.1.round() * (65536f32 / GetSystemMetrics(SM_CYSCREEN) as f32))
                        .round() as i32,
                    mouseData: 0,
                    dwFlags: dw_flags,
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        }];

        let input_len = inputs.len();

        if SendInput(&inputs, input_len as i32) != (input_len as u32) {
            Ok(())
        } else {
            Err(MirrorXError::Other(anyhow::anyhow!(
                "SendInput failed ({:?})",
                GetLastError().to_hresult()
            )))
        }
    }
}

pub fn mouse_move(_: &str, key: MouseKey, position: (f32, f32)) -> Result<(), MirrorXError> {
    let dw_flags = match key {
        MouseKey::None => MOUSEEVENTF_MOVE | MOUSEEVENTF_ABSOLUTE,
        MouseKey::Left => MOUSEEVENTF_LEFTDOWN | MOUSEEVENTF_MOVE | MOUSEEVENTF_ABSOLUTE,
        MouseKey::Right => MOUSEEVENTF_RIGHTDOWN | MOUSEEVENTF_MOVE | MOUSEEVENTF_ABSOLUTE,
        MouseKey::Wheel => MOUSEEVENTF_MIDDLEDOWN | MOUSEEVENTF_MOVE | MOUSEEVENTF_ABSOLUTE,
    };

    unsafe {
        let inputs = [INPUT {
            r#type: INPUT_MOUSE,
            Anonymous: windows::Win32::UI::Input::KeyboardAndMouse::INPUT_0 {
                mi: MOUSEINPUT {
                    dx: (position.0.round() * (65536f32 / GetSystemMetrics(SM_CXSCREEN) as f32))
                        .round() as i32,
                    dy: (position.1.round() * (65536f32 / GetSystemMetrics(SM_CYSCREEN) as f32))
                        .round() as i32,
                    mouseData: 0,
                    dwFlags: dw_flags,
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        }];

        let input_len = inputs.len();

        if SendInput(&inputs, input_len as i32) != (input_len as u32) {
            Ok(())
        } else {
            Err(MirrorXError::Other(anyhow::anyhow!(
                "SendInput failed ({:?})",
                GetLastError().to_hresult()
            )))
        }
    }
}

pub fn mouse_scroll_whell(delta: f32, position: (f32, f32)) -> Result<(), MirrorXError> {
    unsafe {
        let inputs = [INPUT {
            r#type: INPUT_MOUSE,
            Anonymous: windows::Win32::UI::Input::KeyboardAndMouse::INPUT_0 {
                mi: MOUSEINPUT {
                    dx: (position.0.round() * (65536f32 / GetSystemMetrics(SM_CXSCREEN) as f32))
                        .round() as i32,
                    dy: (position.1.round() * (65536f32 / GetSystemMetrics(SM_CYSCREEN) as f32))
                        .round() as i32,
                    mouseData: delta.round() as i32,
                    dwFlags: MOUSEEVENTF_WHEEL,
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        }];

        let input_len = inputs.len();

        if SendInput(&inputs, input_len as i32) != (input_len as u32) {
            Ok(())
        } else {
            Err(MirrorXError::Other(anyhow::anyhow!(
                "SendInput failed ({:?})",
                GetLastError().to_hresult()
            )))
        }
    }
}
