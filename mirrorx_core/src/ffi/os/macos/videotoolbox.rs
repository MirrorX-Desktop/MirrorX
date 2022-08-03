use std::os::raw::c_void;

use block::Block;
use core_foundation::{
    base::{CFTypeRef, OSStatus},
    dictionary::CFDictionaryRef,
    mach_port::CFAllocatorRef,
    string::CFStringRef,
};

use super::{
    core_media::{CMSampleBufferRef, CMTime, CMVideoCodecType},
    core_video::CVImageBufferRef,
};

pub type VTSessionRef = *mut c_void;
pub type VTCompressionSessionRef = VTSessionRef;

pub type VTEncodeInfoFlags = u32;
pub const kVTEncodeInfo_Asynchronous: VTEncodeInfoFlags = 1 << 0;
pub const kVTEncodeInfo_FrameDropped: VTEncodeInfoFlags = 1 << 1;

pub type VTCompressionOutputCallback = fn(
    *mut c_void,       // output_callback_ref_con
    *mut c_void,       // source_frame_ref_con
    OSStatus,          // status
    VTEncodeInfoFlags, // info_flags
    CMSampleBufferRef, // sample_buffer
);

extern "C" {
    pub static kVTCompressionPropertyKey_ProfileLevel: CFStringRef;
    pub static kVTCompressionPropertyKey_RealTime: CFStringRef;
    pub static kVTCompressionPropertyKey_AllowFrameReordering: CFStringRef;
    pub static kVTCompressionPropertyKey_MaxKeyFrameInterval: CFStringRef;
    pub static kVTCompressionPropertyKey_ExpectedFrameRate: CFStringRef;
    pub static kVTCompressionPropertyKey_AverageBitRate: CFStringRef;
    pub static kVTCompressionPropertyKey_DataRateLimits: CFStringRef;

    pub static kVTProfileLevel_H264_Main_5_0: CFStringRef;
}

extern "C" {
    pub fn VTCompressionSessionCreate(
        allocator: CFAllocatorRef,
        width: i32,
        height: i32,
        codec_type: CMVideoCodecType,
        encoder_specification: CFDictionaryRef,
        source_image_buffer_attributes: CFDictionaryRef,
        compressed_data_allocator: CFAllocatorRef,
        output_callback: VTCompressionOutputCallback,
        output_callback_ref_con: *mut c_void,
        compression_session_out: *mut VTCompressionSessionRef,
    ) -> OSStatus;
    pub fn VTSessionSetProperty(
        session: VTSessionRef,
        property_key: CFStringRef,
        property_value: CFTypeRef,
    ) -> OSStatus;
    pub fn VTCompressionSessionPrepareToEncodeFrames(session: VTCompressionSessionRef) -> OSStatus;
    pub fn VTCompressionSessionEncodeFrame(
        session: VTCompressionSessionRef,
        image_buffer: CVImageBufferRef,
        presentation_timestamp: CMTime,
        duration: CMTime,
        frame_properties: CFDictionaryRef,
        source_frame_ref_con: *mut c_void,
        info_flags_out: *mut VTEncodeInfoFlags,
    ) -> OSStatus;
}
