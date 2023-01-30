use super::resampler::{cpal_sample_format_to_av_sample_format, Resampler};
use crate::{
    api::endpoint::message::{AudioSampleFormat, EndPointAudioFrame},
    component::frame::AudioEncodeFrame,
    core_error,
    error::CoreResult,
};
use cpal::SampleFormat;
use mirrorx_native::{ffmpeg::utils::samplefmt::AV_SAMPLE_FMT_FLT, opus::encoder::*};

pub struct AudioEncoder {
    opus_encoder: *mut OpusEncoder,
    channels: u16,
    sample_rate: u32,
    sample_format: SampleFormat,
    encode_buffer: [u8; 64000],
    resampler: Option<Resampler>,
}

impl AudioEncoder {
    pub fn encode(&mut self, capture_frame: AudioEncodeFrame) -> CoreResult<EndPointAudioFrame> {
        unsafe {
            if self.opus_encoder.is_null()
                || self.channels != capture_frame.channels
                || self.sample_rate != capture_frame.sample_rate
                || self.sample_format != capture_frame.sample_format
            {
                if !self.opus_encoder.is_null() {
                    opus_encoder_destroy(self.opus_encoder);
                }

                let mut ret = 0;
                let opus_encoder = opus_encoder_create(
                    48000,
                    capture_frame.channels as _,
                    OPUS_APPLICATION_RESTRICTED_LOWDELAY,
                    &mut ret,
                );

                if ret < 0 {
                    return Err(core_error!("opus_encoder_create returns error ({})", ret));
                }

                self.opus_encoder = opus_encoder;
                self.channels = capture_frame.channels;
                self.sample_format = capture_frame.sample_format;
                self.sample_rate = capture_frame.sample_rate;

                self.resampler = if self.sample_rate != 48000 {
                    let resampler = Resampler::new(
                        (capture_frame.buffer.len()
                            / self.sample_format.sample_size()
                            / (self.channels as usize)) as _,
                        self.channels,
                        self.sample_rate as _,
                        cpal_sample_format_to_av_sample_format(self.sample_format),
                        self.channels,
                        48000,
                        AV_SAMPLE_FMT_FLT,
                    )?;

                    Some(resampler)
                } else {
                    None
                };
            }

            let mut data = if let Some(ref mut resampler) = self.resampler {
                resampler.convert(&capture_frame.buffer)?
            } else {
                capture_frame.buffer
            };

            data.resize(960 * self.sample_format.sample_size(), 0);

            let ret = if capture_frame.sample_format.is_float() {
                opus_encode_float(
                    self.opus_encoder,
                    std::mem::transmute(data.as_ptr()),
                    (960 / self.channels) as _,
                    self.encode_buffer.as_mut_ptr(),
                    self.encode_buffer.len() as _,
                )
            } else {
                opus_encode(
                    self.opus_encoder,
                    std::mem::transmute(data.as_ptr()),
                    (960 / self.channels) as _,
                    self.encode_buffer.as_mut_ptr(),
                    self.encode_buffer.len() as _,
                )
            };

            if ret > 0 {
                Ok(EndPointAudioFrame {
                    channels: self.channels as _,
                    sample_format: AudioSampleFormat::from(self.sample_format),
                    sample_rate: 48000,
                    buffer: self.encode_buffer[..ret as usize].to_vec(),
                })
            } else {
                Err(core_error!("opus encode failed ({})", ret))
            }
        }
    }
}

impl Default for AudioEncoder {
    fn default() -> Self {
        Self {
            opus_encoder: std::ptr::null_mut(),
            channels: 0,
            sample_rate: 0,
            sample_format: SampleFormat::I16,
            encode_buffer: [0u8; 64000],
            resampler: None,
        }
    }
}

impl Drop for AudioEncoder {
    fn drop(&mut self) {
        if !self.opus_encoder.is_null() {
            unsafe { opus_encoder_destroy(self.opus_encoder) }
        }
    }
}
