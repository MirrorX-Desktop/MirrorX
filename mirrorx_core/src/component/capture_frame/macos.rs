use crate::ffi::os::macos::{core_media::CMTime, core_video::CVPixelBufferRef};
use core_foundation::base::CFRelease;

pub struct CaptureFrame {
    pub pts: CMTime,
    pub pixel_buffer: CVPixelBufferRef,
}

unsafe impl Send for CaptureFrame {}

impl Drop for CaptureFrame {
    fn drop(&mut self) {
        if !self.pixel_buffer.is_null() {
            unsafe {
                CFRelease(self.pixel_buffer);
            }
        }
    }
}
