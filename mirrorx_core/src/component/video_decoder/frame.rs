#[cfg(target_os = "macos")]
use crate::ffi::os::macos::core_video::CVPixelBufferRef;

#[cfg(target_os = "macos")]
pub struct DecodedFrame(pub CVPixelBufferRef);

#[cfg(target_os = "windows")]
pub struct DecodedFrame {
    pub buffer: Vec<u8>,
    pub width: i32,
    pub height: i32,
}

unsafe impl Send for DecodedFrame {}
