use std::ffi::c_void;
use std::os::raw::{c_char, c_int};
use std::sync::mpsc::Sender;

/// cbindgen:ignore
extern "C" {
    pub fn new_video_encoder(
        encoder_name: *const c_char,
        fps: c_int,
        src_width: c_int,
        src_height: c_int,
        dst_width: c_int,
        dst_height: c_int,
        encode_callback: unsafe extern "C" fn(
            tx: *mut Sender<Vec<u8>>,
            packet_data: *const u8,
            packet_size: c_int,
        ),
    ) -> *const c_void;

    pub fn video_encode(
        video_encoder: *const c_void,
        tx: *mut Sender<Vec<u8>>,
        width: c_int,
        height: c_int,
        y_line_size: c_int,
        y_buffer: *const u8,
        uv_line_size: c_int,
        uv_buffer: *const u8,
    ) -> c_int;

    pub fn free_video_encoder(video_encoder: *const c_void);
}
