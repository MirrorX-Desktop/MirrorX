use super::core_video::CVImageBufferRef;
use core_foundation::{array::CFIndex, base::OSStatus, mach_port::CFAllocatorRef};
use std::os::raw::c_void;

pub type CMVideoFormatDescriptionRef = *mut c_void;
pub type CMSampleBufferRef = *mut c_void;

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
    pub fn CMTimeMake(value: i64, time_scale: i32) -> CMTime;
    pub fn CMTimeMakeWithSeconds(seconds: f64, preferred_timescale: i32) -> CMTime;
    pub fn CMSampleBufferIsValid(sample_buffer: CMSampleBufferRef) -> bool;
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
}
