use std::os::raw::c_void;

#[cfg(target_os = "macos")]
pub struct Frame(pub *mut c_void);

#[cfg(target_os = "windows")]
pub struct Frame(pub Vec<u8>);

unsafe impl Send for Frame {}
unsafe impl Sync for Frame {}
