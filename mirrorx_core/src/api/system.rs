pub fn set_show_cursor(show: bool) {
    #[cfg(target_os = "windows")]
    unsafe {
        let _ = windows::Win32::UI::WindowsAndMessaging::ShowCursor::<bool>(show);
    }
}
