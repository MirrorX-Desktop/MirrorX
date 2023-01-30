use crate::{
    api::endpoint::message::EndPointAudioFrame,
    component::audio::resampler::{cpal_sample_format_to_av_sample_format, Resampler},
    core_error,
    error::CoreResult,
};
use cpal::{SampleFormat, SampleRate};
use mirrorx_native::opus::decoder::*;

pub struct AudioDecoder {
    opus_decoder: *mut OpusDecoder,
    resampler: Option<Resampler>,
    channels: u8,
    sample_rate: u32,
    sample_format: SampleFormat,
    out_channels: u8,
    out_sample_format: SampleFormat,
    out_sample_rate: SampleRate,
}

impl AudioDecoder {
    pub fn new(
        out_channels: u8,
        out_sample_format: SampleFormat,
        out_sample_rate: SampleRate,
    ) -> AudioDecoder {
        Self {
            opus_decoder: std::ptr::null_mut(),
            resampler: None,
            channels: 0,
            sample_rate: 0,
            sample_format: SampleFormat::F32,
            out_channels,
            out_sample_format,
            out_sample_rate,
        }
    }
    pub fn decode(&mut self, audio_frame: EndPointAudioFrame) -> CoreResult<Vec<u8>> {
        unsafe {
            let audio_frame_sample_format = audio_frame.sample_format.into();

            if self.opus_decoder.is_null()
                || self.channels != audio_frame.channels
                || self.sample_rate != audio_frame.sample_rate
                || self.sample_format != audio_frame_sample_format
            {
                if !self.opus_decoder.is_null() {
                    opus_decoder_destroy(self.opus_decoder);
                }

                let mut ret = 0;
                let opus_decoder = opus_decoder_create(
                    audio_frame.sample_rate as i32,
                    audio_frame.channels as isize,
                    &mut ret,
                );

                if ret < 0 {
                    return Err(core_error!("opus_decoder_create returns error ({})", ret));
                }

                self.opus_decoder = opus_decoder;
                self.channels = audio_frame.channels;
                self.sample_format = audio_frame_sample_format;
                self.sample_rate = audio_frame.sample_rate;

                if self.channels != self.out_channels
                    || self.sample_rate != self.out_sample_rate.0
                    || self.sample_format != self.out_sample_format
                {
                    let input_av_sample_format =
                        cpal_sample_format_to_av_sample_format(self.sample_format);
                    let output_av_sample_format =
                        cpal_sample_format_to_av_sample_format(self.out_sample_format);

                    self.resampler = Some(Resampler::new(
                        960 / self.channels as i32,
                        self.channels as _,
                        self.sample_rate as _,
                        input_av_sample_format as _,
                        self.out_channels as _,
                        self.out_sample_rate.0 as _,
                        output_av_sample_format as _,
                    )?);

                    tracing::info!(
                        input_channels = self.channels,
                        input_sample_rate = self.sample_rate,
                        input_sample_format = ?self.sample_format,
                        output_channels = self.out_channels,
                        output_sample_rate = self.out_sample_rate.0,
                        output_sample_format = ?self.out_sample_format,
                        "use audio re-sampler"
                    );
                }
            }

            let mut buffer = Vec::<u8>::with_capacity(960 * self.sample_format.sample_size());

            let frame_size = buffer.capacity()
                / self.sample_format.sample_size()
                / (audio_frame.channels as usize);

            let ret = match self.sample_format {
                SampleFormat::I16 | SampleFormat::U16 => opus_decode(
                    self.opus_decoder,
                    audio_frame.buffer.as_ptr(),
                    audio_frame.buffer.len() as _,
                    std::mem::transmute(buffer.as_mut_ptr()),
                    frame_size as _,
                    0,
                ),
                SampleFormat::F32 => opus_decode_float(
                    self.opus_decoder,
                    audio_frame.buffer.as_ptr(),
                    audio_frame.buffer.len() as _,
                    std::mem::transmute(buffer.as_mut_ptr()),
                    frame_size as _,
                    0,
                ),
                _ => return Err(core_error!("unsupported sample format")),
            };

            buffer.set_len(
                (ret as usize) * self.sample_format.sample_size() * (self.channels as usize),
            );

            if let Some(ref mut resampler) = self.resampler {
                buffer = resampler.convert(buffer.as_slice())?;
            }

            Ok(buffer)
        }
    }
}

impl Drop for AudioDecoder {
    fn drop(&mut self) {
        if !self.opus_decoder.is_null() {
            unsafe { opus_decoder_destroy(self.opus_decoder) }
        }
    }
}
