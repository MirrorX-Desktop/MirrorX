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

pub type CMBlockBufferFlags = u32;

pub type CMItemCount = CFIndex;

extern "C" {
    pub static kCMSampleAttachmentKey_NotSync: CFStringRef;
    pub static kCMSampleAttachmentKey_DependsOnOthers: CFStringRef;
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
    pub fn CMSampleBufferCreateReady(
        allocator: CFAllocatorRef,
        data_buffer: CMBlockBufferRef,
        format_description: CMFormatDescriptionRef,
        num_samples: CMItemCount,
        num_sample_timing_entries: CMItemCount,
        sample_timing_array: *const CMSampleTimingInfo,
        num_sample_size_entries: CMItemCount,
        sample_size_array: *const isize,
        sample_buffer_out: *mut CMSampleBufferRef,
    ) -> OSStatus;
    pub fn CMSampleBufferGetSampleAttachmentsArray(
        sbuf: CMSampleBufferRef,
        create_if_necessary: bool,
    ) -> CFArrayRef;
    pub fn CMSampleBufferGetFormatDescription(sbuf: CMSampleBufferRef) -> CMFormatDescriptionRef;
    pub fn CMVideoFormatDescriptionGetH264ParameterSetAtIndex(
        video_desc: CMFormatDescriptionRef,
        parameter_set_index: isize,
        parameter_set_pointer_out: *mut *const u8,
        parameter_set_size_out: *mut isize,
        parameter_set_count_out: *mut isize,
        nal_unit_header_length_out: *mut i32,
    ) -> OSStatus;
    pub fn CMVideoFormatDescriptionCreateFromH264ParameterSets(
        allocator: CFAllocatorRef,
        parameter_set_count: isize,
        parameter_set_pointers: *const *const u8,
        parameter_set_sizes: *const isize,
        nal_unit_header_length: i32,
        format_description_out: *mut CMFormatDescriptionRef,
    ) -> OSStatus;
    pub fn CMSampleBufferGetDataBuffer(sbuf: CMSampleBufferRef) -> CMBlockBufferRef;
    pub fn CMBlockBufferGetDataPointer(
        the_buffer: CMBlockBufferRef,
        offset: u32,
        length_at_offset_out: *mut u32,
        total_length_out: *mut u32,
        data_pointer_out: *mut *const u8,
    ) -> OSStatus;
    pub fn CMBlockBufferCreateWithMemoryBlock(
        structure_allocator: CFAllocatorRef,
        memory_block: *mut c_void,
        block_length: isize,
        block_allocator: CFAllocatorRef,
        custom_block_source: *const c_void,
        offset_to_data: isize,
        data_length: isize,
        flags: CMBlockBufferFlags,
        block_buffer_out: *mut CMBlockBufferRef,
    ) -> OSStatus;
}
