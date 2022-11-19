use crate::{
    api::endpoint::EndPointClient, component::frame::AudioEncodeFrame, core_error,
    error::CoreResult,
};
use cpal::SampleFormat;
use mirrorx_native::ffmpeg::{avcodec::*, avutil::*};

pub struct AudioEncoder {
    encode_context: *mut EncodeContext,
    client: EndPointClient,
}

impl AudioEncoder {
    pub fn new(client: EndPointClient) -> CoreResult<AudioEncoder> {
        unsafe {
            av_log_set_level(AV_LOG_INFO);
            av_log_set_flags(AV_LOG_SKIP_REPEATED);
        }

        Ok(AudioEncoder {
            encode_context: std::ptr::null_mut(),
            client,
        })
    }

    pub fn encode(&mut self, capture_frame: AudioEncodeFrame) -> CoreResult<()> {
        unsafe {
            let mut ret: i32;

            if self.encode_context.is_null()
                || (*(*self.encode_context).codec_ctx).channels != (capture_frame.channels as i32)
                || (*(*self.encode_context).codec_ctx).sample_rate
                    != (capture_frame.sample_rate as i32)
                || (*(*self.encode_context).codec_ctx).sample_fmt
                    != match capture_frame.sample_format {
                        SampleFormat::I16 => AV_SAMPLE_FMT_S16,
                        SampleFormat::U16 => AV_SAMPLE_FMT_S16,
                        SampleFormat::F32 => AV_SAMPLE_FMT_FLT,
                    }
            {
                if !self.encode_context.is_null() {
                    let _ = Box::from_raw(self.encode_context);
                }

                let buffer_size = match capture_frame.sample_format {
                    SampleFormat::I16 => {
                        capture_frame.buffer.len() / 2 / (capture_frame.channels as usize)
                    }
                    SampleFormat::U16 => {
                        capture_frame.buffer.len() / 2 / (capture_frame.channels as usize)
                    }
                    SampleFormat::F32 => {
                        capture_frame.buffer.len() / 4 / (capture_frame.channels as usize)
                    }
                };

                self.encode_context = Box::into_raw(Box::new(EncodeContext::new(
                    buffer_size as i32,
                    capture_frame.channels,
                    capture_frame.sample_format,
                    capture_frame.sample_rate,
                )?));
            }

            ret = av_frame_make_writable((*self.encode_context).frame);
            if ret < 0 {
                return Err(core_error!(
                    "av_frame_make_writable returns error code: {}",
                    ret
                ));
            }

            (*(*self.encode_context).frame).data[0] = capture_frame.buffer.as_ptr() as *mut _;
            (*(*self.encode_context).frame).linesize[0] = capture_frame.buffer.len() as i32;

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

                self.client.send_audio_frame(
                    (*(*self.encode_context).codec_ctx).channels as u8,
                    (*(*self.encode_context).codec_ctx).sample_fmt,
                    (*(*self.encode_context).codec_ctx).sample_rate,
                    std::slice::from_raw_parts(
                        (*(*self.encode_context).packet).data,
                        (*(*self.encode_context).packet).size as usize,
                    ),
                )?;

                av_packet_unref((*self.encode_context).packet);
            }
        }
    }
}

impl Drop for AudioEncoder {
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
    channels: u16,
    sample_format: SampleFormat,
    sample_rate: u32,
}

impl EncodeContext {
    pub fn new(
        nb_samples: i32,
        channels: u16,
        sample_format: SampleFormat,
        sample_rate: u32,
    ) -> CoreResult<EncodeContext> {
        unsafe {
            let codec = avcodec_find_encoder(AV_CODEC_ID_OPUS);
            if codec.is_null() {
                return Err(core_error!("avcodec_find_encoder returns null pointer"));
            }

            let encoder_context = EncodeContext {
                codec_ctx: avcodec_alloc_context3(codec),
                frame: av_frame_alloc(),
                packet: av_packet_alloc(),
                channels,
                sample_format,
                sample_rate,
            };

            if encoder_context.codec_ctx.is_null()
                || encoder_context.frame.is_null()
                || encoder_context.packet.is_null()
            {
                return Err(core_error!("avcodec_alloc_context3 returns null pointer"));
            }

            (*encoder_context.codec_ctx).bit_rate = 64000;
            (*encoder_context.codec_ctx).sample_rate = sample_rate as i32;
            (*encoder_context.codec_ctx).channels = channels as i32;
            (*encoder_context.codec_ctx).sample_fmt = match sample_format {
                SampleFormat::I16 => AV_SAMPLE_FMT_S16,
                SampleFormat::U16 => AV_SAMPLE_FMT_S16,
                SampleFormat::F32 => AV_SAMPLE_FMT_FLT,
            };

            (*encoder_context.frame).format = (*encoder_context.codec_ctx).sample_fmt;
            (*encoder_context.frame).nb_samples = nb_samples;
            (*encoder_context.frame).ch_layout = AVChannelLayout {
                order: AV_CHANNEL_ORDER_NATIVE,
                nb_channels: if channels >= 2 { 2 } else { 1 },
                u: AVChannelLayout_u {
                    mask: if channels >= 2 {
                        (1 << 0) | (1 << 1)
                    } else {
                        1 << 2
                    },
                },
                opaque: std::ptr::null_mut(),
            };

            if av_channel_layout_check(&(*encoder_context.frame).ch_layout) == 0 {
                return Err(core_error!("av_channel_layout_check check failed",));
            }

            let ret = av_frame_get_buffer(encoder_context.frame, 0);
            if ret < 0 {
                return Err(core_error!(
                    "av_frame_get_buffer returns error code: {}",
                    ret
                ));
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
