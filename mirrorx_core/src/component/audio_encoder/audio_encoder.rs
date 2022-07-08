use crate::{
    error::MirrorXError,
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
    pub fn new(sample_rate: i32, channels: isize) -> Result<Self, MirrorXError> {
        unsafe {
            let mut err: isize = 0;
            let enc = opus_encoder_create(
                sample_rate,
                channels,
                OPUS_APPLICATION_RESTRICTED_LOWDELAY,
                &mut err as *mut _,
            );

            if err != 0 || enc.is_null() {
                return Err(MirrorXError::Other(anyhow::anyhow!(
                    "initialize opus encoder failed"
                )));
            }

            Ok(Self {
                enc,
                enc_buffer: [0u8; 4000],
                sample_rate,
                channels,
            })
        }
    }

    pub fn encode(&mut self, pcm: &[f32]) -> Result<Vec<u8>, MirrorXError> {
        if self.enc.is_null() {
            return Err(MirrorXError::Other(anyhow::anyhow!("opus encoder is null")));
        }

        unsafe {
            let ret = opus_encode_float(
                self.enc,
                pcm.as_ptr(),
                (pcm.len() as isize) / self.channels,
                self.enc_buffer.as_mut_ptr(),
                self.enc_buffer.len() as i32,
            );

            if ret < 0 {
                return Err(MirrorXError::Other(anyhow::anyhow!(
                    "opus encode failed ({})",
                    ret
                )));
            }

            let data = self.enc_buffer[0..ret as usize].to_vec();
            return Ok(data);
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
