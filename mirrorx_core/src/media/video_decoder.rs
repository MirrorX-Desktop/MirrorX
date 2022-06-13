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
use anyhow::bail;
use crossbeam::channel::{bounded, Receiver, Sender};
use std::{
    ffi::{CStr, CString},
    ptr,
};

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
    pub fn new(decoder_name: &str) -> anyhow::Result<VideoDecoder> {
        let decoder_name_ptr = CString::new(decoder_name)?;

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
                tracing::info!(
                    device_name = CStr::from_ptr(support_hw_device_name).to_str()?,
                    "support hw device name"
                );
            }

            decoder.codec = avcodec_find_decoder_by_name(decoder_name_ptr.as_ptr());
            if decoder.codec.is_null() {
                bail!("find decoder failed");
            }

            decoder.codec_ctx = avcodec_alloc_context3(decoder.codec);
            if decoder.codec_ctx.is_null() {
                bail!("alloc codec context failed");
            }

            (*decoder.codec_ctx).width = 1920;
            (*decoder.codec_ctx).height = 1080;
            (*decoder.codec_ctx).pix_fmt = AV_PIX_FMT_NV12;
            // (*decoder.codec_ctx).flags |= AV_CODEC_FLAG2_LOCAL_HEADER;
            (*decoder.codec_ctx).color_range = AVCOL_RANGE_JPEG;
            // (*decoder.codec_ctx).color_primaries = AVCOL_PRI_BT709;
            // (*decoder.codec_ctx).color_trc = AVCOL_TRC_IEC61966_2_1;
            // (*decoder.codec_ctx).colorspace = AVCOL_SPC_BT709;

            decoder.packet = av_packet_alloc();
            if decoder.packet.is_null() {
                bail!("alloc packet failed");
            }

            decoder.decode_frame = av_frame_alloc();
            if decoder.decode_frame.is_null() {
                bail!("alloc decode frame failed");
            }

            let hw_config = avcodec_get_hw_config(decoder.codec, 0);
            if hw_config.is_null() {
                decoder.parser_ctx = av_parser_init((*decoder.codec).id);
                if decoder.parser_ctx.is_null() {
                    bail!("init parser failed");
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
                    bail!("create hw device context failed");
                }

                (*decoder.codec_ctx).hw_device_ctx = hwdevice_ctx;

                decoder.hw_decode_frame = av_frame_alloc();
                if decoder.hw_decode_frame.is_null() {
                    bail!("alloc hw decode frame failed");
                }
            }

            Ok(decoder)
        }
    }

    pub fn open(&mut self) -> anyhow::Result<Receiver<NativeFrame>> {
        if self.output_tx.is_some() {
            bail!("video decoder already opened");
        }

        unsafe {
            let ret = avcodec_open2(self.codec_ctx, self.codec, ptr::null_mut());
            if ret != 0 {
                bail!("open decoder failed ret={}", ret)
            }

            let (tx, rx) = bounded::<NativeFrame>(600);
            self.output_tx = Some(tx);
            Ok(rx)
        }
    }

    pub fn decode(&self, data: *const u8, data_size: i32, dts: i64, pts: i64) {
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
                    tracing::error!(ret = ret, "av_parser_parse2 failed");
                    return;
                }
            } else {
                (*self.packet).data = data as *mut u8;
                (*self.packet).size = data_size;
                (*self.packet).pts = pts;
                (*self.packet).dts = dts;
            }

            let mut ret = avcodec_send_packet(self.codec_ctx, self.packet);

            if ret == AVERROR(libc::EAGAIN) {
                tracing::error!("can not send more packet to decoder");
                return;
            } else if ret == AVERROR_EOF {
                tracing::error!("decoder closed");
                return;
            } else if ret < 0 {
                tracing::error!(ret = ret, "avcodec_send_packet failed");
                return;
            }

            let mut tmp_frame: *mut AVFrame = ptr::null_mut();

            loop {
                ret = avcodec_receive_frame(self.codec_ctx, self.decode_frame);

                if ret == AVERROR(libc::EAGAIN) {
                    break;
                } else if ret == AVERROR_EOF {
                    tracing::error!("decoder closed");
                    break;
                } else if ret < 0 {
                    tracing::error!(ret = ret, "avcodec_receive_frame failed");
                    break;
                }

                if !self.parser_ctx.is_null() {
                    tmp_frame = self.decode_frame;
                } else {
                    // ret = av_hwframe_transfer_data(self.hw_decode_frame, self.decode_frame, 0);

                    // if ret < 0 {
                    //     tracing::error!(ret = ret, "av_hwframe_transfer_data failed");
                    //     break;
                    // }

                    #[cfg(target_os = "macos")]
                    let native_frame = crate::media::bindings::macos::CVPixelBufferRetain(
                        (*self.decode_frame).data[3]
                            as crate::media::bindings::macos::CVPixelBufferRef,
                    );

                    #[cfg(target_os = "windows")]
                    let native_frame = (*self.decode_frame).data[3] as *mut libc::c_void;

                    if let Some(tx) = &self.output_tx {
                        if let Err(e) = tx.send(NativeFrame(native_frame)) {
                            tracing::error!(e = ?e, "send video frame failed");
                        }
                    }
                }
            }

            av_packet_unref((*self).packet);
        }
    }
}

impl Drop for VideoDecoder {
    fn drop(&mut self) {
        unsafe {
            if self.output_tx.is_some() {
                // inner codec had opened
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
                // av_buffer_unref(self.codec_ctx.hw_device_ctx)
                avcodec_free_context(&mut self.codec_ctx);
            }
        }
    }
}
