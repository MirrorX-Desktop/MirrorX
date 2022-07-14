use crate::{
    component::monitor::Monitor, error::MirrorXError, service::endpoint::message::MouseKey,
};
use windows::Win32::{
    Foundation::GetLastError,
    UI::{Input::KeyboardAndMouse::*, WindowsAndMessaging::*},
};

pub fn mouse_up(
    monitor: &Monitor,
    key: MouseKey,
    position: (f32, f32),
) -> Result<(), MirrorXError> {
    let dw_flags = match key {
        MouseKey::None => return Err(MirrorXError::Other(anyhow::anyhow!("unsupport key"))),
        MouseKey::Left => MOUSEEVENTF_LEFTUP | MOUSEEVENTF_ABSOLUTE | MOUSEEVENTF_VIRTUALDESK,
        MouseKey::Right => MOUSEEVENTF_RIGHTUP | MOUSEEVENTF_ABSOLUTE | MOUSEEVENTF_VIRTUALDESK,
        MouseKey::Wheel => MOUSEEVENTF_MIDDLEUP | MOUSEEVENTF_ABSOLUTE | MOUSEEVENTF_VIRTUALDESK,
    };

    unsafe {
        send_input(
            &[(0, dw_flags)],
            monitor.left,
            monitor.top,
            position.0,
            position.1,
        )
    }
}

pub fn mouse_down(
    monitor: &Monitor,
    key: MouseKey,
    position: (f32, f32),
) -> Result<(), MirrorXError> {
    let dw_flags = match key {
        MouseKey::None => return Err(MirrorXError::Other(anyhow::anyhow!("unsupport key"))),
        MouseKey::Left => MOUSEEVENTF_LEFTDOWN | MOUSEEVENTF_ABSOLUTE | MOUSEEVENTF_VIRTUALDESK,
        MouseKey::Right => MOUSEEVENTF_RIGHTDOWN | MOUSEEVENTF_ABSOLUTE | MOUSEEVENTF_VIRTUALDESK,
        MouseKey::Wheel => MOUSEEVENTF_MIDDLEDOWN | MOUSEEVENTF_ABSOLUTE | MOUSEEVENTF_VIRTUALDESK,
    };

    unsafe {
        send_input(
            &[(0, dw_flags)],
            monitor.left,
            monitor.top,
            position.0,
            position.1,
        )
    }
}

pub fn mouse_move(
    monitor: &Monitor,
    key: MouseKey,
    position: (f32, f32),
) -> Result<(), MirrorXError> {
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
    };

    unsafe {
        send_input(
            &[(0, dw_flags)],
            monitor.left,
            monitor.top,
            position.0,
            position.1,
        )
    }
}

pub fn mouse_scroll_whell(
    monitor: &Monitor,
    delta: f32,
    position: (f32, f32),
) -> Result<(), MirrorXError> {
    unsafe {
        send_input(
            &[(delta.round() as i32, MOUSEEVENTF_WHEEL)],
            monitor.left,
            monitor.top,
            position.0,
            position.1,
        )
    }
}

unsafe fn send_input(
    args: &[(i32, MOUSE_EVENT_FLAGS)],
    left: u16,
    top: u16,
    screen_coordinate_x: f32,
    screen_coordinate_y: f32,
) -> Result<(), MirrorXError> {
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

    if SendInput(&inputs, std::mem::size_of::<INPUT>() as i32) as usize != inputs.len() {
        Ok(())
    } else {
        Err(MirrorXError::Other(anyhow::anyhow!(
            "SendInput failed ({:?})",
            GetLastError().to_hresult()
        )))
    }
}
