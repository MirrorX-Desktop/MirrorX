use crate::{api::endpoint::message::EndPointAudioFrame, core_error, error::CoreResult};
use cpal::{SampleFormat, SampleRate};
use libc::c_void;
use mirrorx_native::ffmpeg::{
    avcodec::*,
    avutil::*,
    swresample::{swr_alloc, swr_convert, swr_free, swr_get_delay, swr_init, SwrContext},
};
use tokio::sync::mpsc::Sender;

const OPT_IN_CHANNEL_LAYOUT: &[u8] = b"in_chlayout\0";
const OPT_IN_SAMPLE_RATE: &[u8] = b"in_sample_rate\0";
const OPT_IN_SAMPLE_FORMAT: &[u8] = b"in_sample_fmt\0";
const OPT_OUT_CHANNEL_LAYOUT: &[u8] = b"out_chlayout\0";
const OPT_OUT_SAMPLE_RATE: &[u8] = b"out_sample_rate\0";
const OPT_OUT_SAMPLE_FORMAT: &[u8] = b"out_sample_fmt\0";

pub struct AudioDecoder {
    decode_context: Option<DecodeContext>,
    resample_context: Option<ResampleContext>,
    output_channels: u16,
    output_sample_format: SampleFormat,
    output_sample_rate: SampleRate,
    output_tx: Sender<Vec<u8>>,
}

impl AudioDecoder {
    pub fn new(
        output_channels: u16,
        output_sample_format: SampleFormat,
        output_sample_rate: SampleRate,
        output_tx: Sender<Vec<u8>>,
    ) -> AudioDecoder {
        unsafe {
            av_log_set_level(AV_LOG_TRACE);
            av_log_set_flags(AV_LOG_SKIP_REPEATED);
        }

        AudioDecoder {
            decode_context: None,
            resample_context: None,
            output_channels,
            output_sample_format,
            output_sample_rate,
            output_tx,
        }
    }

    pub fn decode(&mut self, mut audio_frame: EndPointAudioFrame) -> CoreResult<()> {
        unsafe {
            if let Some(decode_context) = self.decode_context.as_ref() {
                if (*decode_context.codec_ctx).channels != (audio_frame.channels as i32)
                    || (*decode_context.codec_ctx).sample_fmt != audio_frame.sample_format
                    || (*decode_context.codec_ctx).sample_rate != audio_frame.sample_rate
                {
                    self.decode_context = None;
                }
            }

            if self.decode_context.is_none() {
                self.decode_context = Some(DecodeContext::new()?);

                let (output_sample_format, total_samples) = match self.output_sample_format {
                    SampleFormat::I16 => (AV_SAMPLE_FMT_S16, audio_frame.buffer.len() / 2),
                    SampleFormat::U16 => (AV_SAMPLE_FMT_S16, audio_frame.buffer.len() / 2),
                    SampleFormat::F32 => (AV_SAMPLE_FMT_FLT, audio_frame.buffer.len() / 4),
                };

                // check if needs resample
                // self.resample_context = if (audio_frame.channels as u16) != self.output_channels
                //     || audio_frame.sample_format != output_sample_format
                //     || audio_frame.sample_rate != self.output_sample_rate.0 as i32
                // {
                //     Some(ResampleContext::new(
                //         (total_samples as i32) / (audio_frame.channels as i32),
                //         audio_frame.channels as u16,
                //         audio_frame.sample_rate,
                //         audio_frame.sample_format,
                //         self.output_channels,
                //         self.output_sample_rate.0 as i32,
                //         output_sample_format,
                //     )?)
                // } else {
                //     None
                // };

                // tracing::info!(?self.resample_context, "resample context");
            }

            let Some(decode_context)= self.decode_context.as_ref() else{
                return Err(core_error!("decode context is null"));
            };

            (*(decode_context).packet).data = audio_frame.buffer.as_mut_ptr();
            (*(decode_context).packet).size = audio_frame.buffer.len() as i32;
            // (*(decode_context).packet).pts = audio_frame.pts;
            // (*(decode_context).packet).dts = audio_frame.pts;

            let mut ret = avcodec_send_packet((decode_context).codec_ctx, (decode_context).packet);

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
                ret = avcodec_receive_frame((decode_context).codec_ctx, (decode_context).frame);

                if ret == AVERROR(libc::EAGAIN) || ret == AVERROR_EOF {
                    return Ok(());
                } else if ret < 0 {
                    return Err(core_error!(
                        "avcodec_receive_frame returns error code: {}",
                        ret
                    ));
                }

                let linesize = (*decode_context.frame).linesize[0];

                let mut data = std::slice::from_raw_parts(
                    (*decode_context.frame).data[0],
                    (*decode_context.frame).linesize[0] as usize,
                )
                .to_vec();

                tracing::info!(?linesize, "linesize");

                if let Some(resample_context) = self.resample_context.as_mut() {
                    // data = resample_context.convert(&data)?;
                    tracing::info!(?resample_context, "resample context");
                }

                if let Err(err) = self.output_tx.try_send(data) {
                    match err {
                        tokio::sync::mpsc::error::TrySendError::Full(_) => {
                            tracing::warn!("audio play tx is full")
                        }
                        tokio::sync::mpsc::error::TrySendError::Closed(_) => {
                            return Err(core_error!("audio play tx was closed"));
                        }
                    }
                }

                av_frame_unref(decode_context.frame);
            }
        }
    }
}

struct DecodeContext {
    codec_ctx: *mut AVCodecContext,
    packet: *mut AVPacket,
    frame: *mut AVFrame,
}

impl DecodeContext {
    fn new() -> CoreResult<DecodeContext> {
        unsafe {
            let mut decode_ctx = DecodeContext {
                codec_ctx: std::ptr::null_mut(),
                packet: std::ptr::null_mut(),
                frame: std::ptr::null_mut(),
            };

            let codec = avcodec_find_decoder(AV_CODEC_ID_OPUS);

            if codec.is_null() {
                return Err(core_error!("avcodec_find_decoder returns null"));
            }

            decode_ctx.codec_ctx = avcodec_alloc_context3(codec);
            if decode_ctx.codec_ctx.is_null() {
                return Err(core_error!("avcodec_alloc_context3 returns null"));
            }

            decode_ctx.packet = av_packet_alloc();
            if decode_ctx.packet.is_null() {
                return Err(core_error!("av_packet_alloc returns null"));
            }

            decode_ctx.frame = av_frame_alloc();
            if decode_ctx.frame.is_null() {
                return Err(core_error!("av_frame_alloc returns null"));
            }

            let ret = avcodec_open2(decode_ctx.codec_ctx, codec, std::ptr::null_mut());
            if ret != 0 {
                return Err(core_error!("avcodec_open2 returns error code: {}", ret));
            }

            Ok(decode_ctx)
        }
    }
}

impl Drop for DecodeContext {
    fn drop(&mut self) {
        unsafe {
            if !self.codec_ctx.is_null() {
                avcodec_send_packet(self.codec_ctx, std::ptr::null());
            }

            if !self.frame.is_null() {
                av_frame_free(&mut self.frame);
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

#[derive(Debug)]
struct ResampleContext {
    swr_context: *mut SwrContext,

    // src_data: *mut *mut u8,
    // src_linesize: i32,
    src_rate: i32,
    src_nb_samples: i32,
    // src_nb_channels: i32,
    // src_sample_fmt: i32,
    dst_data: *mut *mut u8,
    dst_linesize: i32,
    dst_rate: i32,
    dst_nb_samples: i32,
    dst_nb_channels: i32,
    dst_sample_fmt: i32,

    max_dst_nb_samples: i32,
}

impl ResampleContext {
    fn new(
        nb_samples: i32,
        input_channels: u16,
        input_sample_rate: i32,
        input_sample_format: i32,
        output_channels: u16,
        output_sample_rate: i32,
        output_sample_format: i32,
    ) -> CoreResult<Self> {
        unsafe {
            let swr_context = swr_alloc();
            if swr_context.is_null() {
                return Err(core_error!("init swr context failed"));
            }

            let src_channel_layout = AVChannelLayout {
                order: AV_CHANNEL_ORDER_NATIVE,
                nb_channels: if input_channels >= 2 { 2 } else { 1 },
                u: AVChannelLayout_u {
                    mask: if input_channels >= 2 {
                        (1 << 0) | (1 << 1)
                    } else {
                        1 << 2
                    },
                },
                opaque: std::ptr::null_mut(),
            };

            let dst_channel_layout = AVChannelLayout {
                order: AV_CHANNEL_ORDER_NATIVE,
                nb_channels: if output_channels >= 2 { 2 } else { 1 },
                u: AVChannelLayout_u {
                    mask: if output_channels >= 2 {
                        (1 << 0) | (1 << 1)
                    } else {
                        1 << 2
                    },
                },
                opaque: std::ptr::null_mut(),
            };

            let ret = av_opt_set_chlayout(
                swr_context as *mut c_void,
                OPT_IN_CHANNEL_LAYOUT.as_ptr() as *const i8,
                &src_channel_layout,
                0,
            );

            if ret < 0 {
                return Err(core_error!("set swr input channel layout failed ({})", ret));
            }

            let ret = av_opt_set_int(
                swr_context as *mut c_void,
                OPT_IN_SAMPLE_RATE.as_ptr() as *const i8,
                input_sample_rate.into(),
                0,
            );

            if ret < 0 {
                return Err(core_error!("set swr input sample rate failed ({})", ret));
            }

            let ret = av_opt_set_sample_fmt(
                swr_context as *mut c_void,
                OPT_IN_SAMPLE_FORMAT.as_ptr() as *const i8,
                input_sample_format,
                0,
            );

            if ret < 0 {
                return Err(core_error!("set swr input sample format failed ({})", ret));
            }

            let ret = av_opt_set_chlayout(
                swr_context as *mut c_void,
                OPT_OUT_CHANNEL_LAYOUT.as_ptr() as *const i8,
                &dst_channel_layout,
                0,
            );

            if ret < 0 {
                return Err(core_error!(
                    "set swr output channel layout failed ({})",
                    ret
                ));
            }

            let ret = av_opt_set_int(
                swr_context as *mut c_void,
                OPT_OUT_SAMPLE_RATE.as_ptr() as *const i8,
                output_sample_rate.into(),
                0,
            );

            if ret < 0 {
                return Err(core_error!("set swr output sample rate failed ({})", ret));
            }

            let ret = av_opt_set_sample_fmt(
                swr_context as *mut c_void,
                OPT_OUT_SAMPLE_FORMAT.as_ptr() as *const i8,
                output_sample_format,
                0,
            );

            if ret < 0 {
                return Err(core_error!("set swr output sample format failed ({})", ret));
            }

            let ret = swr_init(swr_context);

            if ret < 0 {
                return Err(core_error!("init swr context failed ({})", ret));
            }

            // let src_nb_samples = nb_samples;
            // let src_nb_channels = src_channel_layout.nb_channels;
            // let mut src_data = std::ptr::null_mut();
            // let mut src_linesize = 0;
            // let ret = av_samples_alloc_array_and_samples(
            //     &mut src_data,
            //     &mut src_linesize,
            //     src_nb_channels,
            //     src_nb_samples,
            //     input_sample_format,
            //     0,
            // );

            // if ret < 0 {
            //     return Err(core_error!(
            //         "av_samples_alloc_array_and_samples failed ({})",
            //         ret
            //     ));
            // }

            let dst_nb_samples = av_rescale_rnd(
                nb_samples.into(),
                output_sample_rate.into(),
                input_sample_rate.into(),
                AV_ROUND_UP,
            ) as i32;
            let max_dst_nb_samples = dst_nb_samples;

            let dst_nb_channels = dst_channel_layout.nb_channels;
            let mut dst_data = std::ptr::null_mut();
            let mut dst_linesize = 0;
            let ret = av_samples_alloc_array_and_samples(
                &mut dst_data,
                &mut dst_linesize,
                dst_channel_layout.nb_channels,
                dst_nb_samples as i32,
                output_sample_format,
                0,
            );

            if ret < 0 {
                return Err(core_error!(
                    "av_samples_alloc_array_and_samples failed ({})",
                    ret
                ));
            }

            Ok(Self {
                swr_context,
                // src_data,
                // src_linesize,
                src_rate: input_sample_rate,
                src_nb_samples: nb_samples,
                // src_nb_channels,
                // src_sample_fmt: input_sample_format,
                dst_data,
                dst_linesize,
                dst_rate: output_sample_rate,
                dst_nb_samples,
                dst_nb_channels,
                dst_sample_fmt: output_sample_format,
                max_dst_nb_samples,
            })
        }
    }

    unsafe fn convert(&mut self, input_data: &[u8]) -> CoreResult<Vec<u8>> {
        self.dst_nb_samples = av_rescale_rnd(
            swr_get_delay(self.swr_context, self.src_rate as _) + (self.src_nb_samples as i64),
            self.dst_rate.into(),
            self.src_rate.into(),
            AV_ROUND_UP,
        ) as i32;

        if self.dst_nb_samples > self.max_dst_nb_samples {
            av_freep(self.dst_data as *mut c_void);

            let ret = av_samples_alloc(
                self.dst_data,
                &mut self.dst_linesize,
                self.dst_nb_channels,
                self.dst_nb_samples,
                self.dst_sample_fmt,
                1,
            );

            if ret != 0 {
                return Err(core_error!("av_samples_alloc failed ({})", ret));
            }

            self.max_dst_nb_samples = self.dst_nb_samples;
        }

        let ret = swr_convert(
            self.swr_context,
            self.dst_data,
            self.dst_nb_samples,
            &input_data.as_ptr(),
            self.src_nb_samples,
        );

        if ret < 0 {
            return Err(core_error!("swr_convert failed ({})", ret));
        }

        let dst_buffer_size = av_samples_get_buffer_size(
            &mut self.dst_linesize,
            self.dst_nb_channels,
            ret,
            self.dst_sample_fmt,
            1,
        );

        if dst_buffer_size < 0 {
            return Err(core_error!(
                "av_samples_get_buffer_size failed ({})",
                dst_buffer_size
            ));
        }

        Ok(std::slice::from_raw_parts(*self.dst_data, dst_buffer_size as usize).to_vec())
    }
}

impl Drop for ResampleContext {
    fn drop(&mut self) {
        if !self.swr_context.is_null() {
            unsafe { swr_free(&mut self.swr_context) }
        }
    }
}
