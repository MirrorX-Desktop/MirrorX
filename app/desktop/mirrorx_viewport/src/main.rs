slint::include_modules!();

fn main() {
    let ui = App::new().unwrap();

    let ui_handle = ui.as_weak();
    ui.global::<Event>().on_close_button_clicked(move || {
        ui_handle.unwrap().hide();
    });

    ui.global::<Event>().on_min_button_clicked(move || {
        set_window_minimize();
    });

    let ui_handle = ui.as_weak();
    ui.global::<Event>().on_drag_window(move |x, y| {
        if let Err(err) = ui_handle.upgrade_in_event_loop(move |handle| {
            let scale_factor = handle.window().scale_factor();
            let mut pos = handle.window().position();
            pos.x += (x * scale_factor) as i32;
            pos.y += (y * scale_factor) as i32;
            handle.window().set_position(pos);
        }) {
            eprintln!("{err:?}");
        }
    });

    ui.show();

    set_window_shadow();
    ui.run();
}

#[cfg(target_os = "windows")]
fn set_window_shadow() {
    use windows::Win32::Foundation::BOOL;
    use windows::Win32::Foundation::HWND;
    use windows::Win32::Foundation::LPARAM;
    use windows::Win32::Graphics::Dwm::DwmExtendFrameIntoClientArea;
    use windows::Win32::System::Threading::GetCurrentThreadId;
    use windows::Win32::UI::Controls::MARGINS;
    use windows::Win32::UI::WindowsAndMessaging::EnumThreadWindows;

    unsafe {
        unsafe extern "system" fn callback(hwnd: HWND, _: LPARAM) -> BOOL {
            DwmExtendFrameIntoClientArea(
                hwnd,
                &MARGINS {
                    cxLeftWidth: -1,
                    cxRightWidth: -1,
                    cyTopHeight: -1,
                    cyBottomHeight: -1,
                },
            )
            .unwrap();
            true.into()
        }

        EnumThreadWindows(GetCurrentThreadId(), Some(callback), None);
    }
}

#[cfg(target_os = "windows")]
fn set_window_minimize() {
    use windows::Win32::Foundation::BOOL;
    use windows::Win32::Foundation::HWND;
    use windows::Win32::Foundation::LPARAM;
    use windows::Win32::Foundation::WPARAM;
    use windows::Win32::System::Threading::GetCurrentThreadId;
    use windows::Win32::UI::WindowsAndMessaging::EnumThreadWindows;
    use windows::Win32::UI::WindowsAndMessaging::PostMessageW;
    use windows::Win32::UI::WindowsAndMessaging::SC_MINIMIZE;
    use windows::Win32::UI::WindowsAndMessaging::WM_SYSCOMMAND;

    unsafe {
        unsafe extern "system" fn callback(hwnd: HWND, _: LPARAM) -> BOOL {
            PostMessageW(hwnd, WM_SYSCOMMAND, WPARAM(SC_MINIMIZE as usize), LPARAM(0));
            true.into()
        }

        EnumThreadWindows(GetCurrentThreadId(), Some(callback), None);
    }
}
