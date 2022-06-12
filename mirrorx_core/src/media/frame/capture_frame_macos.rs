use crate::media::bindings::macos::{CVPixelBufferRef, CVPixelBufferRelease};

pub struct CaptureFrame {
    pub cv_pixel_buffer: CVPixelBufferRef,
}

unsafe impl Send for CaptureFrame {}
unsafe impl Sync for CaptureFrame {}

impl Drop for CaptureFrame {
    fn drop(&mut self) {
        unsafe {
            if !self.cv_pixel_buffer.is_null() {
                CVPixelBufferRelease(self.cv_pixel_buffer);
            }
        }
    }
}
