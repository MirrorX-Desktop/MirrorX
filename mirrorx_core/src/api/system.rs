pub fn set_show_cursor(show: bool) {
    #[cfg(target_os = "windows")]
    unsafe {
        let _ = windows::Win32::UI::WindowsAndMessaging::ShowCursor::<bool>(show);
    }

    #[cfg(target_os = "macos")]
    unsafe {
        use core_graphics::display::{CGDisplayHideCursor, CGDisplayShowCursor};

        if show {
            CGDisplayShowCursor(0);
        } else {
            CGDisplayHideCursor(0);
        }
    }
}
