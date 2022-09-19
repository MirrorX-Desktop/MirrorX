use crate::{
    api::endpoint::{handlers::handshake::EndPointMediaMessage, message::EndPointVideoFrame},
    component::NALU_HEADER_LENGTH,
    core_error,
    error::{CoreError, CoreResult},
    ffi::os::macos::{core_media::*, core_video::*, videotoolbox::*},
};
use core_foundation::{
    base::{kCFAllocatorDefault, kCFAllocatorNull, CFRelease, OSStatus, ToVoid},
    boolean::CFBoolean,
    dictionary::{
        kCFTypeDictionaryKeyCallBacks, kCFTypeDictionaryValueCallBacks, CFDictionaryCreate,
    },
    mach_port::CFIndex,
    number::{kCFBooleanTrue, CFNumber},
};
use flutter_rust_bridge::{StreamSink, ZeroCopyBuffer};
use scopeguard::defer;
use std::os::raw::c_void;

pub struct Decoder {
    format_description: CMVideoFormatDescriptionRef,
    session: VTDecompressionSessionRef,
    stream: StreamSink<EndPointMediaMessage>,
}

unsafe impl Send for Decoder {}

impl Decoder {
    pub fn new(stream: StreamSink<EndPointMediaMessage>) -> Self {
        Decoder {
            format_description: std::ptr::null_mut(),
            session: std::ptr::null_mut(),
            stream,
        }
    }

    pub fn decode(&mut self, mut video_frame: EndPointVideoFrame) -> CoreResult<()> {
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
                return Err(core_error!("decompression session is null"));
            }

            if self.format_description.is_null() {
                return Err(core_error!("decompression format description is null"));
            }

            let nalu_header_bytes =
                ((video_frame.buffer.len() - NALU_HEADER_LENGTH) as u32).to_be_bytes();

            video_frame.buffer[0] = nalu_header_bytes[0];
            video_frame.buffer[1] = nalu_header_bytes[1];
            video_frame.buffer[2] = nalu_header_bytes[2];
            video_frame.buffer[3] = nalu_header_bytes[3];

            let mut block_buffer = std::ptr::null_mut();
            let ret = CMBlockBufferCreateWithMemoryBlock(
                kCFAllocatorDefault,
                video_frame.buffer.as_ptr() as *mut c_void,
                video_frame.buffer.len() as isize,
                kCFAllocatorNull,
                std::ptr::null(),
                0,
                video_frame.buffer.len() as isize,
                0,
                &mut block_buffer,
            );

            if ret != 0 {
                return Err(core_error!(
                    "CMBlockBufferCreateWithMemoryBlock returns error code: {}",
                    ret
                ));
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
                [video_frame.buffer.len() as isize].as_ptr(),
                &mut sample_buffer,
            );

            if ret != 0 {
                return Err(core_error!(
                    "CMSampleBufferCreateReady returns error code: {}",
                    ret
                ));
            }

            let ret = VTDecompressionSessionDecodeFrame(
                self.session,
                sample_buffer,
                kVTDecodeFrame_EnableAsynchronousDecompression,
                &mut self.stream as *mut _ as *mut c_void,
                std::ptr::null_mut(), // todo: pass frame dropped to statistic
            );

            if ret != 0 {
                return Err(core_error!(
                    "VTDecompressionSessionDecodeFrame returns error code: {}",
                    ret
                ));
            }

            Ok(())
        }
    }
}

unsafe fn create_format_description(sps: &[u8], pps: &[u8]) -> CoreResult<CMFormatDescriptionRef> {
    let parameter_set = [sps.as_ptr(), pps.as_ptr()];
    let parameter_set_size = [sps.len() as isize, pps.len() as isize];

    let mut format_description = std::ptr::null_mut();
    let ret = CMVideoFormatDescriptionCreateFromH264ParameterSets(
        kCFAllocatorDefault,
        2,
        parameter_set.as_ptr(),
        parameter_set_size.as_ptr(),
        4,
        &mut format_description,
    );

    if ret != 0 {
        return Err(core_error!(
            "CMVideoFormatDescriptionCreateFromH264ParameterSets returns error code: {}",
            ret
        ));
    }

    Ok(format_description)
}

unsafe fn create_decompression_session(
    format_description: CMVideoFormatDescriptionRef,
) -> CoreResult<VTDecompressionSessionRef> {
    let keys = [
        kCVPixelBufferPixelFormatTypeKey.to_void(),
        kCVPixelBufferMetalCompatibilityKey.to_void(),
        kCVPixelBufferOpenGLCompatibilityKey.to_void(),
    ];

    let values = [
        CFNumber::from(kCVPixelFormatType_32BGRA as i64).to_void(),
        CFBoolean::true_value().to_void(),
        CFBoolean::true_value().to_void(),
    ];

    let destination_pixel_buffer_attributes = CFDictionaryCreate(
        kCFAllocatorDefault,
        keys.as_ptr(),
        values.as_ptr(),
        keys.len() as CFIndex,
        &kCFTypeDictionaryKeyCallBacks,
        &kCFTypeDictionaryValueCallBacks,
    );

    defer! {
        CFRelease(destination_pixel_buffer_attributes.to_void());
    }

    let output_callback = VTDecompressionOutputCallbackRecord {
        decompression_output_callback: decode_output_callback,
        decompression_output_ref_con: std::ptr::null_mut(),
    };

    let mut session = std::ptr::null_mut();
    let ret = VTDecompressionSessionCreate(
        kCFAllocatorDefault,
        format_description,
        std::ptr::null(),
        destination_pixel_buffer_attributes,
        &output_callback,
        &mut session,
    );

    if ret != 0 {
        return Err(core_error!(
            "VTDecompressionSessionCreate returns error code: {}",
            ret
        ));
    }

    let ret = VTSessionSetProperty(
        session,
        kVTDecompressionPropertyKey_RealTime,
        kCFBooleanTrue.to_void(),
    );

    if ret != 0 {
        return Err(core_error!(
            "VTSessionSetProperty returns error code: {}",
            ret,
        ));
    }

    Ok(session)
}

unsafe extern "C" fn decode_output_callback(
    decompressionOutputRefCon: *mut c_void,
    sourceFrameRefCon: *mut c_void,
    status: OSStatus,
    infoFlags: VTDecodeInfoFlags,
    imageBuffer: CVImageBufferRef,
    presentationTimeStamp: CMTime,
    presentationDuration: CMTime,
) {
    if status != 0 {
        tracing::error!(?status, "status is not ok");
        return;
    }

    if sourceFrameRefCon.is_null() {
        tracing::error!("sourceFrameRefCon is null");
        return;
    }

    if imageBuffer.is_null() {
        tracing::error!("imageBuffer is null");
        return;
    }

    let tx = sourceFrameRefCon as *mut StreamSink<EndPointMediaMessage>;
    let pixel_buffer = CVPixelBufferRetain(imageBuffer);
    let pixel_buffer_memory_address: usize = std::mem::transmute(pixel_buffer);

    let success = (*tx).add(EndPointMediaMessage::Video(
        1,
        1,
        ZeroCopyBuffer(pixel_buffer_memory_address.to_le_bytes().to_vec()),
    ));

    if !success {
        CVPixelBufferRelease(pixel_buffer);
        tracing::error!("send EndPointMediaMessage failed");
    }
}
