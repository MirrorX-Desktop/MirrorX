use super::{codec::AVCodec, codec_id::AVCodecID, codec_par::AVFieldOrder};
use crate::ffmpeg::{
    codecs::{
        codec_desc::AVCodecDescriptor,
        defs::{AVAudioServiceType, AVDiscard},
        packet::{AVPacket, AVPacketSideData},
    },
    utils::{
        avutil::AVMediaType,
        buffer::AVBufferRef,
        channel_layout::AVChannelLayout,
        dict::AVDictionary,
        frame::{AVFrame, AV_NUM_DATA_POINTERS},
        log::AVClass,
        pixfmt::{
            AVChromaLocation, AVColorPrimaries, AVColorRange, AVColorSpace,
            AVColorTransferCharacteristic, AVPixelFormat,
        },
        rational::AVRational,
        samplefmt::AVSampleFormat,
    },
};
use std::{ffi::c_void, os::raw::c_char};

pub const AV_PARSER_PTS_NB: usize = 4;

pub type AVPictureStructure = u32;
pub const AV_PICTURE_STRUCTURE_UNKNOWN: AVPictureStructure = 0; //< unknown
pub const AV_PICTURE_STRUCTURE_TOP_FIELD: AVPictureStructure = 1; //< coded as top field
pub const AV_PICTURE_STRUCTURE_BOTTOM_FIELD: AVPictureStructure = 2; //< coded as bottom field
pub const AV_PICTURE_STRUCTURE_FRAME: AVPictureStructure = 3; //< coded as frame

pub type AVSubtitleType = u32;
pub const SUBTITLE_NONE: AVSubtitleType = 0;
pub const SUBTITLE_BITMAP: AVSubtitleType = 1;
pub const SUBTITLE_TEXT: AVSubtitleType = 2;
pub const SUBTITLE_ASS: AVSubtitleType = 3;

pub const AV_CODEC_FLAG2_LOCAL_HEADER: i32 = 1 << 3;
pub const AV_CODEC_FLAG_LOW_DELAY: i32 = 1 << 19;
pub const AV_CODEC_FLAG_GLOBAL_HEADER: i32 = 1 << 22;

pub const FF_PROFILE_H264_CONSTRAINED: i32 = 1 << 9; // 8+1; constraint_set1_flag
pub const FF_PROFILE_H264_INTRA: i32 = 1 << 11; // 8+3; constraint_set3_flag
pub const FF_PROFILE_H264_BASELINE: i32 = 66;
pub const FF_PROFILE_H264_CONSTRAINED_BASELINE: i32 = 66 | FF_PROFILE_H264_CONSTRAINED;
pub const FF_PROFILE_H264_MAIN: i32 = 77;
pub const FF_PROFILE_H264_EXTENDED: i32 = 88;
pub const FF_PROFILE_H264_HIGH: i32 = 100;
pub const FF_PROFILE_H264_HIGH_10: i32 = 110;
pub const FF_PROFILE_H264_HIGH_10_INTRA: i32 = 110 | FF_PROFILE_H264_INTRA;
pub const FF_PROFILE_H264_MULTIVIEW_HIGH: i32 = 118;
pub const FF_PROFILE_H264_HIGH_422: i32 = 122;
pub const FF_PROFILE_H264_HIGH_422_INTRA: i32 = 122 | FF_PROFILE_H264_INTRA;
pub const FF_PROFILE_H264_STEREO_HIGH: i32 = 128;
pub const FF_PROFILE_H264_HIGH_444: i32 = 144;
pub const FF_PROFILE_H264_HIGH_444_PREDICTIVE: i32 = 244;
pub const FF_PROFILE_H264_HIGH_444_INTRA: i32 = 244 | FF_PROFILE_H264_INTRA;
pub const FF_PROFILE_H264_CAVLC_444: i32 = 44;

#[repr(C)]
pub struct AVSubtitleRect {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
    pub nb_colors: i32,
    pub data: [*mut u8; 4],
    pub linesize: [i32; 4],
    pub type_: AVSubtitleType,
    pub text: *mut c_char,
    pub ass: *mut c_char,
    pub flags: i32,
}

#[repr(C)]
pub struct AVSubtitle {
    pub format: u16,
    pub start_display_time: u32,
    pub end_display_time: u32,
    pub num_rects: u32,
    pub rects: *mut *mut AVSubtitleRect,
    pub pts: i64,
}

#[repr(C)]
pub struct RcOverride {
    pub start_frame: i32,
    pub end_frame: i32,
    pub qscale: i32,
    pub quality_factor: f32,
}

#[repr(C)]
pub struct AVCodecParserContext {
    pub priv_data: *mut c_void,
    pub parser: *const c_void,
    pub frame_offset: i64,
    pub cur_offset: i64,
    pub next_frame_offset: i64,
    pub pict_type: AVPixelFormat,
    pub repeat_pict: i32,
    pub pts: i64,
    pub dts: i64,
    pub last_pts: i64,
    pub last_dts: i64,
    pub fetch_timestamp: i32,
    pub cur_frame_start_index: i32,
    pub cur_frame_offset: [i64; AV_PARSER_PTS_NB],
    pub cur_frame_pts: [i64; AV_PARSER_PTS_NB],
    pub cur_frame_dts: [i64; AV_PARSER_PTS_NB],
    pub flags: i32,
    pub offset: i64,
    pub cur_frame_end: [i64; AV_PARSER_PTS_NB],
    pub key_frame: i32,
    pub dts_sync_point: i32,
    pub dts_ref_dts_delta: i32,
    pub pts_dts_delta: i32,
    pub cur_frame_pos: [i64; AV_PARSER_PTS_NB],
    pub pos: i64,
    pub last_pos: i64,
    pub duration: i32,
    pub field_order: AVFieldOrder,
    pub picture_structure: AVPictureStructure,
    pub output_picture_number: i32,
    pub width: i32,
    pub height: i32,
    pub coded_width: i32,
    pub coded_height: i32,
    pub format: i32,
}

pub enum AVCodecInternal {}

#[repr(C)]
pub struct AVCodecContext {
    pub av_class: *const AVClass,
    pub log_level_offset: i32,
    pub codec_type: AVMediaType,
    pub codec: *const c_void,
    pub codec_id: i32,
    pub codec_tag: u32,
    pub priv_data: *mut c_void,
    pub internal: *mut AVCodecInternal,
    pub opaque: *mut c_void,
    pub bit_rate: i64,
    pub bit_rate_tolerance: i32,
    pub global_quality: i32,
    pub compression_level: i32,
    pub flags: i32,
    pub flags2: i32,
    pub extradata: *mut u8,
    pub extradata_size: i32,
    pub time_base: AVRational,
    pub ticks_per_frame: i32,
    pub delay: i32,
    pub width: i32,
    pub height: i32,
    pub coded_width: i32,
    pub coded_height: i32,
    pub gop_size: i32,
    pub pix_fmt: AVPixelFormat,
    pub draw_horiz_band: extern "C" fn(
        s: *mut AVCodecContext,
        src: *const AVFrame,
        offset: [i32; AV_NUM_DATA_POINTERS],
        y: i32,
        type_: i32,
        height: i32,
    ),
    pub get_format:
        extern "C" fn(s: *mut AVCodecContext, fmt: *const AVPixelFormat) -> AVPixelFormat,
    pub max_b_frames: i32,
    pub b_quant_factor: f32,
    pub b_quant_offset: f32,
    pub has_b_frames: i32,
    pub i_quant_factor: f32,
    pub i_quant_offset: f32,
    pub lumi_masking: f32,
    pub temporal_cplx_masking: f32,
    pub spatial_cplx_masking: f32,
    pub p_masking: f32,
    pub dark_masking: f32,
    pub slice_count: i32,
    pub slice_offset: *mut i32,
    pub sample_aspect_ratio: AVRational,
    pub me_cmp: i32,
    pub me_sub_cmp: i32,
    pub mb_cmp: i32,
    pub ildct_cmp: i32,
    pub dia_size: i32,
    pub last_predictor_count: i32,
    pub me_pre_cmp: i32,
    pub pre_dia_size: i32,
    pub me_subpel_quality: i32,
    pub me_range: i32,
    pub slice_flags: i32,
    pub mb_decision: i32,
    pub intra_matrix: *mut u16,
    pub inter_matrix: *mut u16,
    pub intra_dc_precision: i32,
    pub skip_top: i32,
    pub skip_bottom: i32,
    pub mb_lmin: i32,
    pub mb_lmax: i32,
    pub bidir_refine: i32,
    pub keyint_min: i32,
    pub refs: i32,
    pub mv0_threshold: i32,
    pub color_primaries: AVColorPrimaries,
    pub color_trc: AVColorTransferCharacteristic,
    pub colorspace: AVColorSpace,
    pub color_range: AVColorRange,
    pub chroma_sample_location: AVChromaLocation,
    pub slices: i32,
    pub field_order: AVFieldOrder,
    pub sample_rate: i32,
    pub channels: i32,
    pub sample_fmt: AVSampleFormat,
    pub frame_size: i32,
    pub frame_number: i32,
    pub block_align: i32,
    pub cutoff: i32,
    pub channel_layout: u64,
    pub request_channel_layout: u64,
    pub audio_service_type: AVAudioServiceType,
    pub request_sample_fmt: AVSampleFormat,
    pub get_buffer2: extern "C" fn(s: *mut AVCodecContext, frame: *mut AVFrame, flags: i32) -> i32,
    pub qcompress: f32,
    pub qblur: f32,
    pub qmin: i32,
    pub qmax: i32,
    pub max_qdiff: i32,
    pub rc_buffer_size: i32,
    pub rc_override_count: i32,
    pub rc_override: *mut RcOverride,
    pub rc_max_rate: i64,
    pub rc_min_rate: i64,
    pub rc_max_available_vbv_use: f32,
    pub rc_min_vbv_overflow_use: f32,
    pub rc_initial_buffer_occupancy: i32,
    pub trellis: i32,
    pub stats_out: *mut c_char,
    pub stats_in: *mut c_char,
    pub workaround_bugs: i32,
    pub strict_std_compliance: i32,
    pub error_concealment: i32,
    pub debug: i32,
    pub err_recognition: i32,
    pub reordered_opaque: i64,
    pub hwaccel: *const AVHWAccel,
    pub hwaccel_context: *mut c_void,
    pub error: [u64; AV_NUM_DATA_POINTERS],
    pub dct_algo: i32,
    pub idct_algo: i32,
    pub bits_per_coded_sample: i32,
    pub bits_per_raw_sample: i32,
    pub lowres: i32,
    pub thread_count: i32,
    pub thread_type: i32,
    pub active_thread_type: i32,
    pub thread_safe_callbacks: i32, // use when LIBAVCODEC_VERSION_MAJOR < 60, current 59
    pub execute: extern "C" fn(
        c: *mut AVCodecContext,
        func: extern "C" fn(*mut AVCodecContext, *mut c_void),
        arg2: *mut c_void,
        ret: *mut i32,
        count: i32,
        size: i32,
    ) -> i32,
    pub execute2: extern "C" fn(
        c: *mut AVCodecContext,
        func: extern "C" fn(*mut AVCodecContext, *mut c_void, i32, i32),
        arg2: *mut c_void,
        ret: *mut i32,
        count: i32,
    ) -> i32,
    pub nsse_wight: i32,
    pub profile: i32,
    pub level: i32,
    pub skip_loop_filter: AVDiscard,
    pub skip_idct: AVDiscard,
    pub skip_frame: AVDiscard,
    pub subtitle_header: *mut u8,
    pub subtitle_header_size: i32,
    pub initial_padding: i32,
    pub framerate: AVRational,
    pub sw_pix_fmt: AVPixelFormat,
    pub pkt_timebase: AVRational,
    pub codec_descriptor: *const AVCodecDescriptor,
    pub pts_correction_num_faulty_pts: i64,
    pub pts_correction_num_faulty_dts: i64,
    pub pts_correction_last_pts: i64,
    pub pts_correction_last_dts: i64,
    pub sub_charenc: *mut c_char,
    pub sub_charenc_mode: i32,
    pub skip_alpha: i32,
    pub seek_preroll: i32,
    pub debug_mv: i32, // use when LIBAVCODEC_VERSION_MAJOR < 60, current 59
    pub chroma_intra_matrix: *mut u8,
    pub dump_separator: *mut u8,
    pub codec_whitelist: *mut c_char,
    pub properties: u32,
    pub coded_side_data: *mut AVPacketSideData,
    pub nb_codec_side_data: i32,
    pub hw_frames_ctx: *mut AVBufferRef,
    pub sub_text_format: i32, // use when LIBAVCODEC_VERSION_MAJOR < 60, current 59
    pub trailing_padding: i32,
    pub max_pixels: i64,
    pub hw_device_ctx: *mut AVBufferRef,
    pub hwaccel_flags: i32,
    pub apply_cropping: i32,
    pub extra_hw_frames: i32,
    pub discard_damaged_percentage: i32,
    pub max_samples: i64,
    pub export_side_data: i32,
    pub get_encode_buffer:
        extern "C" fn(s: *mut AVCodecContext, pkt: *mut AVPacket, flags: i32) -> i32,
    pub ch_layout: AVChannelLayout,
}

impl Drop for AVCodecContext {
    fn drop(&mut self) {
        unsafe { avcodec_free_context(&mut (self as *mut _)) }
    }
}

pub enum MpegEncContext {}

#[repr(C)]
pub struct AVHWAccel {
    pub name: *const c_char,
    pub type_: AVMediaType,
    pub id: AVCodecID,
    pub pix_fmt: AVPixelFormat,
    pub capabilities: i32,
    pub alloc_frame: extern "C" fn(avctx: *mut AVCodecContext, frame: *mut AVFrame) -> i32,
    pub start_frame:
        extern "C" fn(avctx: *mut AVCodecContext, buf: *const u8, buf_size: u32) -> i32,
    pub decode_params:
        extern "C" fn(avctx: *mut AVCodecContext, type_: i32, buf: *const u8, buf_size: u32) -> i32,
    pub decode_slice:
        extern "C" fn(avctx: *mut AVCodecContext, buf: *const u8, buf_size: u32) -> i32,
    pub end_frame: extern "C" fn(avctx: *mut AVCodecContext) -> i32,
    pub frame_priv_data_size: i32,
    pub decode_mb: extern "C" fn(s: *mut MpegEncContext),
    pub init: extern "C" fn(avctx: *mut AVCodecContext) -> i32,
    pub uninit: extern "C" fn(avctx: *mut AVCodecContext) -> i32,
    pub priv_data_size: i32,
    pub caps_internal: i32,
    pub frame_params:
        extern "C" fn(avctx: *mut AVCodecContext, hw_frames_ctx: *mut AVBufferRef) -> i32,
}

extern "C" {
    pub fn avcodec_receive_packet(avctx: *mut AVCodecContext, avpkt: *mut AVPacket) -> i32;
    pub fn avcodec_send_frame(avctx: *mut AVCodecContext, frame: *mut AVFrame) -> i32;
    pub fn avcodec_send_packet(avctx: *mut AVCodecContext, avpkt: *const AVPacket) -> i32;
    pub fn avcodec_receive_frame(avctx: *mut AVCodecContext, frame: *mut AVFrame) -> i32;
    pub fn av_parser_parse2(
        s: *mut AVCodecParserContext,
        avctx: *mut AVCodecContext,
        poutbuf: *mut *mut u8,
        poutbuf_size: *mut i32,
        buf: *const u8,
        buf_size: i32,
        pts: i64,
        dts: i64,
        pos: i64,
    ) -> i32;
    pub fn av_parser_close(s: *mut AVCodecParserContext);
    pub fn avcodec_alloc_context3(codec: *const AVCodec) -> *mut AVCodecContext;
    pub fn avcodec_open2(
        avctx: *mut AVCodecContext,
        codec: *const AVCodec,
        options: *mut *mut AVDictionary,
    ) -> i32;
    pub fn avcodec_free_context(avctx: *mut *mut AVCodecContext);
    pub fn av_parser_init(codec_id: i32) -> *mut AVCodecParserContext;
}
