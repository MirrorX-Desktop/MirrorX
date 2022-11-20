use crate::{
    api::endpoint::message::EndPointAudioFrame, component::frame::AudioEncodeFrame, core_error,
    error::CoreResult,
};
use cpal::SampleFormat;
use mirrorx_native::opus::encoder::*;

pub struct AudioEncoder {
    opus_encoder: *mut OpusEncoder,
    channels: u16,
    sample_rate: u32,
    sample_format: SampleFormat,
    encode_buffer: [u8; 64000],
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

                let fs = if capture_frame.sample_rate <= 8000 {
                    8000
                } else if capture_frame.sample_rate <= 12000 {
                    12000
                } else if capture_frame.sample_rate <= 16000 {
                    16000
                } else if capture_frame.sample_rate <= 24000 {
                    24000
                } else {
                    48000
                };

                let mut ret = 0;
                let opus_encoder = opus_encoder_create(
                    fs,
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
                self.sample_rate = fs as _;
            }

            let frame_size = capture_frame.buffer.len()
                / capture_frame.sample_format.sample_size()
                / (capture_frame.channels as usize);

            let ret = match capture_frame.sample_format {
                SampleFormat::I16 | SampleFormat::U16 => opus_encode(
                    self.opus_encoder,
                    std::mem::transmute(capture_frame.buffer.as_ptr()),
                    frame_size as _,
                    self.encode_buffer.as_mut_ptr(),
                    self.encode_buffer.len() as _,
                ),
                SampleFormat::F32 => opus_encode(
                    self.opus_encoder,
                    std::mem::transmute(capture_frame.buffer.as_ptr()),
                    frame_size as _,
                    self.encode_buffer.as_mut_ptr(),
                    self.encode_buffer.len() as _,
                ),
            };

            if ret > 0 {
                Ok(EndPointAudioFrame {
                    channels: self.channels as _,
                    sample_format: self.sample_format.into(),
                    sample_rate: self.sample_rate as _,
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
