use crate::{core_error, error::CoreResult};
use cpal::{
    traits::{DeviceTrait, HostTrait},
    SampleFormat, SampleRate, SizedSample, Stream, StreamConfig, SupportedStreamConfig,
};
use tokio::sync::mpsc::{Receiver, Sender};

pub fn default_output_config() -> CoreResult<SupportedStreamConfig> {
    let host = cpal::default_host();

    let device = match host.default_output_device() {
        Some(device) => device,
        None => {
            return Err(core_error!("default audio output device not exist"));
        }
    };
    tracing::info!(name = ?device.name(), "select audio output device");

    Ok(device.default_output_config()?)
}

pub fn new_play_stream_and_tx(
    channels: u16,
    sample_format: SampleFormat,
    sample_rate: SampleRate,
    buffer_size: u32,
) -> CoreResult<(Stream, Sender<Vec<u8>>)> {
    let host = cpal::default_host();

    let device = match host.default_output_device() {
        Some(device) => device,
        None => {
            return Err(core_error!("default audio output device not exist"));
        }
    };
    tracing::info!(name = ?device.name(), "select audio output device");

    tracing::info!(
        ?channels,
        ?sample_format,
        ?sample_rate,
        "select audio stream config"
    );

    let output_config = StreamConfig {
        channels,
        sample_rate,
        buffer_size: cpal::BufferSize::Fixed(buffer_size),
    };

    let (tx, mut rx) = tokio::sync::mpsc::channel::<Vec<u8>>(180);
    let err_fn = |err| tracing::error!(?err, "an error occurred when play audio sample");

    let stream = match sample_format {
        SampleFormat::I16 => device.build_output_stream(
            &output_config,
            move |data, _| play_samples::<i16>(data, &mut rx),
            err_fn,
            None,
        ),
        SampleFormat::U16 => device.build_output_stream(
            &output_config,
            move |data, _| play_samples::<u16>(data, &mut rx),
            err_fn,
            None,
        ),
        SampleFormat::F32 => device.build_output_stream(
            &output_config,
            move |data, _| play_samples::<f32>(data, &mut rx),
            err_fn,
            None,
        ),
        _ => {
            return Err(core_error!(
                "unsupported sample format: {:?}",
                sample_format
            ))
        }
    }?;

    Ok((stream, tx))
}

fn play_samples<T>(data: &mut [T], rx: &mut Receiver<Vec<u8>>)
where
    T: SizedSample,
{
    if let Some(samples) = rx.blocking_recv() {
        unsafe {
            std::ptr::copy_nonoverlapping(
                std::mem::transmute(samples.as_ptr()),
                data.as_mut_ptr(),
                (samples.len() / T::FORMAT.sample_size()).min(data.len()),
            )
        }
    };
}
