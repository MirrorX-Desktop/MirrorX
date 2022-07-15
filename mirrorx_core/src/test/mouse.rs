#[cfg(target_os = "windows")]
#[test]
fn test_mouse_move() -> anyhow::Result<()> {
    use std::mem::zeroed;
    use windows::Win32::{
        Foundation::GetLastError,
        UI::{Input::KeyboardAndMouse::*, WindowsAndMessaging::GetMessageExtraInfo},
    };

    unsafe {
        let mut inputs: [INPUT; 1] = zeroed();
        inputs[0].r#type = INPUT_MOUSE;
        inputs[0].Anonymous.mi.dx = 1000;
        inputs[0].Anonymous.mi.dy = 1000;
        inputs[0].Anonymous.mi.dwFlags = MOUSEEVENTF_MOVE | MOUSEEVENTF_ABSOLUTE;

        // let inputs = [INPUT {
        //     r#type: INPUT_MOUSE,
        //     Anonymous: windows::Win32::UI::Input::KeyboardAndMouse::INPUT_0 {
        //         mi: MOUSEINPUT {
        //             dx: 100,
        //             dy: 100,
        //             mouseData: 0,
        //             dwFlags: MOUSEEVENTF_MOVE | MOUSEEVENTF_ABSOLUTE | MOUSEEVENTF_VIRTUALDESK,
        //             time: 0,
        //             dwExtraInfo: GetMessageExtraInfo().0 as usize,
        //         },
        //     },
        // }];

        let input_len = inputs.len();

        if SendInput(&inputs, std::mem::size_of::<INPUT>() as i32) == (input_len as u32) {
            Ok(())
        } else {
            Err(anyhow::anyhow!(
                "SendInput failed ({:?})",
                GetLastError().to_hresult()
            ))
        }
    }
}
