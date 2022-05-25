// use super::bindings;
// use crate::media::{video_frame::VideoFrame, video_packet::VideoPacket};
// use anyhow::bail;
// use crossbeam_channel::{bounded, Receiver, Sender};
// use log::info;
// use std::{
//     ffi::CString,
//     os::raw::{c_int, c_void},
// };

// struct InnerVideoDecoderPointer(*mut c_void);

// unsafe impl Send for InnerVideoDecoderPointer {}
// unsafe impl Sync for InnerVideoDecoderPointer {}

// impl Drop for InnerVideoDecoderPointer {
//     fn drop(&mut self) {
//         if !self.0.is_null() {
//             unsafe {
//                 info!("free_video_decoder");
//                 bindings::video_decoder_destroy(self.0);
//                 info!("free_video_decoder done");
//             }
//         }
//     }
// }

// pub struct VideoDecoder {
//     video_decoder_ptr: InnerVideoDecoderPointer,
//     tx: Sender<VideoFrame>,
// }

// impl VideoDecoder {
//     pub fn new(decoder_name: &str) -> anyhow::Result<(VideoDecoder, Receiver<VideoFrame>)> {
//         let decoder_name_ptr = CString::new(decoder_name.to_string())?;

//         unsafe {
//             let video_decoder_ptr =
//                 bindings::video_decoder_create(decoder_name_ptr.as_ptr(), bindings::callback);

//             if video_decoder_ptr.is_null() {
//                 bail!("create video decoder failed");
//             }

//             let (tx, rx) = bounded::<VideoFrame>(600);

//             Ok((
//                 VideoDecoder {
//                     video_decoder_ptr: InnerVideoDecoderPointer(video_decoder_ptr),
//                     tx,
//                 },
//                 rx,
//             ))
//         }
//     }

//     pub fn decode(&mut self, packet: &VideoPacket) {
//         unsafe {
//             bindings::video_decoder_decode(
//                 self.video_decoder_ptr.0,
//                 &mut self.tx as *mut _ as *mut c_void,
//                 packet.data.as_ptr(),
//                 packet.data.len() as c_int,
//                 packet.dts,
//                 packet.pts,
//             )
//         }
//     }
// }
