use crate::{core_error, error::CoreResult};
use cpal::{
    traits::{DeviceTrait, HostTrait},
    Sample, SampleFormat, SampleRate, Stream, StreamConfig,
};
use tokio::sync::mpsc::{error::TryRecvError, Receiver, Sender};

pub type StreamAndTx = (Stream, u16, SampleFormat, SampleRate, Sender<Vec<u8>>);

pub fn new_play_stream_and_tx() -> CoreResult<StreamAndTx> {
    let host = cpal::default_host();

    let device = match host.default_output_device() {
        Some(device) => device,
        None => {
            return Err(core_error!("default audio output device not exist"));
        }
    };
    tracing::info!(name = ?device.name(), "select audio output device");

    let supported_output_config = device.default_output_config()?;
    let channels = supported_output_config.channels();
    let sample_format = supported_output_config.sample_format();
    let sample_rate = supported_output_config.sample_rate();
    tracing::info!(?supported_output_config, "select audio stream config");

    let output_config = StreamConfig {
        channels,
        sample_rate,
        buffer_size: cpal::BufferSize::Fixed(960),
    };

    let (tx, mut rx) = tokio::sync::mpsc::channel::<Vec<u8>>(180);
    let err_fn = |err| tracing::error!(?err, "an error occurred when play audio sample");

    let stream = match supported_output_config.sample_format() {
        SampleFormat::I16 => device.build_output_stream(
            &output_config,
            move |data, _| play_samples::<i16>(data, &mut rx),
            err_fn,
        ),
        SampleFormat::U16 => device.build_output_stream(
            &output_config,
            move |data, _| play_samples::<u16>(data, &mut rx),
            err_fn,
        ),
        SampleFormat::F32 => device.build_output_stream(
            &output_config,
            move |data, _| play_samples::<f32>(data, &mut rx),
            err_fn,
        ),
    }?;

    Ok((stream, channels, sample_format, sample_rate, tx))
}

fn play_samples<T>(data: &mut [T], rx: &mut Receiver<Vec<u8>>)
where
    T: Sample,
{
    match rx.try_recv() {
        Ok(samples) => unsafe {
            std::ptr::copy_nonoverlapping(
                std::mem::transmute(samples.as_ptr()),
                data.as_mut_ptr(),
                (samples.len() / T::FORMAT.sample_size()).min(data.len()),
            )
        },
        Err(err) => {
            if err == TryRecvError::Disconnected {
                // let _ = callback_exit_tx.try_send(());
            }
        }
    };
}
