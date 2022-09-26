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
use once_cell::sync::OnceCell;
use tokio::sync::mpsc::{error::TrySendError, Sender};

pub struct AudioEncoder {
    encode_context: Option<EncodeContext>,
    tx: Sender<Option<EndPointMessage>>,
}

impl AudioEncoder {
    pub fn new(tx: Sender<Option<EndPointMessage>>) -> CoreResult<Self> {
        Ok(AudioEncoder {
            encode_context: None,

            tx,
        })
    }

    pub fn encode(&mut self, audio_frame: AudioEncodeFrame) -> CoreResult<()> {
        if let Some((sample_rate, channels)) = audio_frame.initial_encoder_params {
            let encode_context = EncodeContext::new(
                sample_rate,
                channels,
                (audio_frame.bytes.len() / (channels as usize)) as u16,
            )?;

            self.encode_context = Some(encode_context);
        }

        if let Some(encode_context) = &mut self.encode_context {
            let params = encode_context.initial_params.take();
            let buffer = encode_context.encode(&audio_frame.bytes)?;

            let packet = EndPointMessage::AudioFrame(EndPointAudioFrame {
                params,
                buffer: buffer.to_vec(),
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
        } else {
            Err(core_error!("audio encode context not initialized"))
        }
    }
}

struct EncodeContext {
    enc: *mut OpusEncoder,
    enc_buffer: Vec<u8>,
    frame_size: u16,
    initial_params: OnceCell<(u32, AudioSampleFormat, u8, u16)>, // sample_rate, sample_format, channels, frame_size
}

impl EncodeContext {
    pub fn new(sample_rate: u32, channels: u8, frame_size: u16) -> CoreResult<Self> {
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

            let buffer_size = frame_size * (channels as u16) * 4; // todo: 4
            let mut enc_buffer = Vec::new();
            enc_buffer.resize(buffer_size as usize, 0);

            Ok(EncodeContext {
                enc,
                enc_buffer,
                frame_size,
                initial_params: OnceCell::with_value((
                    sample_rate,
                    AudioSampleFormat::F32,
                    channels,
                    frame_size,
                )),
            })
        }
    }

    pub fn encode(&mut self, buffer: &[f32]) -> CoreResult<&[u8]> {
        unsafe {
            let ret = opus_encode_float(
                self.enc,
                buffer.as_ptr(),
                self.frame_size as isize,
                self.enc_buffer.as_mut_ptr(),
                self.enc_buffer.len() as i32,
            );

            if ret < 0 {
                return Err(core_error!("opus_encode_float returns error code: {}", ret));
            }

            let buffer = self.enc_buffer.as_slice();

            Ok(&buffer[0..(ret as usize)])
        }
    }
}

impl Drop for EncodeContext {
    fn drop(&mut self) {
        if !self.enc.is_null() {
            unsafe {
                opus_encoder_destroy(self.enc);
            }
        }
    }
}
