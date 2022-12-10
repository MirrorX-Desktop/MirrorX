use super::core_foundation::CFMutableDataRef;
use core_foundation::{dictionary::CFDictionaryRef, string::CFStringRef};
use core_graphics::sys::CGImageRef;
use std::os::raw::c_void;

pub type CGImageDestinationRef = *mut c_void;

pub type CGDisplayStreamRef = *mut c_void;
pub type CGDisplayStreamUpdateRef = *mut c_void;

pub type CGDisplayStreamFrameStatus = i32;
pub const kCGDisplayStreamFrameStatusFrameComplete: CGDisplayStreamFrameStatus = 0;
pub const kCGDisplayStreamFrameStatusFrameIdle: CGDisplayStreamFrameStatus = 1;
pub const kCGDisplayStreamFrameStatusFrameBlank: CGDisplayStreamFrameStatus = 2;
pub const kCGDisplayStreamFrameStatusStopped: CGDisplayStreamFrameStatus = 3;

pub type CGDisplayStreamFrameAvailableHandler<'a> = &'a block::Block<
    (
        CGDisplayStreamFrameStatus,
        u64,
        super::io_surface::IOSurfaceRef,
        CGDisplayStreamUpdateRef,
    ),
    (),
>;

extern "C" {
    pub static kUTTypePNG: CFStringRef;
}

extern "C" {
    #[allow(improper_ctypes)]
    pub fn CGDisplayStreamCreateWithDispatchQueue(
        display: core_graphics::display::CGDirectDisplayID,
        output_width: usize,
        output_height: usize,
        pixel_format: i32,
        properties: core_foundation::dictionary::CFDictionaryRef,
        queue: dispatch::ffi::dispatch_queue_t,
        handler: CGDisplayStreamFrameAvailableHandler,
    ) -> CGDisplayStreamRef;
    pub fn CGDisplayStreamStart(display_stream: CGDisplayStreamRef)
        -> core_graphics::base::CGError;
    pub fn CGDisplayStreamStop(display_stream: CGDisplayStreamRef) -> core_graphics::base::CGError;
    pub fn CGDisplayStreamUpdateGetDropCount(update_ref: CGDisplayStreamUpdateRef) -> u32;
    pub fn CGImageDestinationCreateWithData(
        data: CFMutableDataRef,
        typ: CFStringRef,
        count: usize,
        options: CFDictionaryRef,
    ) -> CGImageDestinationRef;
    pub fn CGImageDestinationAddImage(
        idst: CGImageDestinationRef,
        image: CGImageRef,
        properties: CFDictionaryRef,
    );
    pub fn CGImageDestinationFinalize(idst: CGImageDestinationRef) -> bool;
    pub fn CGImageRelease(image: CGImageRef);
}
