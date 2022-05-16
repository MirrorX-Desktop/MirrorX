use crate::media::video_packet::VideoPacket;
use crossbeam_channel::Sender;
use log::{error, trace};
use std::ffi::c_void;
use std::os::raw::{c_char, c_int, c_longlong};
use std::slice::from_raw_parts;

/// cbindgen:ignore
extern "C" {
    pub fn video_encoder_create(
        encoder_name: *const c_char,
        screen_width: c_int,
        screen_height: c_int,
        fps: c_int,
        callback: unsafe extern "C" fn(
            tx: *mut c_void,
            packet_data_base_address: *const u8,
            packet_data_size: c_int,
            dts: i64,
            pts: i64,
        ),
    ) -> *mut c_void;

    pub fn video_encoder_encode(
        video_encoder: *mut c_void,
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
    ) -> bool;

    pub fn video_encoder_destroy(video_encoder: *mut c_void);
}

pub unsafe extern "C" fn callback(
    tx: *mut c_void,
    packet_data_base_address: *const u8,
    packet_size: c_int,
    dts: i64,
    pts: i64,
) {
    trace!(
        "video_encoder: encode callback triggered, packet_size: {}",
        packet_size
    );

    if tx.is_null() {
        error!("video_encoder: callback tx ptr is nil");
        return;
    }

    let tx = tx as *mut Sender<VideoPacket>;
    let tx = match tx.as_ref() {
        Some(tx) => tx,
        None => {
            error!("video_encoder callback: tx reference is null");
            return;
        }
    };

    let data = from_raw_parts(packet_data_base_address, packet_size as usize).to_vec();

    let packet = VideoPacket { data, dts, pts };

    if let Err(err) = tx.try_send(packet) {
        error!("video_encoder: callback send resources failed: {}", err);
    }
}
