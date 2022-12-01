use super::Monitor;
use crate::{core_error, error::CoreResult};
use core_graphics::display::{CGDirectDisplayID, CGRect, *};
use mirrorx_native::os::macos::core_graphics::*;
use objc::{class, msg_send, runtime::Class, sel, sel_impl};
use objc_foundation::{
    INSArray, INSData, INSObject, INSString, NSArray, NSDictionary, NSMutableData, NSObject,
    NSString,
};
use objc_id::{Id, Owned};
use scopeguard::defer;
use std::{
    ops::{DerefMut, Index},
    os::raw::c_void,
};

pub fn get_primary_monitor_params() -> CoreResult<Monitor> {
    let monitors = get_active_monitors(false)?;
    for monitor in monitors.into_iter() {
        if monitor.is_primary {
            return Ok(monitor);
        }
    }

    Err(core_error!("no primary display"))
}

pub fn get_active_monitors(take_screen_shot: bool) -> CoreResult<Vec<Monitor>> {
    unsafe {
        let main_display_id = CGMainDisplayID();
        let ns_screens = NSScreen::screens()?;

        let mut displays = Vec::new();

        for ns_screen in ns_screens {
            let display_id = ns_screen.screenNumber();

            let monitor_width = CGDisplayPixelsWide(display_id);
            let monitor_height = CGDisplayPixelsHigh(display_id);

            let screen_shot_buffer = if take_screen_shot {
                take_screen_shot_as_png(display_id)
            } else {
                None
            };

            displays.push(Monitor {
                id: display_id.to_string(),
                name: String::default(), //ns_screen.localizedName(),
                refresh_rate: (ns_screen.maximumFramesPerSecond().min(u8::MAX as isize)) as u8,
                width: monitor_width as u16,
                height: monitor_height as u16,
                is_primary: display_id == main_display_id,
                screen_shot: screen_shot_buffer,
                left: 0,
                top: 0,
            });
        }

        Ok(displays)
    }
}

unsafe fn take_screen_shot_as_png(display_id: CGDirectDisplayID) -> Option<Vec<u8>> {
    let image_ref = CGDisplayCreateImage(display_id);
    if image_ref.is_null() {
        tracing::error!("CGDisplayCreateImage returns null");
        return None;
    }

    defer! {
        CGImageRelease(image_ref);
    }

    let mut data = NSMutableData::new();
    let data_ptr = data.deref_mut() as *mut _ as *mut c_void;

    let dest = CGImageDestinationCreateWithData(data_ptr, kUTTypePNG, 1, std::ptr::null());
    if dest.is_null() {
        tracing::error!("CGImageDestinationCreateWithData returns null");
        return None;
    }

    CGImageDestinationAddImage(dest, image_ref, std::ptr::null());

    if !CGImageDestinationFinalize(dest) {
        tracing::error!("CGImageDestinationFinalize returns false");
        return None;
    }

    Some(data.bytes().to_vec())
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

    #[allow(non_snake_case)]
    pub fn maximumFramesPerSecond(&self) -> isize {
        unsafe { msg_send![self.class, maximumFramesPerSecond] }
    }

    #[allow(non_snake_case)]
    pub fn screenNumber(&self) -> CGDirectDisplayID {
        unsafe {
            let description_ptr: *mut NSDictionary<NSString, NSObject> =
                msg_send![self.class, deviceDescription];

            let description: Id<_, Owned> = Id::from_ptr(description_ptr);

            let key = NSString::from_str("NSScreenNumber");

            let value = description.index(&key);

            msg_send![value, unsignedIntValue]
        }
    }

    #[allow(non_snake_case)]
    pub fn localizedName(&self) -> String {
        unsafe {
            let name: Id<NSString> = msg_send![self.class, localizedName];
            name.as_str().to_string()
        }
    }

    pub fn frame(&self) -> CGRect {
        unsafe { msg_send![self.class, frame] }
    }
}
