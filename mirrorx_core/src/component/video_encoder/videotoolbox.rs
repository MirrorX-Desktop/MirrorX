use crate::{
    component::desktop::CaptureFrame,
    error::MirrorXError,
    ffi::os::macos::{core_media::*, videotoolbox::*},
    service::endpoint::message::{
        EndPointMessage, EndPointMessagePacket, EndPointMessagePacketType, VideoFrame,
    },
};
use bytes::Buf;
use core_foundation::{
    array::{CFArray, CFArrayGetValueAtIndex},
    base::{CFRelease, OSStatus, ToVoid},
    dictionary::{CFDictionaryContainsKey, CFDictionaryRef},
    number::{kCFBooleanFalse, kCFBooleanTrue, CFNumber},
};
use scopeguard::defer;
use std::os::raw::c_void;

const NAL_UNIT_HEADER_LENGTH: usize = 4;

pub struct Encoder {
    session: VTCompressionSessionRef,
}

unsafe impl Send for Encoder {}

impl Encoder {
    pub fn new(frame_width: i32, frame_height: i32) -> Result<Encoder, MirrorXError> {
        unsafe {
            let session = create_compression_session(frame_width, frame_height)?;
            Ok(Encoder { session })
        }
    }

    pub fn encode(
        &self,
        capture_frame: CaptureFrame,
        endpoint_message_tx: *mut tokio::sync::mpsc::Sender<EndPointMessagePacket>,
    ) -> Result<(), MirrorXError> {
        unsafe {
            let ret = VTCompressionSessionEncodeFrame(
                self.session,
                capture_frame.pixel_buffer,
                capture_frame.pts.clone(),
                CMTime::invalid(),
                std::ptr::null_mut(),
                endpoint_message_tx as *mut c_void,
                std::ptr::null_mut(),
            );

            if ret != 0 {
                return Err(MirrorXError::Other(anyhow::anyhow!(
                    "VTCompressionSessionEncodeFrame failed ({})",
                    ret
                )));
            }

            Ok(())
        }
    }
}

unsafe fn create_compression_session(
    frame_width: i32,
    frame_height: i32,
) -> Result<VTCompressionSessionRef, MirrorXError> {
    let mut session = std::ptr::null_mut();
    let mut ret = VTCompressionSessionCreate(
        std::ptr::null_mut(),
        frame_width,
        frame_height,
        kCMVideoCodecType_H264,
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        encode_output_callback,
        std::ptr::null_mut(),
        &mut session,
    );

    if ret != 0 {
        return Err(MirrorXError::Other(anyhow::anyhow!(
            "VTCompressionSessionCreate failed ({})",
            ret
        )));
    }

    ret = VTSessionSetProperty(
        session,
        kVTCompressionPropertyKey_ProfileLevel,
        kVTProfileLevel_H264_Main_5_0.to_void(),
    );

    if ret != 0 {
        return Err(MirrorXError::Other(anyhow::anyhow!(
            "VTSessionSetProperty failed ({}) key={} value={}",
            ret,
            "kVTCompressionPropertyKey_ProfileLevel",
            "kVTProfileLevel_H264_Main_5_0"
        )));
    }

    ret = VTSessionSetProperty(
        session,
        kVTCompressionPropertyKey_RealTime,
        kCFBooleanTrue.to_void(),
    );

    if ret != 0 {
        return Err(MirrorXError::Other(anyhow::anyhow!(
            "VTSessionSetProperty failed ({}) key={} value={}",
            ret,
            "kVTCompressionPropertyKey_RealTime",
            "kCFBooleanTrue"
        )));
    }

    ret = VTSessionSetProperty(
        session,
        kVTCompressionPropertyKey_AllowFrameReordering,
        kCFBooleanFalse.to_void(),
    );

    if ret != 0 {
        return Err(MirrorXError::Other(anyhow::anyhow!(
            "VTSessionSetProperty failed ({}) key={} value={}",
            ret,
            "kVTCompressionPropertyKey_AllowFrameReordering",
            "kCFBooleanFalse"
        )));
    }

    ret = VTSessionSetProperty(
        session,
        kVTCompressionPropertyKey_AllowFrameReordering,
        kCFBooleanFalse.to_void(),
    );

    if ret != 0 {
        return Err(MirrorXError::Other(anyhow::anyhow!(
            "VTSessionSetProperty failed ({}) key={} value={}",
            ret,
            "kVTCompressionPropertyKey_AllowFrameReordering",
            "kCFBooleanFalse"
        )));
    }

    ret = VTSessionSetProperty(
        session,
        kVTCompressionPropertyKey_MaxKeyFrameInterval,
        CFNumber::from(120).to_void(),
    );

    if ret != 0 {
        return Err(MirrorXError::Other(anyhow::anyhow!(
            "VTSessionSetProperty failed ({}) key={} value={}",
            ret,
            "kVTCompressionPropertyKey_MaxKeyFrameInterval",
            "120"
        )));
    }

    ret = VTSessionSetProperty(
        session,
        kVTCompressionPropertyKey_ExpectedFrameRate,
        CFNumber::from(60).to_void(),
    );

    if ret != 0 {
        return Err(MirrorXError::Other(anyhow::anyhow!(
            "VTSessionSetProperty failed ({}) key={} value={}",
            ret,
            "kVTCompressionPropertyKey_ExpectedFrameRate",
            "60"
        )));
    }

    ret = VTSessionSetProperty(
        session,
        kVTCompressionPropertyKey_AverageBitRate,
        CFNumber::from(4000 * 1000).to_void(),
    );

    if ret != 0 {
        return Err(MirrorXError::Other(anyhow::anyhow!(
            "VTSessionSetProperty failed ({}) key={} value={}",
            ret,
            "kVTCompressionPropertyKey_AverageBitRate",
            "4000_0000"
        )));
    }

    ret = VTSessionSetProperty(
        session,
        kVTCompressionPropertyKey_DataRateLimits,
        CFArray::from_CFTypes(&[CFNumber::from(4000 * 1000), CFNumber::from(1)])
            .into_untyped()
            .to_void(),
    );

    if ret != 0 {
        return Err(MirrorXError::Other(anyhow::anyhow!(
            "VTSessionSetProperty failed ({}) key={} value={}",
            ret,
            "kVTCompressionPropertyKey_DataRateLimits",
            "[4000_0000,1]"
        )));
    }

    ret = VTCompressionSessionPrepareToEncodeFrames(session);

    if ret != 0 {
        return Err(MirrorXError::Other(anyhow::anyhow!(
            "VTCompressionSessionPrepareToEncodeFrames failed ({})",
            ret,
        )));
    }

    Ok(session)
}

fn encode_output_callback(
    _: *mut c_void,
    source_frame_ref_con: *mut c_void,
    status: OSStatus,
    info_flags: VTEncodeInfoFlags,
    sample_buffer: CMSampleBufferRef,
) {
    unsafe {
        if status != 0 {
            tracing::error!(?status, "compression wasn't successful");
            return;
        }

        if info_flags & kVTEncodeInfo_FrameDropped != 0 {
            tracing::warn!("compression dropped frame");
            return;
        }

        if sample_buffer.is_null() {
            tracing::error!("CMSampleBufferRef from output callback is null");
            return;
        }

        if !CMSampleBufferDataIsReady(sample_buffer) {
            tracing::error!("CMSampleBufferRef from output is not ready");
            return;
        }

        let endpoint_message_tx =
            source_frame_ref_con as *mut tokio::sync::mpsc::Sender<EndPointMessagePacket>;

        if endpoint_message_tx.is_null() {
            tracing::error!("transmute source_frame_ref_con to *mut tokio::sync::mpsc::Sender<EndPointMessagePacket> is null");
            return;
        }

        let attachments = CMSampleBufferGetSampleAttachmentsArray(sample_buffer, true);
        if attachments.is_null() {
            tracing::error!("CMSampleBufferGetSampleAttachmentsArray returns null CFArrayRef");
            return;
        }

        // defer! {
        //     CFRelease(attachments.to_void());
        // }

        let dic = CFArrayGetValueAtIndex(attachments, 0);

        let is_key_frame = CFDictionaryContainsKey(
            dic as CFDictionaryRef,
            kCMSampleAttachmentKey_NotSync.to_void(),
        ) == 0;

        let mut sps = once_cell::unsync::OnceCell::new();
        let mut pps = once_cell::unsync::OnceCell::new();

        if is_key_frame {
            let format = CMSampleBufferGetFormatDescription(sample_buffer);
            if format.is_null() {
                tracing::error!(
                    "CMSampleBufferGetFormatDescription returns null CMFormatDescriptionRef"
                );
                return;
            }

            // get SPS

            let mut sps_ptr = std::ptr::null();
            let mut sps_size = 0u32;

            let ret = CMVideoFormatDescriptionGetH264ParameterSetAtIndex(
                format,
                0,
                &mut sps_ptr,
                &mut sps_size,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            );

            if ret != 0 {
                tracing::error!(
                    "CMVideoFormatDescriptionGetH264ParameterSetAtIndex index=0 failed ({})",
                    ret
                );
                return;
            }

            // get PPS

            let mut pps_ptr = std::ptr::null();
            let mut pps_size = 0u32;

            let ret = CMVideoFormatDescriptionGetH264ParameterSetAtIndex(
                format,
                1,
                &mut pps_ptr,
                &mut pps_size,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            );

            if ret != 0 {
                tracing::error!(
                    "CMVideoFormatDescriptionGetH264ParameterSetAtIndex index=1 failed ({})",
                    ret
                );
                return;
            }

            let _ = sps.set(std::slice::from_raw_parts(sps_ptr, sps_size as usize).to_vec());
            let _ = pps.set(std::slice::from_raw_parts(pps_ptr, pps_size as usize).to_vec());
        }

        let data_buffer = CMSampleBufferGetDataBuffer(sample_buffer);
        if data_buffer.is_null() {
            tracing::error!("CMSampleBufferGetDataBuffer returns null CMBlockBufferRef");
            return;
        }

        let mut total_length = 0u32;
        let mut data_pointer = std::ptr::null();

        let ret = CMBlockBufferGetDataPointer(
            data_buffer,
            0,
            std::ptr::null_mut(),
            &mut total_length,
            &mut data_pointer,
        );

        if ret != 0 {
            tracing::error!("CMBlockBufferGetDataPointer failed ({})", ret);
            return;
        }

        let mut offset = 0;

        while offset + NAL_UNIT_HEADER_LENGTH < total_length as usize {
            let nalu_header_slice =
                std::slice::from_raw_parts(data_pointer.add(offset), NAL_UNIT_HEADER_LENGTH);

            let nalu_header_bytes: [u8; NAL_UNIT_HEADER_LENGTH] = [
                nalu_header_slice[0],
                nalu_header_slice[1],
                nalu_header_slice[2],
                nalu_header_slice[3],
            ];

            // NAL Unit length is Big Endian format
            let nalu_body_length = u32::from_be_bytes(nalu_header_bytes) as usize;

            let nalu_body_bytes = std::slice::from_raw_parts(
                data_pointer.add(offset + NAL_UNIT_HEADER_LENGTH),
                nalu_body_length,
            )
            .to_vec();

            // todo send sps pps nal_unit_bytes
            if let Err(err) = (*endpoint_message_tx).try_send(EndPointMessagePacket {
                typ: EndPointMessagePacketType::Push,
                call_id: None,
                message: EndPointMessage::VideoFrame(VideoFrame {
                    sps: sps.take(),
                    pps: pps.take(),
                    buffer: nalu_body_bytes,
                }),
            }) {
                tracing::warn!("send message 'VideoFrame' failed ({})", err);
            }

            offset += NAL_UNIT_HEADER_LENGTH + nalu_body_length
        }
    }
}
