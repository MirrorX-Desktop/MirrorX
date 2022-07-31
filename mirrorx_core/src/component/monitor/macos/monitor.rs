use super::ns_screen::NSScreen;
use crate::{
    component::monitor::Monitor,
    error::MirrorXError,
    ffi::os::{macos::core_graphics::*, *},
};
use core_graphics::display::*;
use objc_foundation::{INSData, INSObject, NSMutableData};
use scopeguard::defer;
use std::{ops::DerefMut, os::raw::c_void};
use tracing::error;

pub fn get_active_monitors() -> Result<Vec<Monitor>, MirrorXError> {
    unsafe {
        let main_display_id = CGMainDisplayID();
        let ns_screens = NSScreen::screens()?;

        let mut displays = Vec::new();

        for ns_screen in ns_screens {
            let display_id = ns_screen.screenNumber();

            let monitor_width = CGDisplayPixelsWide(display_id);
            let monitor_height = CGDisplayPixelsHigh(display_id);

            let screen_shot_buffer = take_screen_shot_as_png(display_id);

            if let Some(screen_shot_buffer) = screen_shot_buffer {
                displays.push(Monitor {
                    id: display_id.to_string(),
                    name: ns_screen.localizedName(),
                    refresh_rate: (ns_screen.maximumFramesPerSecond().min(u8::MAX as isize)) as u8,
                    width: monitor_width as u16,
                    height: monitor_height as u16,
                    is_primary: display_id == main_display_id,
                    screen_shot: screen_shot_buffer,
                    left: 0,
                    top: 0,
                });
            }
        }

        Ok(displays)
    }
}

unsafe fn take_screen_shot_as_png(display_id: CGDirectDisplayID) -> Option<Vec<u8>> {
    let image_ref = CGDisplayCreateImage(display_id);
    if image_ref.is_null() {
        error!("CGDisplayCreateImage failed");
        return None;
    }

    defer! {
        CGImageRelease(image_ref);
    }

    let mut data = NSMutableData::new();
    let data_ptr = data.deref_mut() as *mut _ as *mut c_void;

    let dest = CGImageDestinationCreateWithData(data_ptr, kUTTypePNG, 1, std::ptr::null());
    if dest.is_null() {
        error!("CGImageDestinationCreateWithData failed");
        return None;
    }

    CGImageDestinationAddImage(dest, image_ref, std::ptr::null());

    if !CGImageDestinationFinalize(dest) {
        error!("CGImageDestinationFinalize failed");
        return None;
    }

    Some(data.bytes().to_vec())
}
