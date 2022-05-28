use std::os::raw::c_void;

pub struct NativeFrame(pub *mut c_void);

unsafe impl Send for NativeFrame {}
unsafe impl Sync for NativeFrame {}
