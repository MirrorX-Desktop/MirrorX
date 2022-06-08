use crate::media::ffmpeg::{
    avcodec::{
        avcodec::{AVCodecContext, AVCodecParserContext},
        packet::AVPacket,
    },
    avutil::frame::AVFrame,
};
use std::os::raw::c_char;

#[repr(C)]
pub struct VideoDecoder {
    pub codec_ctx: *mut AVCodecContext,
    pub frame: *mut AVFrame,
    pub packet: *mut AVPacket,
    pub parser_ctx: *mut AVCodecParserContext,
    pub hw_frame: *mut AVFrame,
}

extern "C" {
    pub fn video_decoder_create(decoder_name: *const c_char) -> *mut VideoDecoder;
    pub fn video_decoder_destroy(video_decoder: *mut VideoDecoder);
}
