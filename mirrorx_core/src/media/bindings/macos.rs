use core_foundation::{array::CFIndex, base::CFTypeRef, string::CFStringRef};
use std::{ffi::c_void, os::raw::c_int};

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

#[allow(non_upper_case_globals)]
pub static kCVPixelFormatType_420YpCbCr8BiPlanarVideoRange: i32 = 875704438;

#[allow(non_upper_case_globals)]
pub static kCVPixelFormatType_420YpCbCr8BiPlanarFullRange: i32 = 875704422;

#[allow(non_upper_case_globals)]
pub static kCVPixelFormatType_32BGRA: i32 = 1111970369;

pub type CMSampleBufferRef = *mut c_void;
pub type CVImageBufferRef = *mut c_void;
pub type CVPixelBufferRef = CVImageBufferRef;

#[repr(C)]
#[derive(Debug)]
pub struct CMSampleTimingInfo {
    pub duration: CMTime,
    pub presentation_timestamp: CMTime,
    pub decode_timestamp: CMTime,
}

extern "C" {
    pub static kCVPixelBufferPixelFormatTypeKey: CFStringRef;
    pub static kCVPixelBufferWidthKey: CFStringRef;
    pub static kCVPixelBufferHeightKey: CFStringRef;
    pub static kCVImageBufferYCbCrMatrixKey: CFStringRef;

    pub static kCVImageBufferYCbCrMatrix_ITU_R_601_4: CFStringRef;
    pub static kCVImageBufferYCbCrMatrix_ITU_R_709_2: CFStringRef;
    pub static kCVImageBufferYCbCrMatrix_ITU_R_2020: CFStringRef;
    pub static kCVImageBufferYCbCrMatrix_SMPTE_240M_1995: CFStringRef;

    pub fn CMTimeMake(value: i64, time_scale: i32) -> CMTime;

    pub fn CMSampleBufferIsValid(sample_buffer: CMSampleBufferRef) -> bool;
    pub fn CMSampleBufferGetSampleTimingInfo(
        sample_buffer: CMSampleBufferRef,
        sampleIndex: CFIndex,
        timing_info_out: *mut CMSampleTimingInfo,
    ) -> c_int;
    pub fn CMSampleBufferGetImageBuffer(sample_buffer: CMSampleBufferRef) -> CVImageBufferRef;
    pub fn CVPixelBufferGetPixelFormatType(pixel_buffer: CVPixelBufferRef) -> u32;
    pub fn CVPixelBufferLockBaseAddress(pixel_buffer: CVPixelBufferRef, lock_flags: u32) -> i32;
    pub fn CVPixelBufferUnlockBaseAddress(pixel_buffer: CVPixelBufferRef, unlock_flags: u32)
        -> i32;
    pub fn CVPixelBufferGetWidth(pixel_buffer: CVPixelBufferRef) -> isize;
    pub fn CVPixelBufferGetHeight(pixel_buffer: CVPixelBufferRef) -> isize;
    pub fn CVPixelBufferGetBytesPerRowOfPlane(
        pixel_buffer: CVPixelBufferRef,
        planeIndex: u32,
    ) -> u32;
    pub fn CVPixelBufferGetBaseAddressOfPlane(
        pixel_buffer: CVPixelBufferRef,
        planeIndex: u32,
    ) -> *mut c_void;
    pub fn CVPixelBufferGetHeightOfPlane(pixel_buffer: CVPixelBufferRef, planeIndex: u32) -> u32;
    pub fn CVPixelBufferRetain(texture: CVPixelBufferRef) -> CVPixelBufferRef;
    pub fn CVPixelBufferRelease(texture: CVPixelBufferRef);

    pub fn CVBufferGetAttachment(
        buffer: *mut c_void,
        key: CFStringRef,
        attachmentMode: *mut c_void,
    ) -> CFTypeRef;
}
