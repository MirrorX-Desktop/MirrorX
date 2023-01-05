use crate::{core_error, error::CoreResult};
use cocoa::{
    appkit::NSImage,
    base::id,
    foundation::{NSData, NSSize},
};
use objc::{
    class, msg_send,
    runtime::{Class, Object},
    sel, sel_impl,
};
use objc_foundation::{INSObject, INSString, NSString};
use objc_id::{Id, Owned};
use std::path::Path;

struct NSBitmapImageRepClass {}

unsafe impl objc::Message for NSBitmapImageRepClass {}

impl INSObject for NSBitmapImageRepClass {
    fn class() -> &'static objc::runtime::Class {
        Class::get("NSBitmapImageRep").unwrap()
    }
}

pub struct NSBitmapImageRep {
    class: Id<NSBitmapImageRepClass>,
}

impl NSBitmapImageRep {
    #[allow(non_snake_case)]
    pub fn initWithCGImage(cg_image_ref: id) -> CoreResult<NSBitmapImageRep> {
        unsafe {
            let ns_bitmap_image_rep_ptr: *mut Object = msg_send![class!(NSBitmapImageRep), alloc];
            let ns_bitmap_image_rep_ptr: *mut NSBitmapImageRepClass =
                msg_send![ns_bitmap_image_rep_ptr, initWithCGImage: cg_image_ref];

            if ns_bitmap_image_rep_ptr.is_null() {
                return Err(core_error!("NSBitmapImageRep.alloc returns null"));
            }

            let id: Id<_, Owned> = Id::from_ptr(ns_bitmap_image_rep_ptr);

            Ok(NSBitmapImageRep { class: id })
        }
    }

    #[allow(non_snake_case)]
    pub fn setSize(&self, size: NSSize) {
        unsafe {
            let _: () = msg_send![self.class, setSize: size];
        }
    }

    // - (NSData *)representationUsingType:(NSBitmapImageFileType)storageType
    // properties:(NSDictionary<NSBitmapImageRepPropertyKey, id> *)properties;
    #[allow(non_snake_case)]
    pub fn representationUsingTypeForPNG(&self) -> id {
        unsafe { msg_send![self.class, representationUsingType:4 properties:cocoa::base::nil] }
    }
}

struct NSWorkspaceClass {}

unsafe impl objc::Message for NSWorkspaceClass {}

impl INSObject for NSWorkspaceClass {
    fn class() -> &'static objc::runtime::Class {
        Class::get("NSWorkspace").unwrap()
    }
}

pub struct NSWorkspace {
    class: Id<NSWorkspaceClass>,
}

impl NSWorkspace {
    pub fn sharedWorkspace() -> CoreResult<NSWorkspace> {
        unsafe {
            let workspace_ptr: *mut NSWorkspaceClass =
                msg_send![class!(NSWorkspace), sharedWorkspace];
            if workspace_ptr.is_null() {
                return Err(core_error!("NSWorkspace.sharedWorkspace returns null"));
            }

            let id: Id<_, Owned> = Id::from_ptr(workspace_ptr);

            Ok(NSWorkspace { class: id })
        }
    }

    #[allow(non_snake_case)]
    pub fn iconForFile(&self, path: &Path) -> CoreResult<Vec<u8>> {
        unsafe {
            let path = path
                .as_os_str()
                .to_str()
                .ok_or(core_error!("convert path to str failed"))?;
            let path = NSString::from_str(path);

            let ns_image: *mut Object = msg_send![self.class, iconForFile: path];
            if ns_image.is_null() {
                return Err(core_error!("get iconForFile failed"));
            }

            let cg_ref: id = msg_send![
                ns_image,
                CGImageForProposedRect: cocoa::base::nil
                context: cocoa::base::nil
                hints: cocoa::base::nil
            ];

            let rep = NSBitmapImageRep::initWithCGImage(cg_ref)?;
            rep.setSize(ns_image.size());

            let ns_data = rep.representationUsingTypeForPNG();

            Ok(
                std::slice::from_raw_parts(ns_data.bytes() as *const u8, ns_data.length() as usize)
                    .to_vec(),
            )
        }
    }
}
