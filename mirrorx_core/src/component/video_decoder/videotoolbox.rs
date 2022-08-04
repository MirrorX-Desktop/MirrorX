use std::os::raw::c_void;

use crate::{
    error::MirrorXError,
    ffi::os::macos::{core_media::*, core_video::*, videotoolbox::*},
    service::endpoint::message::VideoFrame,
};
use core_foundation::{
    base::{kCFAllocatorDefault, kCFAllocatorNull, CFRelease, OSStatus, ToVoid},
    dictionary::{CFDictionary, CFDictionaryCreate, CFMutableDictionary},
    mach_port::CFIndex,
    number::{kCFBooleanFalse, kCFBooleanTrue, CFNumber},
    string::CFStringRef,
};
use hmac::digest::block_buffer::BlockBuffer;
use scopeguard::defer;

pub struct Decoder {
    format_description: CMVideoFormatDescriptionRef,
    session: VTDecompressionSessionRef,
}

unsafe impl Send for Decoder {}

impl Decoder {
    pub fn new() -> Self {
        Decoder {
            format_description: std::ptr::null_mut(),
            session: std::ptr::null_mut(),
        }
    }

    pub fn decode(&mut self, mut video_frame: VideoFrame) -> Result<(), MirrorXError> {
        unsafe {
            if let (Some(sps), Some(pps)) = (video_frame.sps, video_frame.pps) {
                let format_description = create_format_description(&sps, &pps)?;

                if self.session.is_null() {
                    self.session = create_decompression_session(format_description)?;
                } else if !VTDecompressionSessionCanAcceptFormatDescription(
                    self.session,
                    format_description,
                ) {
                    VTDecompressionSessionWaitForAsynchronousFrames(self.session);
                    VTDecompressionSessionInvalidate(self.session);
                    self.session = create_decompression_session(format_description)?;
                }

                self.format_description = format_description;
            }

            if self.session.is_null() {
                return Err(MirrorXError::Other(anyhow::anyhow!(
                    "decompression session is null"
                )));
            }

            if self.format_description.is_null() {
                return Err(MirrorXError::Other(anyhow::anyhow!(
                    "decompression format description is null"
                )));
            }

            let mut nalu_bytes = (video_frame.buffer.len() as u32).to_be_bytes().to_vec();
            nalu_bytes.append(&mut video_frame.buffer);

            let mut block_buffer = std::ptr::null_mut();
            let ret = CMBlockBufferCreateWithMemoryBlock(
                kCFAllocatorDefault,
                nalu_bytes.as_mut_ptr() as *mut c_void,
                nalu_bytes.len() as u32,
                kCFAllocatorNull,
                std::ptr::null(),
                0,
                nalu_bytes.len() as u32,
                0,
                &mut block_buffer,
            );

            if ret != 0 {
                return Err(MirrorXError::Other(anyhow::anyhow!(
                    "CMBlockBufferCreateWithMemoryBlock failed ({})",
                    ret
                )));
            }

            let mut sample_buffer = std::ptr::null_mut();
            let ret = CMSampleBufferCreateReady(
                kCFAllocatorDefault,
                block_buffer,
                self.format_description,
                1,
                0,
                std::ptr::null(),
                1,
                [nalu_bytes.len() as u32].as_ptr(),
                &mut sample_buffer,
            );

            if ret != 0 {
                return Err(MirrorXError::Other(anyhow::anyhow!(
                    "CMSampleBufferCreateReady failed ({})",
                    ret
                )));
            }

            let ret = VTDecompressionSessionDecodeFrame(
                self.session,
                sample_buffer,
                kVTDecodeFrame_EnableAsynchronousDecompression,
                std::ptr::null_mut(),
                std::ptr::null_mut(), // todo: pass frame dropped to statistic
            );

            if ret != 0 {
                return Err(MirrorXError::Other(anyhow::anyhow!(
                    "VTDecompressionSessionDecodeFrame failed ({})",
                    ret
                )));
            }

            Ok(())
        }
    }
}

unsafe fn create_format_description(
    sps: &[u8],
    pps: &[u8],
) -> Result<CMFormatDescriptionRef, MirrorXError> {
    let parameter_set_ptr = [sps.as_ptr(), pps.as_ptr()].as_ptr();
    let parameter_set_size_ptr = [sps.len() as u32, pps.len() as u32].as_ptr();

    let mut format_description = std::ptr::null_mut();
    let ret = CMVideoFormatDescriptionCreateFromH264ParameterSets(
        kCFAllocatorDefault,
        2,
        parameter_set_ptr,
        parameter_set_size_ptr,
        4,
        &mut format_description,
    );

    if ret != 0 {
        return Err(MirrorXError::Other(anyhow::anyhow!(
            "CMVideoFormatDescriptionCreateFromH264ParameterSets failed ({})",
            ret
        )));
    }

    Ok(format_description)
}

unsafe fn create_decompression_session(
    format_description: CMVideoFormatDescriptionRef,
) -> Result<VTDecompressionSessionRef, MirrorXError> {
    let desitination_pixel_buffer_attributes = CFDictionaryCreate(
        kCFAllocatorDefault,
        [kCVPixelBufferPixelFormatTypeKey.to_void()].as_ptr(),
        [&kCVPixelFormatType_32BGRA as *const _ as *const c_void].as_ptr(),
        1,
        std::ptr::null(),
        std::ptr::null(),
    );

    defer! {
        CFRelease(desitination_pixel_buffer_attributes.to_void());
    }

    let output_callback = VTDecompressionOutputCallbackRecord {
        decompression_output_callback: decode_output_callback,
        decompression_output_ref_con: todo!(),
    };

    let mut session = std::ptr::null_mut();
    let ret = VTDecompressionSessionCreate(
        kCFAllocatorDefault,
        format_description,
        std::ptr::null(),
        desitination_pixel_buffer_attributes,
        &output_callback,
        &mut session,
    );

    if ret != 0 {
        return Err(MirrorXError::Other(anyhow::anyhow!(
            "VTDecompressionSessionCreate failed ({})",
            ret
        )));
    }

    let ret = VTSessionSetProperty(
        session,
        kVTDecompressionPropertyKey_RealTime,
        kCFBooleanTrue.to_void(),
    );

    if ret != 0 {
        return Err(MirrorXError::Other(anyhow::anyhow!(
            "VTSessionSetProperty failed ({}) key={} value={}",
            ret,
            "kVTDecompressionPropertyKey_RealTime",
            "true"
        )));
    }

    Ok(session)
}

extern "C" fn decode_output_callback(
    decompressionOutputRefCon: *mut c_void,
    sourceFrameRefCon: *mut c_void,
    status: OSStatus,
    infoFlags: VTDecodeInfoFlags,
    imageBuffer: CVImageBufferRef,
    presentationTimeStamp: CMTime,
    presentationDuration: CMTime,
) {
    tracing::info!(?presentationTimeStamp, "cmtime");
}
