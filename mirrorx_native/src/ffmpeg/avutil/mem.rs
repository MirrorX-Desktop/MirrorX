use std::ffi::c_void;

extern "C" {
    pub fn av_freep(ptr: *mut c_void);
}
