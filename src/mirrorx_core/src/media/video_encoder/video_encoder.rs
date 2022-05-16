use super::bindings;
use crate::media::{video_frame::VideoFrame, video_packet::VideoPacket};
use anyhow::{bail, Ok};
use crossbeam_channel::{bounded, Receiver, Sender};
use log::info;
use std::{
    ffi::CString,
    os::raw::{c_int, c_void},
};

struct InnerVideoEncoderPointer(*mut c_void);

unsafe impl Send for InnerVideoEncoderPointer {}
unsafe impl Sync for InnerVideoEncoderPointer {}

impl Drop for InnerVideoEncoderPointer {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe {
                info!("free_video_encoder");
                bindings::video_encoder_destroy(self.0);
                info!("free_video_encoder done");
            }
        }
    }
}

pub struct VideoEncoder {
    video_encoder_ptr: InnerVideoEncoderPointer,
    tx: Box<Sender<VideoPacket>>,
}

impl VideoEncoder {
    pub fn new(
        encoder_name: &str,
        fps: i32,
        screen_width: u16,
        screen_height: u16,
    ) -> anyhow::Result<(VideoEncoder, Receiver<VideoPacket>)> {
        let encoder_name_ptr = CString::new(encoder_name.to_string())?;

        unsafe {
            let video_encoder_ptr = bindings::video_encoder_create(
                encoder_name_ptr.as_ptr(),
                screen_width as c_int,
                screen_height as c_int,
                fps,
                bindings::callback,
            );

            if video_encoder_ptr.is_null() {
                bail!("create video encoder failed");
            }

            let (tx, rx) = bounded::<VideoPacket>(600);

            Ok((
                VideoEncoder {
                    video_encoder_ptr: InnerVideoEncoderPointer(video_encoder_ptr),
                    tx: Box::new(tx),
                },
                rx,
            ))
        }
    }

    pub fn encode(&mut self, video_frame: &VideoFrame) -> anyhow::Result<()> {
        unsafe {
            let success = bindings::video_encoder_encode(
                self.video_encoder_ptr.0,
                self.tx.as_mut() as *mut _ as *mut c_void,
                video_frame.width,
                video_frame.height,
                video_frame.is_full_color_range,
                video_frame.y_plane_buffer.as_ptr(),
                video_frame.y_plane_stride,
                video_frame.uv_plane_buffer.as_ptr(),
                video_frame.uv_plane_stride,
                video_frame.dts,
                video_frame.dts_scale,
                video_frame.pts,
                video_frame.pts_scale,
            );

            if success {
                Ok(())
            } else {
                bail!("encode video failed")
            }
        }
    }
}
