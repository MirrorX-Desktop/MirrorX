use super::frame::DecodedFrame;
use crate::{
    error::MirrorXError,
    ffi::ffmpeg::{avcodec::*, avutil::*},
};
use anyhow::anyhow;
use std::{
    collections::HashMap,
    ffi::{CStr, CString},
    ptr,
};
use tracing::{error, info, warn};

pub struct VideoDecoder {
    codec: *const AVCodec,
    codec_ctx: *mut AVCodecContext,
    parser_ctx: *mut AVCodecParserContext,
    packet: *mut AVPacket,
    decode_frame: *mut AVFrame,
    hw_decode_frame: *mut AVFrame,
}

unsafe impl Send for VideoDecoder {}
unsafe impl Sync for VideoDecoder {}

impl VideoDecoder {
    pub fn new(
        decoder_name: &str,
        options: HashMap<&str, &str>,
    ) -> Result<VideoDecoder, MirrorXError> {
        let decoder_name_ptr =
            CString::new(decoder_name).map_err(|err| MirrorXError::Other(anyhow!(err)))?;

        unsafe {
            av_log_set_level(AV_LOG_TRACE);
            av_log_set_flags(AV_LOG_SKIP_REPEATED);

            let mut decoder = VideoDecoder {
                codec: ptr::null(),
                codec_ctx: ptr::null_mut(),
                parser_ctx: ptr::null_mut(),
                packet: ptr::null_mut(),
                decode_frame: ptr::null_mut(),
                hw_decode_frame: ptr::null_mut(),
            };

            let mut support_hw_device_type = AV_HWDEVICE_TYPE_NONE;
            loop {
                support_hw_device_type = av_hwdevice_iterate_types(support_hw_device_type);
                if support_hw_device_type == AV_HWDEVICE_TYPE_NONE {
                    break;
                }

                let support_hw_device_name = av_hwdevice_get_type_name(support_hw_device_type);
                match CStr::from_ptr(support_hw_device_name).to_str() {
                    Ok(name) => info!(device=?name,"support hw device name"),
                    Err(err) => warn!(err=?err,"convert hw device name from C str failed"),
                }
            }

            decoder.codec = avcodec_find_decoder_by_name(decoder_name_ptr.as_ptr());
            if decoder.codec.is_null() {
                return Err(MirrorXError::MediaVideoDecoderNotFound(
                    decoder_name.to_string(),
                ));
            }

            decoder.codec_ctx = avcodec_alloc_context3(decoder.codec);
            if decoder.codec_ctx.is_null() {
                return Err(MirrorXError::MediaVideoDecoderAllocContextFailed);
            }

            (*decoder.codec_ctx).width = 1920;
            (*decoder.codec_ctx).height = 1080;
            (*decoder.codec_ctx).pix_fmt = AV_PIX_FMT_NV12;
            (*decoder.codec_ctx).color_range = AVCOL_RANGE_JPEG;
            (*decoder.codec_ctx).color_primaries = AVCOL_PRI_BT709;
            (*decoder.codec_ctx).color_trc = AVCOL_TRC_BT709;
            (*decoder.codec_ctx).colorspace = AVCOL_SPC_BT709;
            (*decoder.codec_ctx).flags |= AV_CODEC_FLAG_LOW_DELAY;

            for (k, v) in options {
                Self::set_opt(decoder.codec_ctx, k, v, 0)?;
            }

            decoder.packet = av_packet_alloc();
            if decoder.packet.is_null() {
                return Err(MirrorXError::MediaVideoDecoderAVPacketAllocFailed);
            }

            decoder.decode_frame = av_frame_alloc();
            if decoder.decode_frame.is_null() {
                return Err(MirrorXError::MediaVideoDecoderAVFrameAllocFailed);
            }

            let hw_config = avcodec_get_hw_config(decoder.codec, 0);
            if hw_config.is_null() {
                decoder.parser_ctx = av_parser_init((*decoder.codec).id);
                if decoder.parser_ctx.is_null() {
                    return Err(MirrorXError::MediaVideoDecoderParserInitFailed);
                }
            } else {
                let mut hwdevice_ctx = ptr::null_mut();

                let device = if cfg!(target_os = "windows") {
                    CString::new("auto")
                        .map_err(|err| MirrorXError::Other(anyhow::anyhow!(err)))?
                        .as_ptr()
                } else {
                    ptr::null()
                };

                let ret = av_hwdevice_ctx_create(
                    &mut hwdevice_ctx,
                    (*hw_config).device_type,
                    device,
                    ptr::null_mut(),
                    0,
                );

                if ret < 0 {
                    return Err(MirrorXError::MediaVideoDecoderHWDeviceCreateFailed(ret));
                }

                (*decoder.codec_ctx).hw_device_ctx = av_buffer_ref(hwdevice_ctx);

                decoder.hw_decode_frame = av_frame_alloc();
                if decoder.hw_decode_frame.is_null() {
                    return Err(MirrorXError::MediaVideoDecoderHWAVFrameAllocFailed);
                }
            }

            let ret = avcodec_open2(decoder.codec_ctx, decoder.codec, ptr::null_mut());
            if ret != 0 {
                return Err(MirrorXError::MediaVideoDecoderOpenFailed(ret));
            }

            Ok(decoder)
        }
    }

    pub fn set_opt(
        codec_ctx: *mut AVCodecContext,
        key: &str,
        value: &str,
        search_flags: i32,
    ) -> Result<(), MirrorXError> {
        let opt_name =
            CString::new(key.to_string()).map_err(|err| MirrorXError::Other(anyhow!(err)))?;
        let opt_value =
            CString::new(value.to_string()).map_err(|err| MirrorXError::Other(anyhow!(err)))?;

        unsafe {
            let ret = av_opt_set(
                (*codec_ctx).priv_data,
                opt_name.as_ptr(),
                opt_value.as_ptr(),
                search_flags,
            );

            if ret == AVERROR_OPTION_NOT_FOUND {
                return Err(MirrorXError::MediaVideoDecoderOptionNotFound(
                    key.to_string(),
                ));
            } else if ret == AVERROR(libc::ERANGE) {
                return Err(MirrorXError::MediaVideoDecoderOptionValueOutOfRange {
                    key: key.to_string(),
                    value: value.to_string(),
                });
            } else if ret == AVERROR(libc::EINVAL) {
                return Err(MirrorXError::MediaVideoDecoderOptionValueInvalid {
                    key: key.to_string(),
                    value: value.to_string(),
                });
            } else if ret != 0 {
                return Err(MirrorXError::MediaVideoDecoderOptionSetFailed {
                    key: key.to_string(),
                    value: value.to_string(),
                    error_code: ret,
                });
            } else {
                Ok(())
            }
        }
    }

    pub fn decode(
        &self,
        mut data: Vec<u8>,
        dts: i64,
        pts: i64,
    ) -> Result<Vec<DecodedFrame>, MirrorXError> {
        unsafe {
            if !self.parser_ctx.is_null() {
                let ret = av_parser_parse2(
                    self.parser_ctx,
                    self.codec_ctx,
                    &mut (*self.packet).data,
                    &mut (*self.packet).size,
                    data.as_ptr(),
                    data.len() as i32,
                    pts,
                    dts,
                    0,
                );

                if ret < 0 {
                    return Err(MirrorXError::MediaVideoDecoderParser2Failed(ret));
                }
            } else {
                (*self.packet).data = data.as_mut_ptr();
                (*self.packet).size = data.len() as i32;
                (*self.packet).pts = pts;
                (*self.packet).dts = dts;
            }

            let mut ret = avcodec_send_packet(self.codec_ctx, self.packet);

            if ret == AVERROR(libc::EAGAIN) {
                return Err(MirrorXError::MediaVideoDecoderPacketUnacceptable);
            } else if ret == AVERROR_EOF {
                return Err(MirrorXError::MediaVideoDecoderClosed);
            } else if ret < 0 {
                error!(size = data.len(), "packet size");
                return Err(MirrorXError::MediaVideoDecoderSendPacketFailed(ret));
            }

            let mut frames = Vec::new();
            loop {
                ret = avcodec_receive_frame(self.codec_ctx, self.decode_frame);
                if ret == AVERROR(libc::EAGAIN) || ret == AVERROR_EOF {
                    return Ok(frames);
                } else if ret < 0 {
                    return Err(MirrorXError::MediaVideoDecoderReceiveFrameFailed(ret));
                }

                if !self.parser_ctx.is_null() {
                } else {
                    match self.send_native_frame() {
                        Ok(frame) => frames.push(frame),
                        Err(err) => return Err(err),
                    };
                }

                av_frame_unref((*self).decode_frame);
            }
        }
    }

    #[cfg(target_os = "macos")]
    unsafe fn send_native_frame(&self) -> Result<DecodedFrame, MirrorXError> {
        use crate::ffi::os::{CVPixelBufferRef, CVPixelBufferRetain};

        let native_frame = CVPixelBufferRetain((*self.decode_frame).data[3] as CVPixelBufferRef);

        Ok(DecodedFrame(native_frame))
    }

    #[cfg(target_os = "windows")]
    unsafe fn send_native_frame(&self) -> Result<DecodedFrame, MirrorXError> {
        use crate::ffi::libyuv::*;

        let ret = av_hwframe_transfer_data(self.hw_decode_frame, self.decode_frame, 0);
        if ret < 0 {
            error!(ret = ret, "av_hwframe_transfer_data failed");
            return Err(MirrorXError::MediaVideoDecoderOutputTxSendFailed);
        }

        let abgr_frame_size = ((*self.hw_decode_frame).width as usize)
            * ((*self.hw_decode_frame).height as usize)
            * 4;

        let mut abgr_frame = Vec::<u8>::with_capacity(abgr_frame_size);

        // the actual AVFrame format is NV12, but in the libyuv, function 'NV12ToABGRMatrix' is a macro to function 'NV21ToARGBMatrix'
        // and Rust FFI can't convert macro so we directly use it's result function 'NV21ToARGBMatrix' and yuvconstants
        let ret = NV21ToARGBMatrix(
            (*self.hw_decode_frame).data[0],
            (*self.hw_decode_frame).linesize[0] as isize,
            (*self.hw_decode_frame).data[1],
            (*self.hw_decode_frame).linesize[1] as isize,
            abgr_frame.as_mut_ptr(),
            ((*self.hw_decode_frame).width as isize) * 4,
            &kYvuF709Constants,
            (*self.hw_decode_frame).width as isize,
            (*self.hw_decode_frame).height as isize,
        );

        if ret != 0 {
            return Err(MirrorXError::Other(anyhow::anyhow!(
                "libyuv::NV21ToARGBMatrix returns {}",
                ret
            )));
        }

        abgr_frame.set_len(abgr_frame_size);

        Ok(DecodedFrame(abgr_frame))
    }
}

impl Drop for VideoDecoder {
    fn drop(&mut self) {
        unsafe {
            if !self.codec_ctx.is_null() {
                avcodec_send_packet(self.codec_ctx, ptr::null());
            }

            if !self.hw_decode_frame.is_null() {
                av_frame_free(&mut self.hw_decode_frame);
            }

            if !self.parser_ctx.is_null() {
                av_parser_close(self.parser_ctx);
            }

            if !self.decode_frame.is_null() {
                av_frame_free(&mut self.decode_frame);
            }

            if !self.packet.is_null() {
                av_packet_free(&mut self.packet);
            }

            if !self.codec_ctx.is_null() {
                if !(*self.codec_ctx).hw_device_ctx.is_null() {
                    av_buffer_ref((*self.codec_ctx).hw_device_ctx);
                }
                avcodec_free_context(&mut self.codec_ctx);
            }
        }
    }
}
