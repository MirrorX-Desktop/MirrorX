use crate::ffmpeg::utils::{buffer::AVBufferRef, rational::AVRational};
use std::ffi::c_void;

pub type AVPacketSideDataType = u32;
pub const AV_PKT_DATA_PALETTE: AVPacketSideDataType = 0;
pub const AV_PKT_DATA_NEW_EXTRADATA: AVPacketSideDataType = 1;
pub const AV_PKT_DATA_PARAM_CHANGE: AVPacketSideDataType = 2;
pub const AV_PKT_DATA_H263_MB_INFO: AVPacketSideDataType = 3;
pub const AV_PKT_DATA_REPLAYGAIN: AVPacketSideDataType = 4;
pub const AV_PKT_DATA_DISPLAYMATRIX: AVPacketSideDataType = 5;
pub const AV_PKT_DATA_STEREO3D: AVPacketSideDataType = 6;
pub const AV_PKT_DATA_AUDIO_SERVICE_TYPE: AVPacketSideDataType = 7;
pub const AV_PKT_DATA_QUALITY_STATS: AVPacketSideDataType = 8;
pub const AV_PKT_DATA_FALLBACK_TRACK: AVPacketSideDataType = 9;
pub const AV_PKT_DATA_CPB_PROPERTIES: AVPacketSideDataType = 10;
pub const AV_PKT_DATA_SKIP_SAMPLES: AVPacketSideDataType = 11;
pub const AV_PKT_DATA_JP_DUALMONO: AVPacketSideDataType = 12;
pub const AV_PKT_DATA_STRINGS_METADATA: AVPacketSideDataType = 13;
pub const AV_PKT_DATA_SUBTITLE_POSITION: AVPacketSideDataType = 14;
pub const AV_PKT_DATA_MATROSKA_BLOCKADDITIONAL: AVPacketSideDataType = 15;
pub const AV_PKT_DATA_WEBVTT_IDENTIFIER: AVPacketSideDataType = 16;
pub const AV_PKT_DATA_WEBVTT_SETTINGS: AVPacketSideDataType = 17;
pub const AV_PKT_DATA_METADATA_UPDATE: AVPacketSideDataType = 18;
pub const AV_PKT_DATA_MPEGTS_STREAM_ID: AVPacketSideDataType = 19;
pub const AV_PKT_DATA_MASTERING_DISPLAY_METADATA: AVPacketSideDataType = 20;
pub const AV_PKT_DATA_SPHERICAL: AVPacketSideDataType = 21;
pub const AV_PKT_DATA_CONTENT_LIGHT_LEVEL: AVPacketSideDataType = 22;
pub const AV_PKT_DATA_A53_CC: AVPacketSideDataType = 23;
pub const AV_PKT_DATA_ENCRYPTION_INIT_INFO: AVPacketSideDataType = 24;
pub const AV_PKT_DATA_ENCRYPTION_INFO: AVPacketSideDataType = 25;
pub const AV_PKT_DATA_AFD: AVPacketSideDataType = 26;
pub const AV_PKT_DATA_PRFT: AVPacketSideDataType = 27;
pub const AV_PKT_DATA_ICC_PROFILE: AVPacketSideDataType = 28;
pub const AV_PKT_DATA_DOVI_CONF: AVPacketSideDataType = 29;
pub const AV_PKT_DATA_S12M_TIMECODE: AVPacketSideDataType = 30;
pub const AV_PKT_DATA_DYNAMIC_HDR10_PLUS: AVPacketSideDataType = 31;

#[repr(C)]
pub struct AVPacketSideData {
    pub data: *mut u8,
    pub size: usize,
    pub typ: AVPacketSideDataType,
}

#[repr(C)]
pub struct AVPacket {
    pub buf: *mut AVBufferRef,
    pub pts: i64,
    pub dts: i64,
    pub data: *mut u8,
    pub size: i32,
    pub stream_index: i32,
    pub flags: i32,
    pub side_data: *mut AVPacketSideData,
    pub side_data_elems: i32,
    pub duration: i64,
    pub pos: i64,
    pub opaque: *mut c_void,
    pub opaque_ref: *mut AVBufferRef,
    pub time_base: AVRational,
}

impl Drop for AVPacket {
    fn drop(&mut self) {
        unsafe { av_packet_free(&mut (self as *mut _)) }
    }
}

extern "C" {
    pub fn av_packet_free(pkt: *mut *mut AVPacket);
    pub fn av_packet_alloc() -> *mut AVPacket;
    pub fn av_new_packet(pkt: *mut AVPacket, size: i32) -> i32;
    pub fn av_packet_unref(pkt: *mut AVPacket);
    pub fn av_packet_rescale_ts(pkt: *mut AVPacket, tb_src: AVRational, tb_dst: AVRational);
}
