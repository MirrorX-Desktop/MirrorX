use crate::error::MirrorXError;
use crate::media::{
    ffmpeg::{
        avcodec::{
            avcodec::{
                av_parser_close, av_parser_init, av_parser_parse2, avcodec_alloc_context3,
                avcodec_free_context, avcodec_open2, avcodec_receive_frame, avcodec_send_packet,
                AVCodecContext, AVCodecParserContext,
            },
            codec::{avcodec_find_decoder_by_name, avcodec_get_hw_config, AVCodec},
            packet::{av_packet_alloc, av_packet_free, av_packet_unref, AVPacket},
        },
        avutil::{
            error::{AVERROR, AVERROR_EOF},
            frame::{av_frame_alloc, av_frame_free, AVFrame},
            hwcontext::{
                av_hwdevice_ctx_create, av_hwdevice_get_type_name, av_hwdevice_iterate_types,
                AV_HWDEVICE_TYPE_NONE,
            },
            log::{av_log_set_flags, av_log_set_level, AV_LOG_SKIP_REPEATED, AV_LOG_TRACE},
            pixfmt::{AVCOL_RANGE_JPEG, AV_PIX_FMT_NV12},
        },
    },
    frame::NativeFrame,
};
use anyhow::anyhow;
use crossbeam::channel::{bounded, Receiver, Sender};
use scopeguard::defer;
use std::{
    ffi::{CStr, CString},
    ptr,
};
use tracing::{error, info, warn};

use super::ffmpeg::avutil::buffer::av_buffer_ref;

use super::ffmpeg::avutil::error::AVERROR_OPTION_NOT_FOUND;
use super::ffmpeg::avutil::opt::av_opt_set;

pub struct VideoDecoder {
    codec: *const AVCodec,
    codec_ctx: *mut AVCodecContext,
    parser_ctx: *mut AVCodecParserContext,
    packet: *mut AVPacket,
    decode_frame: *mut AVFrame,
    hw_decode_frame: *mut AVFrame,
    output_tx: Option<Sender<NativeFrame>>,
}

unsafe impl Send for VideoDecoder {}
unsafe impl Sync for VideoDecoder {}

impl VideoDecoder {
    pub fn new(decoder_name: &str) -> Result<VideoDecoder, MirrorXError> {
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
                output_tx: None,
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
            (*decoder.codec_ctx).coded_width = 1920;
            (*decoder.codec_ctx).coded_height = 1080;
            (*decoder.codec_ctx).pix_fmt = AV_PIX_FMT_NV12;
            // (*decoder.codec_ctx).flags |= AV_CODEC_FLAG2_LOCAL_HEADER;
            (*decoder.codec_ctx).color_range = AVCOL_RANGE_JPEG;
            // (*decoder.codec_ctx).color_primaries = AVCOL_PRI_BT709;
            // (*decoder.codec_ctx).color_trc = AVCOL_TRC_IEC61966_2_1;
            // (*decoder.codec_ctx).colorspace = AVCOL_SPC_BT709;

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

                let ret = av_hwdevice_ctx_create(
                    &mut hwdevice_ctx,
                    (*hw_config).device_type,
                    ptr::null(),
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

            Ok(decoder)
        }
    }

    pub fn set_opt(&self, key: &str, value: &str, search_flags: i32) -> Result<(), MirrorXError> {
        let opt_name =
            CString::new(key.to_string()).map_err(|err| MirrorXError::Other(anyhow!(err)))?;
        let opt_value =
            CString::new(value.to_string()).map_err(|err| MirrorXError::Other(anyhow!(err)))?;

        unsafe {
            let ret = av_opt_set(
                (*self.codec_ctx).priv_data,
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

    pub fn open(&mut self) -> Result<Receiver<NativeFrame>, MirrorXError> {
        if self.output_tx.is_some() {
            return Err(MirrorXError::MediaVideoDecoderAlreadyOpened);
        }

        unsafe {
            let ret = avcodec_open2(self.codec_ctx, self.codec, ptr::null_mut());
            if ret != 0 {
                return Err(MirrorXError::MediaVideoDecoderOpenFailed(ret));
            }

            let (tx, rx) = bounded::<NativeFrame>(600);
            self.output_tx = Some(tx);
            Ok(rx)
        }
    }

    pub fn decode(
        &self,
        data: *const u8,
        data_size: i32,
        dts: i64,
        pts: i64,
    ) -> Result<(), MirrorXError> {
        unsafe {
            if !self.parser_ctx.is_null() {
                let ret = av_parser_parse2(
                    self.parser_ctx,
                    self.codec_ctx,
                    &mut (*self.packet).data,
                    &mut (*self.packet).size,
                    data,
                    data_size,
                    pts,
                    dts,
                    0,
                );

                if ret < 0 {
                    return Err(MirrorXError::MediaVideoDecoderParser2Failed(ret));
                }
            } else {
                (*self.packet).data = data as *mut u8;
                (*self.packet).size = data_size;
                (*self.packet).pts = pts;
                (*self.packet).dts = dts;
            }

            let mut ret = avcodec_send_packet(self.codec_ctx, self.packet);

            if ret == AVERROR(libc::EAGAIN) {
                return Err(MirrorXError::MediaVideoDecoderPacketUnacceptable);
            } else if ret == AVERROR_EOF {
                return Err(MirrorXError::MediaVideoDecoderClosed);
            } else if ret < 0 {
                return Err(MirrorXError::MediaVideoDecoderSendPacketFailed(ret));
            }

            defer! {
                av_packet_unref((*self).packet);
            }

            let mut tmp_frame: *mut AVFrame = ptr::null_mut();

            loop {
                ret = avcodec_receive_frame(self.codec_ctx, self.decode_frame);

                if ret == AVERROR(libc::EAGAIN) {
                    return Ok(());
                } else if ret == AVERROR_EOF {
                    return Ok(());
                } else if ret < 0 {
                    return Err(MirrorXError::MediaVideoDecoderReceiveFrameFailed(ret));
                }

                if !self.parser_ctx.is_null() {
                    tmp_frame = self.decode_frame;
                } else {
                    if let Err(err) = self.send_native_frame() {
                        return Err(err);
                    }
                }
            }
        }
    }

    #[cfg(target_os = "macos")]
    unsafe fn send_native_frame(&self) -> Result<(), MirrorXError> {
        let native_frame = crate::media::bindings::macos::CVPixelBufferRetain(
            (*self.decode_frame).data[3] as crate::media::bindings::macos::CVPixelBufferRef,
        );

        if let Some(tx) = &self.output_tx {
            tx.try_send(NativeFrame(native_frame))
                .map_err(|_| MirrorXError::MediaVideoDecoderOutputTxSendFailed)
        } else {
            Err(MirrorXError::ComponentUninitialized)
        }
    }

    #[cfg(target_os = "windows")]
    unsafe fn send_native_frame(&self) -> Result<(), MirrorXError> {
        use super::libyuv;
        use crate::media::{
            ffmpeg::avutil::hwcontext::av_hwframe_transfer_data, libyuv::kYvuF709Constants,
        };

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
        let ret = libyuv::NV21ToARGBMatrix(
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

        if let Some(tx) = self.output_tx.as_ref() {
            tx.try_send(NativeFrame(abgr_frame))
                .map_err(|_| MirrorXError::MediaVideoDecoderOutputTxSendFailed)?;

            info!("output tx send success");
            Ok(())
        } else {
            Err(MirrorXError::ComponentUninitialized)
        }
    }
}

impl Drop for VideoDecoder {
    fn drop(&mut self) {
        unsafe {
            if self.output_tx.is_some() {
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
