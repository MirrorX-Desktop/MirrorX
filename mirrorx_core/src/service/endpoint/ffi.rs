use std::os::raw::c_void;

// #[cfg(target_os = "macos")]
// pub const unsafe fn create_callback_fn(
//     callback_ptr: i64,
// ) -> unsafe extern "C" fn(i64, *mut c_void, *const c_void) {
//     std::mem::transmute::<
//         *const c_void,
//         unsafe extern "C" fn(
//             texture_id: i64,
//             video_texture_ptr: *mut c_void,
//             new_frame_ptr: *const c_void,
//         ),
//     >(callback_ptr as *const c_void)
// }

// #[cfg(target_os = "windows")]
pub const unsafe fn create_callback_fn(
    callback_ptr: i64,
) -> unsafe extern "C" fn(*mut c_void, *const u8, usize, usize) {
    std::mem::transmute::<
        *const c_void,
        unsafe extern "C" fn(
            video_texture_ptr: *mut c_void,
            frame_buffer: *const u8,
            frame_width: usize,
            frame_height: usize,
        ),
    >(callback_ptr as *const c_void)
}
