use crate::{
    component::display::{Display, Rect, RESIZE_FACTOR},
    core_error,
    error::CoreResult,
};
use core_foundation::{
    base::kCFAllocatorDefault,
    dictionary::CFDictionaryCreate,
    number::{kCFNumberFloat32Type, CFNumber, CFNumberCreate, CFNumberRef},
};
use core_graphics::{context::CGContext, display::*, image::CGImage};
use image::imageops::FilterType;
use mirrorx_native::os::macos::core_graphics::*;
use objc::{class, msg_send, runtime::Class, sel, sel_impl};
use objc_foundation::*;
use objc_id::*;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use scopeguard::defer;
use std::{
    io::{BufWriter, Cursor},
    ops::{DerefMut, Index},
    os::raw::c_void,
};

pub fn enum_all_available_displays() -> CoreResult<Vec<Display>> {
    unsafe {
        let ns_screens = NSScreen::screens()?;

        let displays = ns_screens
            .par_iter()
            .map(|ns_screen| {
                let id = ns_screen.screen_number();
                let rect = CGDisplayBounds(id);

                Display {
                    id: id.to_string(),
                    name: ns_screen.localized_name(),
                    left: rect.origin.x as _,
                    top: rect.origin.y as _,
                    width: rect.size.width as _,
                    height: rect.size.height as _,
                }
            })
            .collect();

        Ok(displays)
    }
}

struct NSScreenClass {}

unsafe impl objc::Message for NSScreenClass {}

impl INSObject for NSScreenClass {
    fn class() -> &'static objc::runtime::Class {
        Class::get("NSScreen").unwrap()
    }
}

pub struct NSScreen {
    class: Id<NSScreenClass>,
}

impl NSScreen {
    pub fn screens() -> CoreResult<Vec<NSScreen>> {
        unsafe {
            let ns_screens_ptr: *mut NSArray<NSScreenClass> = msg_send![class!(NSScreen), screens];
            if ns_screens_ptr.is_null() {
                return Err(core_error!("NSScreen.screens returns null"));
            }

            let ns_screens: Id<_, Owned> = Id::from_ptr(ns_screens_ptr);
            let screens = NSArray::into_vec(ns_screens)
                .into_iter()
                .map(|id| NSScreen { class: id })
                .collect();

            Ok(screens)
        }
    }

    pub fn maximum_frames_per_second(&self) -> isize {
        unsafe { msg_send![self.class, maximumFramesPerSecond] }
    }

    pub fn screen_number(&self) -> CGDirectDisplayID {
        unsafe {
            let description_ptr: *mut NSDictionary<NSString, NSObject> =
                msg_send![self.class, deviceDescription];

            let description: Id<_, Owned> = Id::from_ptr(description_ptr);

            let key = NSString::from_str("NSScreenNumber");

            let value = description.index(&key);

            msg_send![value, unsignedIntValue]
        }
    }

    // pub fn localized_name(&self) -> String {
    //     unsafe {
    //         let name: Id<NSString> = msg_send![self.class, localizedName];
    //         name.as_str().to_string()
    //     }
    // }
}
