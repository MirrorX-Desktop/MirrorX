use crate::{
    core_error,
    error::{CoreError, CoreResult},
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
    pub fn new(sample_rate: i32, channels: isize) -> CoreResult<Self> {
        unsafe {
            let mut error_code = 0;
            let dec = opus_decoder_create(sample_rate, channels, &mut error_code);

            if dec.is_null() {
                return Err(core_error!("opus_decoder_create returns null"));
            }

            if error_code != 0 {
                return Err(core_error!(
                    "opus_decoder_create returns error code: {}",
                    error_code
                ));
            }

            Ok(Self {
                dec,
                dec_buffer: [0f32; 2880],
                sample_rate,
                channels,
            })
        }
    }

    pub fn decode(&mut self, data: &[u8], frame_size_per_channel: u16) -> CoreResult<Vec<f32>> {
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
                return Err(core_error!("opus_decode_float returns error code: {}", ret));
            }

            let data = self.dec_buffer
                [0..(frame_size_per_channel as usize) * (self.channels as usize)]
                .to_vec();

            Ok(data)
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
