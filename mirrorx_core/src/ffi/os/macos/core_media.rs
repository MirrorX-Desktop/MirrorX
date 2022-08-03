use super::{core_video::CVImageBufferRef, four_char_code};
use core_foundation::{
    array::{CFArrayRef, CFIndex},
    base::OSStatus,
    mach_port::CFAllocatorRef,
    string::CFStringRef,
};
use std::os::raw::c_void;

pub type CMVideoCodecType = u32;
pub const kCMVideoCodecType_H264: CMVideoCodecType = four_char_code('a', 'v', 'c', '1');

pub type CMFormatDescriptionRef = *mut c_void;
pub type CMVideoFormatDescriptionRef = CMFormatDescriptionRef;
pub type CMSampleBufferRef = *mut c_void;
pub type CMBlockBufferRef = *mut c_void;

pub type CMTimeValue = i64;
pub type CMTimeScale = i32;
pub type CMTimeEpoch = i64;
pub type CMTimeFlags = u32;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct CMTime {
    pub value: CMTimeValue,
    pub time_scale: CMTimeScale,
    pub flags: CMTimeFlags,
    pub epoch: CMTimeEpoch,
}

impl CMTime {
    pub fn invalid() -> CMTime {
        CMTime {
            value: 0,
            time_scale: 0,
            flags: 0,
            epoch: 0,
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CMSampleTimingInfo {
    pub duration: CMTime,
    pub presentation_timestamp: CMTime,
    pub decode_timestamp: CMTime,
}

extern "C" {
    pub static kCMSampleAttachmentKey_NotSync: CFStringRef;
}

extern "C" {
    pub fn CMTimeMake(value: i64, time_scale: i32) -> CMTime;
    pub fn CMTimeMakeWithSeconds(seconds: f64, preferred_timescale: i32) -> CMTime;
    pub fn CMSampleBufferIsValid(sample_buffer: CMSampleBufferRef) -> bool;
    pub fn CMSampleBufferDataIsReady(sample_buffer: CMSampleBufferRef) -> bool;
    pub fn CMSampleBufferGetSampleTimingInfo(
        sample_buffer: CMSampleBufferRef,
        sampleIndex: CFIndex,
        timing_info_out: *mut CMSampleTimingInfo,
    ) -> OSStatus;
    pub fn CMSampleBufferGetImageBuffer(sample_buffer: CMSampleBufferRef) -> CVImageBufferRef;
    pub fn CMVideoFormatDescriptionCreateForImageBuffer(
        allocator: CFAllocatorRef,
        image_buffer: CVImageBufferRef,
        format_description_out: *mut CMVideoFormatDescriptionRef,
    ) -> OSStatus;
    pub fn CMSampleBufferCreateReadyWithImageBuffer(
        allocator: CFAllocatorRef,
        image_buffer: CVImageBufferRef,
        format_description: CMVideoFormatDescriptionRef,
        sample_timing: *const CMSampleTimingInfo,
        sample_buffer_out: *mut CMSampleBufferRef,
    ) -> OSStatus;
    pub fn CMSampleBufferGetSampleAttachmentsArray(
        sbuf: CMSampleBufferRef,
        create_if_necessary: bool,
    ) -> CFArrayRef;
    pub fn CMSampleBufferGetFormatDescription(sbuf: CMSampleBufferRef) -> CMFormatDescriptionRef;
    pub fn CMVideoFormatDescriptionGetH264ParameterSetAtIndex(
        video_desc: CMFormatDescriptionRef,
        parameter_set_index: u32,
        parameter_set_pointer_out: *mut *const u8,
        parameter_set_size_out: *mut u32,
        parameter_set_count_out: *mut u32,
        nal_unit_header_length_out: *mut isize,
    ) -> OSStatus;
    pub fn CMSampleBufferGetDataBuffer(sbuf: CMSampleBufferRef) -> CMBlockBufferRef;
    pub fn CMBlockBufferGetDataPointer(
        the_buffer: CMBlockBufferRef,
        offset: u32,
        length_at_offset_out: *mut u32,
        total_length_out: *mut u32,
        data_pointer_out: *mut *const u8,
    ) -> OSStatus;
}
