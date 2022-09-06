use super::frame::DecodedFrame;
use crate::{
    core_error,
    error::{CoreError, CoreResult},
    ffi::{
        ffmpeg::{avcodec::*, avutil::*},
        libyuv::*,
    },
    service::endpoint::message::VideoFrame,
};
use crossbeam::channel::Sender;
use std::ffi::{CStr, CString};

pub struct VideoDecoder {
    codec_ctx: *mut AVCodecContext,
    parser_ctx: *mut AVCodecParserContext,
    packet: *mut AVPacket,
    decode_frame: *mut AVFrame,
    hw_decode_frame: *mut AVFrame,
}

unsafe impl Send for VideoDecoder {}

impl VideoDecoder {
    pub fn new(width: i32, height: i32) -> CoreResult<VideoDecoder> {
        unsafe {
            av_log_set_level(AV_LOG_TRACE);
            av_log_set_flags(AV_LOG_SKIP_REPEATED);

            let mut decoder = VideoDecoder {
                codec_ctx: std::ptr::null_mut(),
                parser_ctx: std::ptr::null_mut(),
                packet: std::ptr::null_mut(),
                decode_frame: std::ptr::null_mut(),
                hw_decode_frame: std::ptr::null_mut(),
            };

            let codec = avcodec_find_decoder(AV_CODEC_ID_H264);
            if codec.is_null() {
                return Err(core_error!("avcodec_find_decoder returns null"));
            }

            decoder.codec_ctx = avcodec_alloc_context3(codec);
            if decoder.codec_ctx.is_null() {
                return Err(core_error!("avcodec_alloc_context3 returns null"));
            }

            (*decoder.codec_ctx).width = width;
            (*decoder.codec_ctx).height = height;
            (*decoder.codec_ctx).framerate = AVRational { num: 1, den: 1 };
            // (*decoder.codec_ctx).pkt_timebase = AVRational { num: 1, den: fps };
            (*decoder.codec_ctx).pix_fmt = AV_PIX_FMT_NV12;
            (*decoder.codec_ctx).color_range = AVCOL_RANGE_JPEG;
            (*decoder.codec_ctx).color_primaries = AVCOL_PRI_BT709;
            (*decoder.codec_ctx).color_trc = AVCOL_TRC_BT709;
            (*decoder.codec_ctx).colorspace = AVCOL_SPC_BT709;
            (*decoder.codec_ctx).flags |= AV_CODEC_FLAG_LOW_DELAY;

            let mut hw_device_type =
                av_hwdevice_find_type_by_name(CString::new("d3d11va")?.as_ptr());

            if hw_device_type == AV_HWDEVICE_TYPE_NONE {
                tracing::error!("current environment does't support 'd3d11va'");

                let mut devices = Vec::new();
                loop {
                    hw_device_type = av_hwdevice_iterate_types(hw_device_type);
                    if hw_device_type == AV_HWDEVICE_TYPE_NONE {
                        break;
                    }

                    let device_name = av_hwdevice_get_type_name(hw_device_type);

                    devices.push(
                        CStr::from_ptr(device_name)
                            .to_str()
                            .map_or("unknown", |v| v),
                    );
                }

                tracing::info!(?devices, "support hw device");
                tracing::info!("init software decoder");

                decoder.parser_ctx = av_parser_init(AV_CODEC_ID_H264);
                if decoder.parser_ctx.is_null() {
                    return Err(core_error!("av_parser_init returns null"));
                }
            } else {
                let mut hwdevice_ctx = std::ptr::null_mut();

                let ret = av_hwdevice_ctx_create(
                    &mut hwdevice_ctx,
                    hw_device_type,
                    std::ptr::null(),
                    std::ptr::null_mut(),
                    0,
                );

                if ret < 0 {
                    return Err(core_error!(
                        "av_hwdevice_ctx_create returns error code: {}",
                        ret,
                    ));
                }

                (*decoder.codec_ctx).hw_device_ctx = av_buffer_ref(hwdevice_ctx);
            }

            decoder.packet = av_packet_alloc();
            if decoder.packet.is_null() {
                return Err(core_error!("av_packet_alloc returns null"));
            }

            decoder.decode_frame = av_frame_alloc();
            if decoder.decode_frame.is_null() {
                return Err(core_error!("av_frame_alloc returns null"));
            }

            decoder.hw_decode_frame = av_frame_alloc();
            if decoder.hw_decode_frame.is_null() {
                return Err(core_error!("av_frame_alloc returns null"));
            }

            let ret = avcodec_open2(decoder.codec_ctx, codec, std::ptr::null_mut());
            if ret != 0 {
                return Err(core_error!("avcodec_open2 returns error code: {}", ret));
            }

            Ok(decoder)
        }
    }

    pub fn decode(&self, mut frame: VideoFrame, tx: &Sender<DecodedFrame>) -> CoreResult<()> {
        unsafe {
            if !self.parser_ctx.is_null() {
                let ret = av_parser_parse2(
                    self.parser_ctx,
                    self.codec_ctx,
                    &mut (*self.packet).data,
                    &mut (*self.packet).size,
                    frame.buffer.as_ptr(),
                    frame.buffer.len() as i32,
                    0,
                    0,
                    0,
                );

                if ret < 0 {
                    return Err(core_error!("av_parser_parse2 returns error code: {}", ret));
                }
            } else {
                (*self.packet).data = frame.buffer.as_mut_ptr();
                (*self.packet).size = frame.buffer.len() as i32;
                // (*self.packet).pts = frame.pts;
                // (*self.packet).dts = frame.pts;
            }

            // av_packet_rescale_ts(self.packet, AV_TIME_BASE_Q, (*self.codec_ctx).pkt_timebase);

            let mut ret = avcodec_send_packet(self.codec_ctx, self.packet);

            if ret == AVERROR(libc::EAGAIN) {
                return Err(core_error!("avcodec_send_packet returns EAGAIN"));
            } else if ret == AVERROR_EOF {
                return Err(core_error!("avcodec_send_packet returns AVERROR_EOF"));
            } else if ret < 0 {
                return Err(core_error!(
                    "avcodec_send_packet returns error code: {}",
                    ret
                ));
            }

            loop {
                ret = avcodec_receive_frame(self.codec_ctx, self.decode_frame);
                if ret == AVERROR(libc::EAGAIN) || ret == AVERROR_EOF {
                    return Ok(());
                } else if ret < 0 {
                    return Err(core_error!(
                        "avcodec_receive_frame returns error code: {}",
                        ret
                    ));
                }

                let mut tmp_frame: *mut AVFrame;

                if !self.parser_ctx.is_null() {
                    tmp_frame = self.decode_frame;
                } else {
                    let ret = av_hwframe_transfer_data(self.hw_decode_frame, self.decode_frame, 0);
                    if ret < 0 {
                        return Err(core_error!(
                            "av_hwframe_transfer_data returns error code: {}",
                            ret
                        ));
                    }

                    tmp_frame = self.hw_decode_frame;
                }

                av_frame_unref(tmp_frame);

                let decode_frame_buffer = convert_yuv_to_rgb(tmp_frame)?;
                let decoded_frame = DecodedFrame {
                    buffer: decode_frame_buffer,
                    width: (*tmp_frame).width,
                    height: (*tmp_frame).height,
                };

                if let Err(err) = tx.try_send(decoded_frame) {
                    if err.is_full() {
                        tracing::warn!("decoded frame tx is full");
                    } else if err.is_disconnected() {
                        return Err(core_error!("decoded frame tx is disconnected"));
                    }
                }
            }
        }
    }
}

impl Drop for VideoDecoder {
    fn drop(&mut self) {
        unsafe {
            if !self.codec_ctx.is_null() {
                avcodec_send_packet(self.codec_ctx, std::ptr::null());
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

unsafe fn convert_yuv_to_rgb(frame: *mut AVFrame) -> CoreResult<Vec<u8>> {
    let argb_stride = 4 * ((32 * (*frame).width + 31) / 32);
    let argb_frame_size = (argb_stride as usize) * ((*frame).height as usize) * 4;
    let mut argb_frame_buffer = Vec::<u8>::with_capacity(argb_frame_size);

    let ret = NV21ToARGBMatrix(
        (*frame).data[0],
        (*frame).linesize[0] as isize,
        (*frame).data[1],
        (*frame).linesize[1] as isize,
        argb_frame_buffer.as_mut_ptr(),
        argb_stride as isize,
        &kYvuF709Constants,
        (*frame).width as isize,
        (*frame).height as isize,
    );

    if ret != 0 {
        return Err(core_error!("NV21ToARGBMatrix returns error code: {}", ret));
    }

    argb_frame_buffer.set_len(argb_frame_size);

    Ok(argb_frame_buffer)
}
