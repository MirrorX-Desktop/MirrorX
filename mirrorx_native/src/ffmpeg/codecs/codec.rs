use super::codec_id::AVCodecID;
use crate::ffmpeg::{
    codecs::{
        avcodec::{AVCodecContext, AVSubtitle},
        packet::AVPacket,
    },
    utils::{
        avutil::AVMediaType, frame::AVFrame, hwcontext::AVHWDeviceType, log::AVClass,
        pixfmt::AVPixelFormat, rational::AVRational, samplefmt::AVSampleFormat,
    },
};
use std::os::raw::{c_char, c_void};

pub const AV_CODEC_CAP_TRUNCATED: i32 = 1 << 3;

#[repr(C)]
pub struct AVProfile {
    pub profile: i32,
    pub name: *const c_char,
}

pub enum AVCodecDefault {}

#[repr(C)]
pub struct AVCodec {
    pub name: *const c_char,
    pub long_name: *const c_char,
    pub type_: AVMediaType,
    pub id: AVCodecID,
    pub capabilities: i32,
    pub max_lowres: u8,
    pub supported_framerates: *const AVRational,
    pub pix_fmts: *const AVPixelFormat,
    pub supported_samplerates: *const i32,
    pub sample_fmts: *const AVSampleFormat,
    pub channel_layouts: *const u64,
    pub priv_class: *const AVClass,
    pub profiles: *const AVProfile,
    pub wrapper_name: *const c_char,
    pub caps_internal: i32,
    pub priv_data_size: i32,
    pub update_thread_context:
        extern "C" fn(dst: *mut AVCodecContext, src: *const AVCodecContext) -> i32,
    pub update_thread_context_for_user:
        extern "C" fn(dst: *mut AVCodecContext, src: *const AVCodecContext) -> i32,
    pub defaults: *const AVCodecDefault,
    pub init_static_data: extern "C" fn(codec: *mut AVCodec),
    pub init: extern "C" fn(codec: *mut AVCodecContext) -> i32,
    pub encode_sub: extern "C" fn(
        avctx: *mut AVCodecContext,
        buf: *mut u8,
        buf_size: i32,
        sub: *const AVSubtitle,
    ) -> i32,
    pub encode2: extern "C" fn(
        avctx: *mut AVCodecContext,
        avpkt: *mut AVPacket,
        frame: *const AVFrame,
        got_packet_ptr: *mut i32,
    ) -> i32,
    pub decode: extern "C" fn(
        avctx: *mut AVCodecContext,
        outdata: *mut c_void,
        got_frame_ptr: *mut i32,
        avpkt: *mut AVPacket,
    ) -> i32,
    pub close: extern "C" fn(avctx: *mut AVCodecContext) -> i32,
    pub receive_packet: extern "C" fn(avctx: *mut AVCodecContext, avpkt: *mut AVPacket) -> i32,
    pub receive_frame: extern "C" fn(avctx: *mut AVCodecContext, frame: *mut AVFrame) -> i32,
    pub flush: extern "C" fn(avctx: *mut AVCodecContext) -> i32,
    pub bsfs: *const c_char,
    pub hw_configs: *const *const c_void,
    pub codec_tags: *const u32,
}

#[repr(C)]
pub struct AVCodecHWConfig {
    pub pix_fmt: AVPixelFormat,
    pub methods: i32,
    pub device_type: AVHWDeviceType,
}

extern "C" {
    pub fn avcodec_find_encoder(id: AVCodecID) -> *const AVCodec;
    pub fn avcodec_find_decoder(id: AVCodecID) -> *const AVCodec;
    pub fn avcodec_find_encoder_by_name(name: *const c_char) -> *const AVCodec;
    pub fn avcodec_find_decoder_by_name(name: *const c_char) -> *const AVCodec;
    pub fn avcodec_get_hw_config(codec: *const AVCodec, index: i32) -> *const AVCodecHWConfig;
}
