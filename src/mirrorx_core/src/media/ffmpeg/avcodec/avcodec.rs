use super::packet::AVPacket;
use crate::media::ffmpeg::avutil::frame::AVFrame;
use std::ffi::c_void;

extern "C" {
    pub fn avcodec_receive_packet(avctx: *mut c_void, avpkt: *mut AVPacket) -> i32;
    pub fn avcodec_send_frame(avctx: *mut c_void, frame: *mut AVFrame) -> i32;
}
