use super::config::EncoderConfig;
use crate::{
    api::endpoint::{
        client::EndPointClient,
        message::{EndPointMessage, EndPointVideoFrame},
    },
    component::frame::DesktopEncodeFrame,
    core_error,
    error::CoreResult,
};
use mirrorx_native::ffmpeg::{
    codecs::{avcodec::*, codec::*, packet::*},
    utils::{error::*, frame::*, imgutils::*, log::*, pixfmt::*, rational::AVRational},
};
use std::sync::Arc;

pub struct VideoEncoder<T>
where
    T: EncoderConfig,
{
    encoder_config: T,
    encode_context: Option<EncodeContext>,
    client: Arc<EndPointClient>,
}

impl<T> VideoEncoder<T>
where
    T: EncoderConfig,
{
    pub fn new(encoder_config: T, client: Arc<EndPointClient>) -> CoreResult<VideoEncoder<T>> {
        unsafe {
            av_log_set_level(AV_LOG_INFO);
            av_log_set_flags(AV_LOG_SKIP_REPEATED);
        }

        Ok(VideoEncoder {
            encoder_config,
            encode_context: None,
            client,
        })
    }

    pub fn encode(&mut self, capture_frame: DesktopEncodeFrame) -> CoreResult<()> {
        unsafe {
            let mut ret: i32;

            if let Some(ref encode_context) = self.encode_context {
                if (*encode_context.codec_ctx).width != capture_frame.width
                    || (*encode_context.codec_ctx).height != capture_frame.height
                {
                    self.encode_context = None;
                }
            }

            if self.encode_context.is_none() {
                self.encode_context = Some(EncodeContext::new(
                    capture_frame.width,
                    capture_frame.height,
                    &self.encoder_config,
                )?);
            }

            let Some(ref encode_context)= self.encode_context else{
                return Err(core_error!("encode context is empty"))
            };

            ret = av_frame_make_writable(encode_context.frame);
            if ret < 0 {
                return Err(core_error!(
                    "av_frame_make_writable returns error code: {}",
                    ret
                ));
            }

            (*(encode_context).frame).data[0] = capture_frame.luminance_bytes.as_ptr() as *mut _;
            (*(encode_context).frame).linesize[0] = capture_frame.luminance_stride;
            (*(encode_context).frame).data[1] = capture_frame.chrominance_bytes.as_ptr() as *mut _;
            (*(encode_context).frame).linesize[1] = capture_frame.chrominance_stride;
            (*(encode_context).frame).pts = (capture_frame.capture_time.as_secs_f64()
                * ((*(encode_context).codec_ctx).time_base.den as f64))
                as i64;

            ret = avcodec_send_frame((encode_context).codec_ctx, (encode_context).frame);

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
                ret = avcodec_receive_packet((encode_context).codec_ctx, (encode_context).packet);

                if ret == AVERROR(libc::EAGAIN) || ret == AVERROR_EOF {
                    return Ok(());
                } else if ret < 0 {
                    return Err(core_error!(
                        "avcodec_receive_packet returns error code: {}",
                        ret
                    ));
                }

                let frame = EndPointVideoFrame {
                    width: (*(encode_context).codec_ctx).width,
                    height: (*(encode_context).codec_ctx).height,
                    pts: (*(encode_context).packet).pts,
                    buffer: std::slice::from_raw_parts(
                        (*(encode_context).packet).data,
                        (*(encode_context).packet).size as usize,
                    )
                    .to_vec(),
                };

                self.client
                    .blocking_send(&EndPointMessage::VideoFrame(frame))?;

                av_packet_unref((encode_context).packet);
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

            let mut ret = av_frame_get_buffer(encoder_context.frame, 0);
            if ret < 0 {
                return Err(core_error!(
                    "av_frame_get_buffer returns error code: {}",
                    ret
                ));
            }

            let packet_size =
                av_image_get_buffer_size((*encoder_context.codec_ctx).pix_fmt, width, height, 1);

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
