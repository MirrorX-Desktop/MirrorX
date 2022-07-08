use crate::{
    error::MirrorXError,
    ffi::opus::decoder::{
        opus_decode_float, opus_decoder_create, opus_decoder_destroy, OpusDecoder,
    },
};

pub struct AudioDecoder {
    dec: *mut OpusDecoder,
    dec_buffer: [f32; 2880],
    sample_rate: i32,
    channels: isize,
}

unsafe impl Send for AudioDecoder {}

impl AudioDecoder {
    pub fn new(sample_rate: i32, channels: isize) -> Result<Self, MirrorXError> {
        unsafe {
            let mut err: isize = 0;
            let dec = opus_decoder_create(sample_rate, channels, &mut err as *mut _);

            if err != 0 || dec.is_null() {
                return Err(MirrorXError::Other(anyhow::anyhow!(
                    "initialize opus decoder failed"
                )));
            }

            Ok(Self {
                dec,
                dec_buffer: [0f32; 2880],
                sample_rate,
                channels,
            })
        }
    }

    pub fn decode(
        &mut self,
        data: &[u8],
        frame_size_per_channel: u16,
    ) -> Result<Vec<f32>, MirrorXError> {
        if self.dec.is_null() {
            return Err(MirrorXError::Other(anyhow::anyhow!("opus encoder is null")));
        }

        unsafe {
            let ret = opus_decode_float(
                self.dec,
                data.as_ptr(),
                data.len() as i32,
                self.dec_buffer.as_mut_ptr(),
                frame_size_per_channel as isize,
                0,
            );

            if ret < 0 {
                return Err(MirrorXError::Other(anyhow::anyhow!(
                    "opus encode failed ({})",
                    ret
                )));
            }

            let data = self.dec_buffer[0..ret as usize].to_vec();
            return Ok(data);
        }
    }
}

impl Drop for AudioDecoder {
    fn drop(&mut self) {
        if !self.dec.is_null() {
            unsafe {
                opus_decoder_destroy(self.dec);
            }
        }
    }
}
