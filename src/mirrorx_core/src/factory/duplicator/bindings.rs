use std::ffi::c_void;
use std::os::raw::c_int;

/// cbindgen:ignore
extern "C" {
    pub fn create_duplication_context(
        display_index: c_int,
        tx: *const c_void,
        cb: extern "C" fn(
            tx: *const c_void,
            width: c_int,
            height: c_int,
            y_line_size: c_int,
            y_buffer_address: *const u8,
            uv_line_size: c_int,
            uv_buffer_address: *const u8,
        ),
    ) -> *const c_void;

    pub fn release_duplication_context(context: *const c_void);

    pub fn start_capture(context: *const c_void);

    pub fn stop_capture(context: *const c_void);
}
