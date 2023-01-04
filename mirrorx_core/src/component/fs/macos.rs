use crate::{core_error, error::CoreResult};
use cocoa::{appkit::NSImage, foundation::NSData};
use objc::{
    class, msg_send,
    runtime::{Class, Object},
    sel, sel_impl,
};
use objc_foundation::{INSObject, INSString, NSString};
use objc_id::{Id, Owned};
use std::{io::Cursor, path::Path};

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

            let ns_data = ns_image.TIFFRepresentation();

            let tiff = image::load_from_memory_with_format(
                std::slice::from_raw_parts(ns_data.bytes() as *const u8, ns_data.length() as usize),
                image::ImageFormat::Tiff,
            )?;

            let tiff = tiff.thumbnail(32, 32);

            let mut buffer = Vec::with_capacity(4096);
            let mut writer = Cursor::new(&mut buffer);

            tiff.write_to(&mut writer, image::ImageOutputFormat::Png)?;

            Ok(buffer)
        }
    }
}
