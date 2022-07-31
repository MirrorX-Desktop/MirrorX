use std::os::raw::c_void;

pub type IOSurfaceRef = *mut c_void;

pub struct IOSurfaceRefWrapper {
    pub surface: IOSurfaceRef,
}

unsafe impl Send for IOSurfaceRefWrapper {}

extern "C" {
    pub fn IOSurfaceIncrementUseCount(buffer: *mut c_void);
    pub fn IOSurfaceDecrementUseCount(buffer: *mut c_void);
}
