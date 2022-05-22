use crate::media::video_frame::VideoFrame;
use crossbeam_channel::Sender;
use log::error;
use std::ffi::c_void;
use std::os::raw::c_int;
use std::slice::from_raw_parts;

/// cbindgen:ignore
extern "C" {
    pub fn desktop_duplicator_create(
        display_index: c_int,
        fps: c_int,
        tx: *mut c_void,
        callback: unsafe extern "C" fn(
            tx: *mut c_void,
            width: u16,
            height: u16,
            is_full_color_range: bool,
            y_plane_buffer_address: *const u8,
            y_plane_stride: u32,
            uv_plane_buffer_address: *const u8,
            uv_plane_stride: u32,
            dts: i64,
            dts_scale: i32,
            pts: i64,
            pts_scale: i32,
        ),
    ) -> *mut c_void;

    pub fn desktop_duplicator_destroy(context: *mut c_void);

    pub fn desktop_duplicator_start(context: *mut c_void);

    pub fn desktop_duplicator_stop(context: *mut c_void);
}

pub unsafe extern "C" fn callback(
    tx: *mut c_void,
    width: u16,
    height: u16,
    is_full_color_range: bool,
    y_plane_buffer_address: *const u8,
    y_plane_stride: u32,
    uv_plane_buffer_address: *const u8,
    uv_plane_stride: u32,
    dts: i64,
    dts_scale: i32,
    pts: i64,
    pts_scale: i32,
) {
    if tx.is_null() {
        error!("desktop_duplicator callback: tx is null");
        return;
    }

    let tx = tx as *mut Sender<VideoFrame>;
    let tx = match tx.as_ref() {
        Some(tx) => tx,
        None => {
            error!("desktop_duplicator callback: tx reference is null");
            return;
        }
    };

    let y_plane_buffer = from_raw_parts(
        y_plane_buffer_address,
        (y_plane_stride as usize) * (height as usize),
    )
    .to_vec();

    let uv_plane_buffer = from_raw_parts(
        uv_plane_buffer_address,
        (uv_plane_stride as usize) * (height as usize) / 2,
    )
    .to_vec();

    let frame = VideoFrame {
        width,
        height,
        is_full_color_range,
        y_plane_buffer,
        y_plane_stride,
        uv_plane_buffer,
        uv_plane_stride,
        dts,
        dts_scale,
        pts,
        pts_scale,
    };

    if let Err(err) = tx.try_send(frame) {
        error!("duplicator: callback send resources failed: {}", err);
    }
}
