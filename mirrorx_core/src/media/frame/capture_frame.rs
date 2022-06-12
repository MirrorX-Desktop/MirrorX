use crate::media::bindings::macos::{CVPixelBufferRef, CVPixelBufferRelease};

pub struct CaptureFrame {
    #[cfg(target_os = "macos")]
    pub cv_pixel_buffer: CVPixelBufferRef,
}

unsafe impl Send for CaptureFrame {}
unsafe impl Sync for CaptureFrame {}

impl Drop for CaptureFrame {
    fn drop(&mut self) {
        #[cfg(target_os = "macos")]
        unsafe {
            if !self.cv_pixel_buffer.is_null() {
                CVPixelBufferRelease(self.cv_pixel_buffer);
            }
        }
    }
}
