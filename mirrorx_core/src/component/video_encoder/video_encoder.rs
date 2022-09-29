use super::config::{EncoderConfig, EncoderType};
use crate::{
    api::endpoint::message::{EndPointMessage, EndPointVideoFrame},
    component::frame::DesktopEncodeFrame,
    core_error,
    error::{CoreError, CoreResult},
    ffi::ffmpeg::{avcodec::*, avutil::*},
};
use tokio::sync::mpsc::{error::TrySendError, Sender};

pub struct VideoEncoder {
    encode_config: Box<dyn EncoderConfig>,
    encode_context: *mut EncodeContext,
    tx: Sender<Option<EndPointMessage>>,
}

impl VideoEncoder {
    pub fn new(
        encoder_type: EncoderType,
        tx: Sender<Option<EndPointMessage>>,
    ) -> CoreResult<VideoEncoder> {
        let encode_config = encoder_type.create_config();

        unsafe {
            // av_log_set_level(AV_LOG_TRACE);
            // av_log_set_flags(AV_LOG_SKIP_REPEATED);

            Ok(VideoEncoder {
                encode_config,
                encode_context: std::ptr::null_mut(),
                tx,
            })
        }
    }

    pub fn encode(&mut self, capture_frame: DesktopEncodeFrame) -> CoreResult<()> {
        if self.tx.is_closed() {
            return Err(core_error!("message tx has closed"));
        }

        unsafe {
            let mut ret: i32;

            if self.encode_context.is_null()
                || (*(*self.encode_context).codec_ctx).width != capture_frame.width
                || (*(*self.encode_context).codec_ctx).height != capture_frame.height
            {
                if !self.encode_context.is_null() {
                    let _ = Box::from_raw(self.encode_context);
                }

                self.encode_context = Box::into_raw(Box::new(EncodeContext::new(
                    capture_frame.width,
                    capture_frame.height,
                    self.encode_config.as_ref(),
                )?));
            }

            ret = av_frame_make_writable((*self.encode_context).frame);
            if ret < 0 {
                return Err(core_error!(
                    "av_frame_make_writable returns error code: {}",
                    ret
                ));
            }

            (*(*self.encode_context).frame).data[0] =
                capture_frame.luminance_bytes.as_ptr() as *mut _;

            (*(*self.encode_context).frame).linesize[0] = capture_frame.luminance_stride;

            (*(*self.encode_context).frame).data[1] =
                capture_frame.chrominance_bytes.as_ptr() as *mut _;

            (*(*self.encode_context).frame).linesize[1] = capture_frame.chrominance_stride;

            (*(*self.encode_context).frame).pts = (capture_frame.capture_time.as_secs_f64()
                * ((*(*self.encode_context).codec_ctx).time_base.den as f64))
                as i64;

            ret = avcodec_send_frame(
                (*self.encode_context).codec_ctx,
                (*self.encode_context).frame,
            );

            if ret != 0 {
                if ret == AVERROR(libc::EAGAIN) {
                    return Err(core_error!("avcodec_send_frame returns EAGAIN"));
                } else if ret == AVERROR_EOF {
                    return Err(core_error!("avcodec_send_frame returns AVERROR_EOF"));
                }
                return Err(core_error!(
                    "avcodec_send_frame returns error code: {}",
                    ret
                ));
            }

            loop {
                ret = avcodec_receive_packet(
                    (*self.encode_context).codec_ctx,
                    (*self.encode_context).packet,
                );

                if ret == AVERROR(libc::EAGAIN) || ret == AVERROR_EOF {
                    return Ok(());
                } else if ret < 0 {
                    return Err(core_error!(
                        "avcodec_receive_packet returns error code: {}",
                        ret
                    ));
                }

                let buffer = std::slice::from_raw_parts(
                    (*(*self.encode_context).packet).data,
                    (*(*self.encode_context).packet).size as usize,
                )
                .to_vec();

                let packet = EndPointMessage::VideoFrame(EndPointVideoFrame {
                    width: (*(*self.encode_context).codec_ctx).width,
                    height: (*(*self.encode_context).codec_ctx).height,
                    pts: (*(*self.encode_context).packet).pts,
                    buffer,
                });

                av_packet_unref((*self.encode_context).packet);

                if let Err(err) = self.tx.try_send(Some(packet)) {
                    if let TrySendError::Full(_) = err {
                        tracing::warn!(
                            "video encoder send EndPointMessage failed, channel is full!"
                        );
                    } else {
                        return Err(core_error!(
                            "video encoder send EndPointMessage failed, channel is closed"
                        ));
                    }
                }
            }
        }
    }
}

impl Drop for VideoEncoder {
    fn drop(&mut self) {
        if !self.encode_context.is_null() {
            unsafe {
                let _ = Box::from_raw(self.encode_context);
            }
        }
    }
}

struct EncodeContext {
    codec_ctx: *mut AVCodecContext,
    frame: *mut AVFrame,
    packet: *mut AVPacket,
}

impl EncodeContext {
    pub fn new(
        width: i32,
        height: i32,
        encoder_config: &dyn EncoderConfig,
    ) -> CoreResult<EncodeContext> {
        unsafe {
            let codec = avcodec_find_encoder(encoder_config.av_codec_id());
            if codec.is_null() {
                return Err(core_error!("avcodec_find_encoder returns null pointer"));
            }

            let encoder_context = EncodeContext {
                codec_ctx: avcodec_alloc_context3(codec),
                frame: av_frame_alloc(),
                packet: av_packet_alloc(),
            };

            if encoder_context.codec_ctx.is_null()
                || encoder_context.frame.is_null()
                || encoder_context.packet.is_null()
            {
                return Err(core_error!("avcodec_alloc_context3 returns null pointer"));
            }

            (*encoder_context.codec_ctx).width = width;
            (*encoder_context.codec_ctx).height = height;
            (*encoder_context.codec_ctx).framerate = AVRational { num: 60, den: 1 };
            (*encoder_context.codec_ctx).time_base = AVRational { num: 1, den: 60 };
            (*encoder_context.codec_ctx).gop_size = 4000;
            (*encoder_context.codec_ctx).bit_rate = 4000 * 1000;
            (*encoder_context.codec_ctx).rc_max_rate = 4000 * 1000;
            (*encoder_context.codec_ctx).rc_min_rate = 4000 * 1000;
            (*encoder_context.codec_ctx).rc_buffer_size = 4000 * 1000 * 2;
            (*encoder_context.codec_ctx).has_b_frames = 0;
            (*encoder_context.codec_ctx).max_b_frames = 0;
            (*encoder_context.codec_ctx).pix_fmt = AV_PIX_FMT_NV12;
            (*encoder_context.codec_ctx).flags2 |= AV_CODEC_FLAG2_LOCAL_HEADER;
            (*encoder_context.codec_ctx).color_range = AVCOL_RANGE_JPEG;
            (*encoder_context.codec_ctx).color_primaries = AVCOL_PRI_BT709;
            (*encoder_context.codec_ctx).color_trc = AVCOL_TRC_BT709;
            (*encoder_context.codec_ctx).colorspace = AVCOL_SPC_BT709;

            (*encoder_context.frame).format = (*encoder_context.codec_ctx).pix_fmt;
            (*encoder_context.frame).width = width;
            (*encoder_context.frame).height = height;

            encoder_config.apply_option(encoder_context.codec_ctx)?;

            let mut ret = av_frame_get_buffer(encoder_context.frame, 1);
            if ret < 0 {
                return Err(core_error!(
                    "av_frame_get_buffer returns error code: {}",
                    ret
                ));
            }

            let packet_size =
                av_image_get_buffer_size((*encoder_context.codec_ctx).pix_fmt, width, height, 32);

            ret = av_new_packet(encoder_context.packet, packet_size);
            if ret < 0 {
                return Err(core_error!("av_new_packet returns error code: {}", ret));
            }

            let ret = avcodec_open2(encoder_context.codec_ctx, codec, std::ptr::null_mut());
            if ret != 0 {
                return Err(core_error!("avcodec_open2 returns null pointer"));
            }

            Ok(encoder_context)
        }
    }
}

impl Drop for EncodeContext {
    fn drop(&mut self) {
        unsafe {
            if !self.codec_ctx.is_null() {
                avcodec_send_frame(self.codec_ctx, std::ptr::null_mut());
                avcodec_free_context(&mut self.codec_ctx);
            }

            if !self.frame.is_null() {
                av_frame_free(&mut self.frame);
            }

            if !self.packet.is_null() {
                av_packet_free(&mut self.packet);
            }
        }
    }
}
