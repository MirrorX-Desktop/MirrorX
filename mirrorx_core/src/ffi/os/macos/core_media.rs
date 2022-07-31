use std::os::raw::c_void;

use core_foundation::{array::CFIndex, base::OSStatus};

use super::core_video::CVImageBufferRef;

pub type CMSampleBufferRef = *mut c_void;

pub type CMTimeValue = i64;
pub type CMTimeScale = i32;
pub type CMTimeEpoch = i64;
pub type CMTimeFlags = u32;

#[repr(C)]
#[derive(Debug)]
pub struct CMTime {
    pub value: CMTimeValue,
    pub time_scale: CMTimeScale,
    pub flags: CMTimeFlags,
    pub epoch: CMTimeEpoch,
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
    pub fn CMSampleBufferIsValid(sample_buffer: CMSampleBufferRef) -> bool;
    pub fn CMSampleBufferGetSampleTimingInfo(
        sample_buffer: CMSampleBufferRef,
        sampleIndex: CFIndex,
        timing_info_out: *mut CMSampleTimingInfo,
    ) -> OSStatus;
    pub fn CMSampleBufferGetImageBuffer(sample_buffer: CMSampleBufferRef) -> CVImageBufferRef;
}
