use std::os::raw::c_void;

use block::Block;
use core_foundation::{
    base::{CFTypeRef, OSStatus},
    dictionary::CFDictionaryRef,
    mach_port::CFAllocatorRef,
    string::CFStringRef,
};

use super::{
    core_media::{
        CMFormatDescriptionRef, CMSampleBufferRef, CMTime, CMVideoCodecType,
        CMVideoFormatDescriptionRef,
    },
    core_video::CVImageBufferRef,
};

pub type VTSessionRef = *mut c_void;
pub type VTCompressionSessionRef = VTSessionRef;
pub type VTDecompressionSessionRef = VTSessionRef;

pub type VTEncodeInfoFlags = u32;
pub const kVTEncodeInfo_Asynchronous: VTEncodeInfoFlags = 1 << 0;
pub const kVTEncodeInfo_FrameDropped: VTEncodeInfoFlags = 1 << 1;

pub type VTDecodeInfoFlags = u32;
pub const kVTDecodeInfo_Asynchronous: VTDecodeInfoFlags = 1 << 0;
pub const kVTDecodeInfo_FrameDropped: VTDecodeInfoFlags = 1 << 1;
pub const kVTDecodeInfo_ImageBufferModifiable: VTDecodeInfoFlags = 1 << 2;

pub type VTDecodeFrameFlags = u32;
pub const kVTDecodeFrame_EnableAsynchronousDecompression: VTDecodeFrameFlags = 1 << 0;
pub const kVTDecodeFrame_DoNotOutputFrame: VTDecodeFrameFlags = 1 << 1;
pub const kVTDecodeFrame_1xRealTimePlayback: VTDecodeFrameFlags = 1 << 2;
pub const kVTDecodeFrame_EnableTemporalProcessing: VTDecodeFrameFlags = 1 << 3;

pub type VTCompressionOutputCallback = extern "C" fn(
    output_callback_ref_con: *mut c_void,
    source_frame_ref_con: *mut c_void,
    status: OSStatus,
    info_flags: VTEncodeInfoFlags,
    sample_buffer: CMSampleBufferRef,
);

pub type VTDecompressionOutputCallback = extern "C" fn(
    decompressionOutputRefCon: *mut c_void,
    sourceFrameRefCon: *mut c_void,
    status: OSStatus,
    infoFlags: VTDecodeInfoFlags,
    imageBuffer: CVImageBufferRef,
    presentationTimeStamp: CMTime,
    presentationDuration: CMTime,
);

#[repr(C)]
pub struct VTDecompressionOutputCallbackRecord {
    pub decompression_output_callback: VTDecompressionOutputCallback,
    pub decompression_output_ref_con: *mut c_void,
}

extern "C" {
    pub static kVTProfileLevel_H264_Main_5_0: CFStringRef;
}

extern "C" {
    pub static kVTCompressionPropertyKey_ProfileLevel: CFStringRef;
    pub static kVTCompressionPropertyKey_RealTime: CFStringRef;
    pub static kVTCompressionPropertyKey_AllowFrameReordering: CFStringRef;
    pub static kVTCompressionPropertyKey_MaxKeyFrameInterval: CFStringRef;
    pub static kVTCompressionPropertyKey_ExpectedFrameRate: CFStringRef;
    pub static kVTCompressionPropertyKey_AverageBitRate: CFStringRef;
    pub static kVTCompressionPropertyKey_DataRateLimits: CFStringRef;
}

extern "C" {
    pub static kVTDecompressionPropertyKey_RealTime: CFStringRef;
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
    pub fn VTCompressionSessionCompleteFrames(
        session: VTCompressionSessionRef,
        complete_until_presentation_timestamp: CMTime,
    ) -> OSStatus;
    pub fn VTCompressionSessionInvalidate(session: VTCompressionSessionRef);
    pub fn VTDecompressionSessionInvalidate(session: VTDecompressionSessionRef);
    pub fn VTDecompressionSessionWaitForAsynchronousFrames(
        session: VTDecompressionSessionRef,
    ) -> OSStatus;
    pub fn VTDecompressionSessionCreate(
        allocator: CFAllocatorRef,
        video_format_description: CMVideoFormatDescriptionRef,
        video_decoder_specification: CFDictionaryRef,
        destination_image_buffer_attributes: CFDictionaryRef,
        output_callback: &VTDecompressionOutputCallbackRecord,
        decompression_session_out: *mut VTDecompressionSessionRef,
    ) -> OSStatus;
    pub fn VTDecompressionSessionCanAcceptFormatDescription(
        session: VTDecompressionSessionRef,
        new_format_desc: CMFormatDescriptionRef,
    ) -> bool;
    pub fn VTDecompressionSessionDecodeFrame(
        session: VTDecompressionSessionRef,
        sample_buffer: CMSampleBufferRef,
        decode_flags: VTDecodeFrameFlags,
        source_frame_ref_con: *mut c_void,
        info_flags_out: *mut VTDecodeInfoFlags,
    ) -> OSStatus;
}
