use super::opt::{AVOption, AVOptionRanges};
use std::os::raw::{c_char, c_void};

pub const AV_LOG_QUIET: i32 = -8;
pub const AV_LOG_PANIC: i32 = 0;
pub const AV_LOG_FATAL: i32 = 8;
pub const AV_LOG_ERROR: i32 = 16;
pub const AV_LOG_WARNING: i32 = 24;
pub const AV_LOG_INFO: i32 = 32;
pub const AV_LOG_VERBOSE: i32 = 40;
pub const AV_LOG_DEBUG: i32 = 48;
pub const AV_LOG_TRACE: i32 = 56;

pub const AV_LOG_SKIP_REPEATED: i32 = 1;
pub const AV_LOG_PRINT_LEVEL: i32 = 2;

pub type AVClassCategory = u32;
pub const AV_CLASS_CATEGORY_NA: AVClassCategory = 0;
pub const AV_CLASS_CATEGORY_INPUT: AVClassCategory = 1;
pub const AV_CLASS_CATEGORY_OUTPUT: AVClassCategory = 2;
pub const AV_CLASS_CATEGORY_MUXER: AVClassCategory = 3;
pub const AV_CLASS_CATEGORY_DEMUXER: AVClassCategory = 4;
pub const AV_CLASS_CATEGORY_ENCODER: AVClassCategory = 5;
pub const AV_CLASS_CATEGORY_DECODER: AVClassCategory = 6;
pub const AV_CLASS_CATEGORY_FILTER: AVClassCategory = 7;
pub const AV_CLASS_CATEGORY_BITSTREAM_FILTER: AVClassCategory = 8;
pub const AV_CLASS_CATEGORY_SWSCALER: AVClassCategory = 9;
pub const AV_CLASS_CATEGORY_SWRESAMPLER: AVClassCategory = 10;
pub const AV_CLASS_CATEGORY_DEVICE_VIDEO_OUTPUT: AVClassCategory = 40;
pub const AV_CLASS_CATEGORY_DEVICE_VIDEO_INPUT: AVClassCategory = 41;
pub const AV_CLASS_CATEGORY_DEVICE_AUDIO_OUTPUT: AVClassCategory = 42;
pub const AV_CLASS_CATEGORY_DEVICE_AUDIO_INPUT: AVClassCategory = 43;
pub const AV_CLASS_CATEGORY_DEVICE_OUTPUT: AVClassCategory = 44;
pub const AV_CLASS_CATEGORY_DEVICE_INPUT: AVClassCategory = 45;

#[repr(C)]
pub struct AVClass {
    pub class_name: *const c_char,
    pub item_name: extern "C" fn(ctx: *mut c_void) -> *const c_char,
    pub option: *const AVOption,
    pub version: i32,
    pub log_level_offset_offset: i32,
    pub parent_log_context_offset: i32,
    pub category: AVClassCategory,
    pub get_category: extern "C" fn(ctx: *mut c_void) -> AVClassCategory,
    pub query_ranges: extern "C" fn(
        *mut *mut AVOptionRanges,
        obj: *mut c_void,
        key: *const c_char,
        flags: i32,
    ) -> i32,
    pub child_next: extern "C" fn(obj: *mut c_void, prev: *mut c_void) -> *mut c_void,
    pub child_class_iterate: extern "C" fn(iter: *mut *mut c_void) -> *mut AVClass,
}

extern "C" {
    pub fn av_log_set_level(level: i32);
    pub fn av_log_get_level() -> i32;
    pub fn av_log_set_flags(arg: i32);
    pub fn av_log_get_flags() -> i32;

    // pub fn av_log_set_callback(
    //     callback: extern "C" fn(avcl: *mut c_void, level: i32, fmt: *const c_char, vl: VaList),
    // );
}
