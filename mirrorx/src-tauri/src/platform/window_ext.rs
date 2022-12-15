use tauri::{Runtime, Window};

pub trait WindowExt {
    #[cfg(target_os = "macos")]
    fn expand_title_bar(&self);
}

impl<R: Runtime> WindowExt for Window<R> {
    #[cfg(target_os = "macos")]
    fn expand_title_bar(&self) {
        use cocoa::appkit::NSWindow;
        use cocoa::appkit::NSWindowButton;
        use objc::{msg_send, runtime::YES, sel, sel_impl};

        unsafe {
            let id = self.ns_window().unwrap() as cocoa::base::id;
            let zoom_button = id.standardWindowButton_(NSWindowButton::NSWindowZoomButton);
            let _: () = msg_send![zoom_button, setHidden: YES];
        }
    }
}
