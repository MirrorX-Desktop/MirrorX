#[cfg(target_os = "macos")]
use std::os::raw::c_void;

#[cfg(target_os = "macos")]
pub struct DecodedFrame(pub *mut c_void);

#[cfg(target_os = "windows")]
pub struct DecodedFrame(pub Vec<u8>);

unsafe impl Send for DecodedFrame {}
unsafe impl Sync for DecodedFrame {}
