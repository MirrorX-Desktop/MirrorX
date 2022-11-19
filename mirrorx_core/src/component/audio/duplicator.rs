use crate::{component::frame::AudioEncodeFrame, core_error, error::CoreResult};
use cpal::{
    traits::{DeviceTrait, HostTrait},
    Sample, SampleRate, Stream,
};
use tokio::sync::mpsc::{Receiver, Sender};

pub fn new_record_stream_and_rx() -> CoreResult<(Stream, Receiver<AudioEncodeFrame>)> {
    let host = cpal::default_host();

    let device = match host.default_output_device() {
        Some(device) => device,
        None => {
            return Err(core_error!("default audio output device not exist"));
        }
    };

    tracing::info!(name = ?device.name(), "select default audio output device");

    let output_config = device.default_output_config()?;
    tracing::info!(?output_config, "select audio config");

    let channels = output_config.channels();
    let sample_rate = output_config.sample_rate().0;

    let (audio_encode_frame_tx, audio_encode_frame_rx) = tokio::sync::mpsc::channel(180);
    let err_fn = |err| tracing::error!(?err, "error occurred on the output input stream");

    let stream = match output_config.sample_format() {
        cpal::SampleFormat::I16 => device.build_input_stream(
            &output_config.into(),
            move |data, _| {
                send_audio_frame::<i16>(data, channels, sample_rate, &audio_encode_frame_tx)
            },
            err_fn,
        ),
        cpal::SampleFormat::U16 => device.build_input_stream(
            &output_config.into(),
            move |data, _| {
                send_audio_frame::<u16>(data, channels, sample_rate, &audio_encode_frame_tx)
            },
            err_fn,
        ),
        cpal::SampleFormat::F32 => device.build_input_stream(
            &output_config.into(),
            move |data, _| {
                send_audio_frame::<f32>(data, channels, sample_rate, &audio_encode_frame_tx)
            },
            err_fn,
        ),
    }?;

    Ok((stream, audio_encode_frame_rx))
}

fn send_audio_frame<T>(data: &[T], channels: u16, sample_rate: u32, tx: &Sender<AudioEncodeFrame>)
where
    T: Sample,
{
    let buffer_length = match T::FORMAT {
        cpal::SampleFormat::I16 => data.len() * 2,
        cpal::SampleFormat::U16 => data.len() * 2,
        cpal::SampleFormat::F32 => data.len() * 4,
    };

    let audio_encode_frame = AudioEncodeFrame {
        channels,
        sample_format: T::FORMAT,
        sample_rate,
        buffer: unsafe {
            std::slice::from_raw_parts(data.as_ptr() as *const u8, buffer_length).to_vec()
        },
    };

    if tx.try_send(audio_encode_frame).is_err() {
        tracing::warn!("audio encode frame tx try send failed!");
    }
}
