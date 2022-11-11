use crate::{
    api::endpoint::{message::AudioSampleFormat, EndPointClient},
    component::frame::AudioEncodeFrame,
    core_error,
    error::CoreResult,
    ffi::opus::encoder::{
        opus_encode_float, opus_encoder_create, opus_encoder_destroy, OpusEncoder,
        OPUS_APPLICATION_RESTRICTED_LOWDELAY,
    },
};
use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    InputCallbackInfo,
};
use once_cell::sync::OnceCell;
use tokio::sync::mpsc::Receiver;

pub struct AudioDuplicator {
    encode_context: Option<EncodeContext>,
    audio_stream: cpal::Stream,
    audio_frame_rx: Receiver<Option<AudioEncodeFrame>>,
    client: EndPointClient,
}

unsafe impl Send for AudioDuplicator {}

impl AudioDuplicator {
    pub fn new(client: EndPointClient) -> CoreResult<Self> {
        let (audio_stream, audio_frame_rx) = new_cpal_stream_and_rx()?;

        audio_stream.play()?;

        Ok(AudioDuplicator {
            encode_context: None,
            audio_stream,
            audio_frame_rx,
            client,
        })
    }

    pub async fn capture_samples(&mut self) -> CoreResult<()> {
        let audio_encode_frame = match self.audio_frame_rx.recv().await {
            Some(frame) => match frame {
                Some(frame) => frame,
                None => {
                    return Err(core_error!("audio duplicator callback has error occurred"));
                }
            },
            None => {
                return Err(core_error!("audio duplicator channel closed"));
            }
        };

        if let Some((sample_rate, channels)) = audio_encode_frame.initial_encoder_params {
            let encode_context = EncodeContext::new(
                sample_rate,
                channels,
                (audio_encode_frame.buffer.len() / (channels as usize)) as u16,
            )?;

            self.encode_context = Some(encode_context);
        }

        if let Some(encode_context) = &mut self.encode_context {
            let params = encode_context.initial_params.take();
            let buffer = encode_context.encode(&audio_encode_frame.buffer)?;
            self.client.send_audio_frame(params, buffer)?;
            Ok(())
        } else {
            Err(core_error!(
                "audio duplicator encode context not initialized"
            ))
        }
    }
}

impl Drop for AudioDuplicator {
    fn drop(&mut self) {
        let _ = self.audio_stream.pause();
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

            Ok(&self.enc_buffer[0..(ret as usize)])
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

pub fn new_cpal_stream_and_rx() -> CoreResult<(cpal::Stream, Receiver<Option<AudioEncodeFrame>>)> {
    let host = cpal::default_host();

    let device = match host.default_output_device() {
        Some(device) => device,
        None => {
            return Err(core_error!("default audio output device not exist"));
        }
    };

    tracing::info!(name = ?device.name(), "select default audio output device");

    let output_config = device.default_output_config()?.config();

    let mut initial_encoder_params = once_cell::unsync::OnceCell::with_value((
        output_config.sample_rate.0,
        output_config.channels as u8,
    ));

    let (audio_encode_frame_tx, audio_encode_frame_rx) = tokio::sync::mpsc::channel(180);
    let err_callback_tx = audio_encode_frame_tx.clone();

    let input_callback = move |data: &[f32], _: &InputCallbackInfo| {
        let audio_encode_frame = AudioEncodeFrame {
            initial_encoder_params: initial_encoder_params.take(),
            buffer: data.to_vec(),
        };

        if audio_encode_frame_tx
            .try_send(Some(audio_encode_frame))
            .is_err()
        {
            tracing::warn!("audio encode frame tx try send failed!");
        }
    };

    let err_callback = move |err| {
        tracing::error!(?err, "error occurred on the output input stream");
        let _ = err_callback_tx.try_send(None);
    };

    let stream = device.build_input_stream(&output_config, input_callback, err_callback)?;

    Ok((stream, audio_encode_frame_rx))
}
