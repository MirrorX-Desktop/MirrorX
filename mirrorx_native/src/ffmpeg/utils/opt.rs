use super::{channel_layout::AVChannelLayout, samplefmt::AVSampleFormat};
use crate::ffmpeg::utils::rational::AVRational;
use std::os::raw::{c_char, c_void};

pub type AVOptionType = u32;
pub const AV_OPT_TYPE_FLAGS: AVOptionType = 0;
pub const AV_OPT_TYPE_INT: AVOptionType = 1;
pub const AV_OPT_TYPE_INT64: AVOptionType = 2;
pub const AV_OPT_TYPE_DOUBLE: AVOptionType = 3;
pub const AV_OPT_TYPE_FLOAT: AVOptionType = 4;
pub const AV_OPT_TYPE_STRING: AVOptionType = 5;
pub const AV_OPT_TYPE_RATIONAL: AVOptionType = 6;
pub const AV_OPT_TYPE_BINARY: AVOptionType = 7;
pub const AV_OPT_TYPE_DICT: AVOptionType = 8;
pub const AV_OPT_TYPE_UINT64: AVOptionType = 9;
pub const AV_OPT_TYPE_CONST: AVOptionType = 10;
pub const AV_OPT_TYPE_IMAGE_SIZE: AVOptionType = 11;
pub const AV_OPT_TYPE_PIXEL_FMT: AVOptionType = 12;
pub const AV_OPT_TYPE_SAMPLE_FMT: AVOptionType = 13;
pub const AV_OPT_TYPE_VIDEO_RATE: AVOptionType = 14;
pub const AV_OPT_TYPE_DURATION: AVOptionType = 15;
pub const AV_OPT_TYPE_COLOR: AVOptionType = 16;
pub const AV_OPT_TYPE_CHANNEL_LAYOUT: AVOptionType = 17;
pub const AV_OPT_TYPE_BOOL: AVOptionType = 18;

#[repr(C)]
pub struct AVOption {
    pub name: *const c_char,
    pub help: *const c_char,
    pub offset: i32,
    pub type_: AVOptionType,
    pub default_val: AVOptionDefaultVal,
    pub min: f64,
    pub max: f64,
    pub flags: i32,
    pub unit: *const c_char,
}

#[repr(C)]
pub union AVOptionDefaultVal {
    i64_: i64,
    dbl: f64,
    str_: *const c_char,
    q: AVRational,
}

#[repr(C)]
pub struct AVOptionRange {
    pub str_: *const c_char,
    pub value_min: f64,
    pub value_max: f64,
    pub component_min: f64,
    pub component_max: f64,
    pub is_range: i32,
}

#[repr(C)]
pub struct AVOptionRanges {
    pub range: *mut *mut AVOptionRange,
    pub nb_ranges: i32,
    pub nb_components: i32,
}

extern "C" {
    pub fn av_opt_set(
        obj: *mut c_void,
        name: *const c_char,
        val: *const c_char,
        search_flags: i32,
    ) -> i32;
}

extern "C" {
    pub fn av_opt_set_chlayout(
        obj: *mut c_void,
        name: *const i8,
        layout: *const AVChannelLayout,
        search_flags: i32,
    ) -> i32;
    pub fn av_opt_set_int(obj: *mut c_void, name: *const i8, val: i64, search_flags: i32) -> i32;
    pub fn av_opt_set_sample_fmt(
        obj: *mut c_void,
        name: *const i8,
        fmt: AVSampleFormat,
        search_flags: i32,
    ) -> i32;
}
