// use crate::media::video_frame::VideoFrame;
// use crossbeam_channel::Sender;
// use log::error;
// use std::ffi::c_void;
// use std::os::raw::{c_char, c_int, c_longlong};
// use std::slice::from_raw_parts;

// /// cbindgen:ignore
// extern "C" {
//     pub fn video_decoder_create(
//         decoder_name: *const c_char,
//         callback: unsafe extern "C" fn(
//             tx: *mut c_void,
//             width: u16,
//             height: u16,
//             is_full_color_range: bool,
//             y_plane_buffer_address: *const u8,
//             y_plane_stride: u32,
//             uv_plane_buffer_address: *const u8,
//             uv_plane_stride: u32,
//             dts: i64,
//             pts: i64,
//         ),
//     ) -> *mut c_void;

//     pub fn video_decoder_decode(
//         video_decoder: *mut c_void,
//         tx: *mut c_void,
//         packet_data: *const u8,
//         packet_size: c_int,
//         dts: i64,
//         pts: i64,
//     );

//     pub fn video_decoder_destroy(video_decoder: *mut c_void);
// }

// pub unsafe extern "C" fn callback(
//     tx: *mut c_void,
//     width: u16,
//     height: u16,
//     is_full_color_range: bool,
//     y_plane_buffer_address: *const u8,
//     y_plane_stride: u32,
//     uv_plane_buffer_address: *const u8,
//     uv_plane_stride: u32,
//     dts: i64,
//     pts: i64,
// ) {
//     if tx.is_null() {
//         error!("video_decoder: callback tx ptr is nil");
//         return;
//     }

//     let tx = tx as *mut Sender<VideoFrame>;
//     let tx = match tx.as_ref() {
//         Some(tx) => tx,
//         None => {
//             error!("desktop_duplicator callback: tx reference is null");
//             return;
//         }
//     };

//     let y_plane_buffer = from_raw_parts(
//         y_plane_buffer_address,
//         (y_plane_stride as usize) * (height as usize),
//     )
//     .to_vec();

//     let uv_plane_buffer = from_raw_parts(
//         uv_plane_buffer_address,
//         (uv_plane_stride as usize) * (height as usize) / 2,
//     )
//     .to_vec();

//     let frame = VideoFrame {
//         width,
//         height,
//         is_full_color_range,
//         y_plane_buffer,
//         y_plane_stride,
//         uv_plane_buffer,
//         uv_plane_stride,
//         dts,
//         dts_scale: 0,
//         pts,
//         pts_scale: 0,
//     };

//     if let Err(err) = tx.try_send(frame) {
//         error!("video_decoder: callback send resources failed: {}", err);
//     }
// }
