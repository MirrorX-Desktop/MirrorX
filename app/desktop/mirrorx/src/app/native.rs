#[cfg(target_os = "windows")]
pub mod windows {
    pub fn set_window_shadow() {
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

    pub fn set_window_minimize() {
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
}
