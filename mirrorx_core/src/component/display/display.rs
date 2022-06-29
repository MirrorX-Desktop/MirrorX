use crate::{error::MirrorXError, ffi::os::*};
use core_graphics::{display::*, sys::CGImageRef};
use libc::c_void;
use objc_foundation::{INSData, INSObject, NSMutableData};
use scopeguard::defer;
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};
use tracing::error;

const MAX_QUERY_DISPLAYS_COUNT: usize = 16;

pub struct Display {
    pub id: u32,
    pub is_main: bool,
    pub screen_shot: Vec<u8>,
}

pub fn get_active_displays() -> Result<Vec<Display>, MirrorXError> {
    unsafe {
        let mut display_ids: [CGDirectDisplayID; MAX_QUERY_DISPLAYS_COUNT] = Default::default();
        let mut display_count = 0u32;
        let cg_error = CGGetActiveDisplayList(
            MAX_QUERY_DISPLAYS_COUNT as u32,
            display_ids.as_mut_ptr(),
            &mut display_count as *mut _,
        );

        if cg_error != 0 {
            return Err(MirrorXError::Other(anyhow::anyhow!(
                "CGGetActiveDisplayList error({})",
                cg_error
            )));
        }

        let mut displays = Vec::new();
        let main_display_id = CGMainDisplayID();
        for i in 0..display_count {
            let display_id = display_ids[i as usize];
            let screen_shot_buffer = take_screen_shot_as_png(display_id);

            if let Some(buffer) = screen_shot_buffer {
                displays.push(Display {
                    id: display_id,
                    is_main: display_id == main_display_id,
                    screen_shot: buffer,
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
