use crate::{api::endpoint::message::EndPointAudioFrame, core_error, error::CoreResult};
use cpal::SampleFormat;
use mirrorx_native::opus::decoder::{
    opus_decode, opus_decode_float, opus_decoder_create, opus_decoder_destroy, OpusDecoder,
};

#[derive(Debug)]
pub struct AudioDecoder {
    opus_decoder: *mut OpusDecoder,
    channels: u8,
    sample_rate: u32,
    sample_format: SampleFormat,
}

impl AudioDecoder {
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
            };

            buffer.set_len(
                (ret as usize) * self.sample_format.sample_size() * (self.channels as usize),
            );

            Ok(buffer)
        }
    }
}

impl Default for AudioDecoder {
    fn default() -> Self {
        Self {
            opus_decoder: std::ptr::null_mut(),
            channels: 0,
            sample_format: SampleFormat::I16,
            sample_rate: 0,
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
