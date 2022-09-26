use crate::{
    api::endpoint::message::{AudioSampleFormat, EndPointAudioFrame, EndPointMessage},
    component::frame::AudioEncodeFrame,
    core_error,
    error::{CoreError, CoreResult},
    ffi::opus::encoder::{
        opus_encode_float, opus_encoder_create, opus_encoder_destroy, OpusEncoder,
        OPUS_APPLICATION_RESTRICTED_LOWDELAY,
    },
};
use tokio::sync::mpsc::{error::TrySendError, Sender};

pub struct AudioEncoder {
    enc: *mut OpusEncoder,
    enc_buffer: [u8; 11520],
    sample_rate: u32,
    channels: u8,
    tx: Sender<Option<EndPointMessage>>,
    initial_data: once_cell::unsync::OnceCell<(u32, AudioSampleFormat, u8)>,
}

impl AudioEncoder {
    pub fn new(
        sample_rate: u32,
        channels: u8,
        tx: Sender<Option<EndPointMessage>>,
    ) -> CoreResult<Self> {
        unsafe {
            let mut error_code = 0;
            let enc = opus_encoder_create(
                sample_rate as i32,
                channels as isize,
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
                enc_buffer: [0u8; 11520],
                sample_rate,
                channels,
                tx,
                initial_data: once_cell::unsync::OnceCell::with_value((
                    sample_rate,
                    AudioSampleFormat::F32,
                    channels,
                )),
            })
        }
    }

    pub fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    pub fn channels(&self) -> u8 {
        self.channels
    }

    pub fn encode(&mut self, audio_frame: AudioEncodeFrame) -> CoreResult<()> {
        // todo: split f32 and i16 or u16 encode
        unsafe {
            let ret = opus_encode_float(
                self.enc,
                audio_frame.bytes.as_ptr(),
                (audio_frame.bytes.len() as isize) / (self.channels as isize),
                self.enc_buffer.as_mut_ptr(),
                self.enc_buffer.len() as i32,
            );

            if ret < 0 {
                return Err(core_error!("opus_encode_float returns error code: {}", ret));
            }

            let buffer = self.enc_buffer[0..ret as usize].to_vec();

            let packet = EndPointMessage::AudioFrame(EndPointAudioFrame {
                re_init_data: self.initial_data.take(),
                buffer,
            });

            if let Err(err) = self.tx.try_send(Some(packet)) {
                if let TrySendError::Full(_) = err {
                    tracing::warn!("audio encoder send EndPointMessage failed, channel is full!");
                } else {
                    return Err(core_error!(
                        "video encoder send EndPointMessage failed, channel is closed"
                    ));
                }
            };

            Ok(())
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
