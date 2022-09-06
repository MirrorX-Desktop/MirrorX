use crate::{
    core_error,
    error::{CoreError, CoreResult},
    ffi::opus::encoder::{
        opus_encode_float, opus_encoder_create, opus_encoder_destroy, OpusEncoder,
        OPUS_APPLICATION_RESTRICTED_LOWDELAY,
    },
};

pub struct AudioEncoder {
    enc: *mut OpusEncoder,
    enc_buffer: [u8; 4000],
    sample_rate: i32,
    channels: isize,
}

unsafe impl Send for AudioEncoder {}

impl AudioEncoder {
    pub fn new(sample_rate: i32, channels: isize) -> CoreResult<Self> {
        unsafe {
            let mut error_code = 0;
            let enc = opus_encoder_create(
                sample_rate,
                channels,
                OPUS_APPLICATION_RESTRICTED_LOWDELAY,
                &mut error_code,
            );

            if enc.is_null() {
                return Err(core_error!("opus_encoder_create returns null"));
            }

            if error_code != 0 {
                return Err(core_error!(
                    "opus_encoder_create returns error code: {}",
                    error_code
                ));
            }

            Ok(Self {
                enc,
                enc_buffer: [0u8; 4000],
                sample_rate,
                channels,
            })
        }
    }

    pub fn encode(&mut self, pcm: &[f32]) -> CoreResult<Vec<u8>> {
        unsafe {
            let ret = opus_encode_float(
                self.enc,
                pcm.as_ptr(),
                (pcm.len() as isize) / self.channels,
                self.enc_buffer.as_mut_ptr(),
                self.enc_buffer.len() as i32,
            );

            if ret < 0 {
                return Err(core_error!("opus_encode_float returns error code: {}", ret));
            }

            Ok(self.enc_buffer[0..ret as usize].to_vec())
        }
    }
}

impl Drop for AudioEncoder {
    fn drop(&mut self) {
        if !self.enc.is_null() {
            unsafe {
                opus_encoder_destroy(self.enc);
            }
        }
    }
}
