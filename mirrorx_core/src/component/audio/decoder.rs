use crate::{
    api::endpoint::message::EndPointAudioFrame,
    core_error,
    error::{CoreError, CoreResult},
    ffi::opus::decoder::{
        opus_decode_float, opus_decoder_create, opus_decoder_destroy, OpusDecoder,
    },
};
use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    BufferSize, OutputCallbackInfo, SampleRate,
};
use tokio::sync::mpsc::{error::TryRecvError, Receiver};

pub struct AudioPlayer {
    sample_rate: u32,
    channels: u8,
    frame_size: u16,
    decode_context: Option<DecodeContext>,
    playback_context: Option<PlaybackContext>,
}

impl AudioPlayer {
    pub fn new() -> Self {
        AudioPlayer {
            sample_rate: 0,
            channels: 0,
            frame_size: 0,
            decode_context: None,
            playback_context: None,
        }
    }

    pub fn play_samples(&mut self, audio_frame: EndPointAudioFrame) -> CoreResult<()> {
        if let Some((sample_rate, sample_format, channels, frame_size)) = audio_frame.params {
            tracing::info!(
                "sample_rate: {}, channels: {}, frame_size:{}",
                sample_rate,
                channels,
                frame_size
            );

            let decode_context = DecodeContext::new(sample_rate, channels, frame_size)?;
            let playback_context =
                PlaybackContext::new(sample_rate, channels, self.frame_size as usize)?;

            self.sample_rate = sample_rate;
            self.channels = channels;
            self.frame_size = frame_size;
            self.decode_context = Some(decode_context);
            self.playback_context = Some(playback_context);
        }

        let decoded_buffer = if let Some(decode_context) = self.decode_context.as_mut() {
            decode_context.decode(&audio_frame.buffer)
        } else {
            Err(core_error!("audio decode context not initialized"))
        }?;

        let success = if let Some(playback_context) = self.playback_context.as_mut() {
            Ok(playback_context.enqueue_samples(decoded_buffer))
        } else {
            Err(core_error!("audio playback context not initialized"))
        }?;

        if !success {
            let mut playback_context =
                PlaybackContext::new(self.sample_rate, self.channels, self.frame_size as usize)?;

            if !playback_context.enqueue_samples(decoded_buffer) {
                return Err(core_error!("too many playback context initialize failures"));
            }

            self.playback_context = Some(playback_context);
        }

        Ok(())
    }
}

struct DecodeContext {
    dec: *mut OpusDecoder,
    dec_buffer: Vec<f32>,
    frame_size: u16,
}

impl DecodeContext {
    pub fn new(sample_rate: u32, channels: u8, frame_size: u16) -> CoreResult<Self> {
        unsafe {
            let mut error_code = 0;
            let dec = opus_decoder_create(sample_rate as i32, channels as isize, &mut error_code);

            if dec.is_null() {
                return Err(core_error!("opus_decoder_create returns null"));
            }

            if error_code != 0 {
                return Err(core_error!(
                    "opus_decoder_create returns error code: {}",
                    error_code
                ));
            }

            let buffer_size = frame_size * (channels as u16);
            let mut dec_buffer = Vec::new();
            dec_buffer.resize(buffer_size as usize, 0f32);

            Ok(Self {
                dec,
                dec_buffer,
                frame_size,
            })
        }
    }

    pub fn decode(&mut self, buffer: &[u8]) -> CoreResult<&[f32]> {
        unsafe {
            let ret = opus_decode_float(
                self.dec,
                buffer.as_ptr(),
                buffer.len() as i32,
                self.dec_buffer.as_mut_ptr(),
                (self.frame_size) as isize,
                0,
            );

            if ret < 0 {
                return Err(core_error!("opus_decode_float returns error code: {}", ret));
            }

            tracing::info!("decode ret: {}", ret);

            Ok(&self.dec_buffer[0..(ret as usize)])
        }
    }
}

impl Drop for DecodeContext {
    fn drop(&mut self) {
        if !self.dec.is_null() {
            unsafe {
                opus_decoder_destroy(self.dec);
            }
        }
    }
}

struct PlaybackContext {
    stream: cpal::Stream,
    audio_sample_tx: rtrb::Producer<f32>,
    callback_exit_rx: Receiver<()>,
}

impl PlaybackContext {
    pub fn new(sample_rate: u32, channels: u8, buffer_size: usize) -> CoreResult<Self> {
        let host = cpal::default_host();

        let device = match host.default_output_device() {
            Some(device) => device,
            None => {
                return Err(core_error!("default audio output device not exist"));
            }
        };

        tracing::info!(name = ?device.name(), "select audio output device");

        let stream_config = cpal::StreamConfig {
            channels: channels as u16,
            sample_rate: SampleRate(sample_rate),
            buffer_size: BufferSize::Fixed(buffer_size as u32),
        };

        let (audio_sample_tx, mut audio_sample_rx) = rtrb::RingBuffer::new(180);
        let (callback_exit_tx, callback_exit_rx) = tokio::sync::mpsc::channel(1);
        let err_callback_exit_tx = callback_exit_tx.clone();

        let input_callback =
            move |data: &mut [f32], info: &OutputCallbackInfo| match audio_sample_rx
                .read_chunk(data.len().min(audio_sample_rx.slots()))
            {
                Ok(chunk) => {
                    for (i, v) in chunk.into_iter().enumerate() {
                        data[i] = v;
                    }
                }
                Err(err) => {
                    tracing::error!(?err, "audio sample rx required invalid slots capacity");
                    let _ = callback_exit_tx.send(());
                }
            };

        let err_callback = move |err| {
            tracing::error!(?err, "error occurred on the output audio stream");
            let _ = err_callback_exit_tx.send(());
        };

        let stream = device.build_output_stream(&stream_config, input_callback, err_callback)?;
        stream.play()?;

        Ok(PlaybackContext {
            stream,
            audio_sample_tx,
            callback_exit_rx,
        })
    }

    pub fn enqueue_samples(&mut self, mut buffer: &[f32]) -> bool {
        match self.callback_exit_rx.try_recv() {
            Ok(_) => return false,
            Err(err) => {
                if err == TryRecvError::Disconnected {
                    return false;
                }
            }
        };

        while !(buffer).is_empty() {
            match self
                .audio_sample_tx
                .write_chunk_uninit(buffer.len().min(self.audio_sample_tx.slots()))
            {
                Ok(mut chunk) => {
                    let (slice1, slice2) = chunk.as_mut_slices();

                    let mut total_copy_length = 0;
                    let mut copy_length = slice1.len().min(buffer.len());
                    unsafe {
                        std::ptr::copy_nonoverlapping(
                            buffer.as_ptr(),
                            slice1.as_mut_ptr() as *mut f32,
                            copy_length,
                        );
                    }
                    total_copy_length += copy_length;
                    buffer = &buffer[copy_length..];

                    if !buffer.is_empty() && !slice2.is_empty() {
                        copy_length = slice2.len().min(buffer.len());
                        unsafe {
                            std::ptr::copy_nonoverlapping(
                                buffer.as_ptr(),
                                slice2.as_mut_ptr() as *mut f32,
                                copy_length,
                            );
                        }
                        total_copy_length += copy_length;
                        buffer = &buffer[copy_length..];
                    }

                    unsafe { chunk.commit(total_copy_length) };
                }
                Err(err) => {
                    tracing::error!(?err, "audio sample tx required invalid slots capacity");
                    return false;
                }
            }
        }

        true
    }

    pub fn pause(&self) {
        let _ = self.stream.pause();
    }
}

impl Drop for PlaybackContext {
    fn drop(&mut self) {
        self.pause()
    }
}
