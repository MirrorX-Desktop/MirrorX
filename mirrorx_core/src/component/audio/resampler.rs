use crate::{core_error, error::CoreResult};
use cpal::SampleFormat;
use mirrorx_native::ffmpeg::{
    swresample::*,
    utils::{channel_layout::*, mathematics::*, mem::av_freep, opt::*, samplefmt::*},
};
use std::os::raw::c_void;

const OPT_IN_CHANNEL_LAYOUT: &[u8] = b"in_chlayout\0";
const OPT_IN_SAMPLE_RATE: &[u8] = b"in_sample_rate\0";
const OPT_IN_SAMPLE_FORMAT: &[u8] = b"in_sample_fmt\0";
const OPT_OUT_CHANNEL_LAYOUT: &[u8] = b"out_chlayout\0";
const OPT_OUT_SAMPLE_RATE: &[u8] = b"out_sample_rate\0";
const OPT_OUT_SAMPLE_FORMAT: &[u8] = b"out_sample_fmt\0";

#[derive(Debug)]
pub struct Resampler {
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

impl Resampler {
    pub fn new(
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
                dst_nb_samples,
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

    pub fn convert(&mut self, input_data: &[u8]) -> CoreResult<Vec<u8>> {
        unsafe {
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

                if ret < 0 {
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
}

impl Drop for Resampler {
    fn drop(&mut self) {
        if !self.swr_context.is_null() {
            unsafe {
                av_freep(self.dst_data as *mut c_void);
                swr_free(&mut self.swr_context);
            }
        }
    }
}

pub fn cpal_sample_format_to_av_sample_format(sample_format: SampleFormat) -> AVSampleFormat {
    match sample_format {
        SampleFormat::I8 | SampleFormat::U8 => AV_SAMPLE_FMT_U8,
        SampleFormat::I16 | SampleFormat::U16 => AV_SAMPLE_FMT_S16,
        SampleFormat::I32 | SampleFormat::U32 => AV_SAMPLE_FMT_S32,
        SampleFormat::I64 | SampleFormat::U64 => AV_SAMPLE_FMT_S64,
        SampleFormat::F32 => AV_SAMPLE_FMT_FLT,
        SampleFormat::F64 => AV_SAMPLE_FMT_DBL,
        _ => panic!("unsupported sample format"),
    }
}
