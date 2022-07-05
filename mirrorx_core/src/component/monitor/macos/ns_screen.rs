use crate::error::MirrorXError;
use core_graphics::display::CGDirectDisplayID;
use objc::{
    class, msg_send,
    runtime::{Class, Object},
    sel, sel_impl,
};
use objc_foundation::{
    INSArray, INSObject, INSString, NSArray, NSDictionary, NSObject, NSString, NSValue,
};
use objc_id::{Id, Owned, Shared};
use scopeguard::defer;
use std::ops::{Deref, Index};

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
    pub fn screens() -> Result<Vec<NSScreen>, MirrorXError> {
        unsafe {
            let ns_screens_ptr: *mut NSArray<NSScreenClass> = msg_send![class!(NSScreen), screens];
            if ns_screens_ptr.is_null() {
                return Err(MirrorXError::Other(anyhow::anyhow!("get ns screen failed")));
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
}
