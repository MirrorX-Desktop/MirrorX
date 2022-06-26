use std::os::raw::c_void;

#[cfg(target_os = "macos")]
pub struct NativeFrame(pub *mut c_void);

#[cfg(target_os = "windows")]
pub struct NativeFrame(pub Vec<u8>);

unsafe impl Send for NativeFrame {}
unsafe impl Sync for NativeFrame {}
