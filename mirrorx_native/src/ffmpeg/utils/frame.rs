use super::{
    avutil::AVPictureType,
    buffer::AVBufferRef,
    channel_layout::AVChannelLayout,
    dict::AVDictionary,
    pixfmt::{
        AVChromaLocation, AVColorPrimaries, AVColorRange, AVColorSpace,
        AVColorTransferCharacteristic, AVPixelFormat,
    },
    rational::AVRational,
};
use std::ffi::c_void;

pub const AV_NUM_DATA_POINTERS: usize = 8;

pub type AVFrameSideDataType = u32;
pub const AV_FRAME_DATA_PANSCAN: AVFrameSideDataType = 0;
pub const AV_FRAME_DATA_A53_CC: AVFrameSideDataType = 1;
pub const AV_FRAME_DATA_STEREO3D: AVFrameSideDataType = 2;
pub const AV_FRAME_DATA_MATRIXENCODING: AVFrameSideDataType = 3;
pub const AV_FRAME_DATA_DOWNMIX_INFO: AVFrameSideDataType = 4;
pub const AV_FRAME_DATA_REPLAYGAIN: AVFrameSideDataType = 5;
pub const AV_FRAME_DATA_DISPLAYMATRIX: AVFrameSideDataType = 6;
pub const AV_FRAME_DATA_AFD: AVFrameSideDataType = 7;
pub const AV_FRAME_DATA_MOTION_VECTORS: AVFrameSideDataType = 8;
pub const AV_FRAME_DATA_SKIP_SAMPLES: AVFrameSideDataType = 9;
pub const AV_FRAME_DATA_AUDIO_SERVICE_TYPE: AVFrameSideDataType = 10;
pub const AV_FRAME_DATA_MASTERING_DISPLAY_METADATA: AVFrameSideDataType = 11;
pub const AV_FRAME_DATA_GOP_TIMECODE: AVFrameSideDataType = 12;
pub const AV_FRAME_DATA_SPHERICAL: AVFrameSideDataType = 13;
pub const AV_FRAME_DATA_CONTENT_LIGHT_LEVEL: AVFrameSideDataType = 14;
pub const AV_FRAME_DATA_ICC_PROFILE: AVFrameSideDataType = 15;
pub const AV_FRAME_DATA_S12M_TIMECODE: AVFrameSideDataType = 16;
pub const AV_FRAME_DATA_DYNAMIC_HDR_PLUS: AVFrameSideDataType = 17;
pub const AV_FRAME_DATA_REGIONS_OF_INTEREST: AVFrameSideDataType = 18;
pub const AV_FRAME_DATA_VIDEO_ENC_PARAMS: AVFrameSideDataType = 19;
pub const AV_FRAME_DATA_SEI_UNREGISTERED: AVFrameSideDataType = 20;
pub const AV_FRAME_DATA_FILM_GRAIN_PARAMS: AVFrameSideDataType = 21;
pub const AV_FRAME_DATA_DETECTION_BBOXES: AVFrameSideDataType = 22;
pub const AV_FRAME_DATA_DOVI_RPU_BUFFER: AVFrameSideDataType = 23;
pub const AV_FRAME_DATA_DOVI_METADATA: AVFrameSideDataType = 24;

#[repr(C)]
pub struct AVFrameSideData {
    pub typ: AVFrameSideDataType,
    pub data: *mut u8,
    pub size: usize,
    pub metadata: *mut AVDictionary,
    pub buf: *mut AVBufferRef,
}

#[repr(C)]
pub struct AVFrame {
    pub data: [*mut u8; AV_NUM_DATA_POINTERS],
    pub linesize: [i32; AV_NUM_DATA_POINTERS],
    pub extended_data: *mut *mut u8,
    pub width: i32,
    pub height: i32,
    pub nb_samples: i32,
    pub format: AVPixelFormat,
    pub key_frame: i32,
    pub pict_type: AVPictureType,
    pub sample_aspect_ratio: AVRational,
    pub pts: i64,
    pub pkt_pts: i64,
    pub time_base: AVRational,
    pub coded_picture_number: i32,
    pub display_picture_number: i32,
    pub quality: i32,
    pub opaque: *mut c_void,
    pub repeat_pict: i32,
    pub interlaced_frame: i32,
    pub top_field_first: i32,
    pub palette_has_changed: i32,
    pub reordered_opaque: i64,
    pub sample_rate: i32,
    pub channel_layout: u64,
    pub buf: [*mut AVBufferRef; AV_NUM_DATA_POINTERS],
    pub extended_buf: *mut *mut AVBufferRef,
    pub nb_extended_buf: i32,
    pub side_data: *mut *mut AVFrameSideData,
    pub nb_side_data: i32,
    pub flags: i32,
    pub color_range: AVColorRange,
    pub color_primaries: AVColorPrimaries,
    pub color_trc: AVColorTransferCharacteristic,
    pub color_space: AVColorSpace,
    pub chroma_location: AVChromaLocation,
    pub best_effort_timestamp: i64,
    pub pkt_pos: i64,
    pub pkt_duration: i64,
    pub metadata: *mut AVDictionary,
    pub decode_error_flags: i32,
    pub channels: i32,
    pub pkt_size: i32,
    pub hw_frames_ctx: *mut AVBufferRef,
    pub opaque_ref: *mut AVBufferRef,
    pub crop_top: usize,
    pub crop_bottom: usize,
    pub crop_left: usize,
    pub crop_right: usize,
    pub private_ref: *mut AVBufferRef,
    pub ch_layout: AVChannelLayout,
}

impl Drop for AVFrame {
    fn drop(&mut self) {
        unsafe { av_frame_free(&mut (self as *mut _)) }
    }
}

extern "C" {
    pub fn av_frame_free(frame: *mut *mut AVFrame);
    pub fn av_frame_alloc() -> *mut AVFrame;
    pub fn av_frame_get_buffer(frame: *mut AVFrame, align: i32) -> i32;
    pub fn av_frame_make_writable(frame: *mut AVFrame) -> i32;
    pub fn av_frame_unref(frame: *mut AVFrame) -> i32;
    pub fn get_audio_buffer(frame: *mut AVFrame, align: i32) -> i32;
}
