use crate::media::ffmpeg::{
    avcodec::{avcodec::AVCodecContext, packet::AVPacket},
    avutil::frame::AVFrame,
};
use std::{
    ffi::c_void,
    os::raw::{c_char, c_int},
};

#[repr(C)]
pub struct VideoEncoder {
    pub codec: *const c_void,
    pub codec_ctx: *mut AVCodecContext,
    pub frame: *mut AVFrame,
    pub packet: *mut AVPacket,
}

/// cbindgen:ignore
extern "C" {
    pub fn video_encoder_create(
        encoder_name: *const c_char,
        screen_width: c_int,
        screen_height: c_int,
        fps: c_int,
    ) -> *mut VideoEncoder;

    pub fn video_encoder_set_opt(
        encoder: *mut VideoEncoder,
        opt_name: *const c_char,
        opt_value: *const c_char,
    ) -> bool;

    pub fn video_encoder_open(encoder: *mut VideoEncoder) -> bool;

    pub fn video_encoder_destroy(video_encoder: *mut VideoEncoder);
}
