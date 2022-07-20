// #[cfg(target_os = "macos")]
// use std::os::raw::c_void;

// #[cfg(target_os = "macos")]
// pub struct DecodedFrame(pub *mut c_void);

// #[cfg(target_os = "windows")]
pub struct DecodedFrame {
    pub buffer: Vec<u8>,
    pub width: u32,
    pub height: u32,
}

unsafe impl Send for DecodedFrame {}
unsafe impl Sync for DecodedFrame {}
