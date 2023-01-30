use crate::{component::frame::AudioEncodeFrame, core_error, error::CoreResult};
use cpal::{
    traits::{DeviceTrait, HostTrait},
    Stream, StreamConfig,
};
use tokio::sync::mpsc::Receiver;

pub fn new_record_stream_and_rx() -> CoreResult<(Stream, Receiver<AudioEncodeFrame>)> {
    let host = cpal::default_host();

    let device = match host.default_output_device() {
        Some(device) => device,
        None => {
            return Err(core_error!("default audio output device not exist"));
        }
    };

    tracing::info!(name = ?device.name(), "select default audio output device");

    let config = device.default_output_config()?;
    tracing::info!(?config, "audio default output config");

    let channels = config.channels();
    let sample_format = config.sample_format();
    let sample_rate = config.sample_rate().0;

    let config = StreamConfig {
        channels: config.channels(),
        sample_rate: config.sample_rate(),
        buffer_size: cpal::BufferSize::Fixed(960),
    };

    let (tx, rx) = tokio::sync::mpsc::channel(180);
    let error_handler = |err| tracing::error!(?err, "error occurred on the output input stream");
    let stream = device.build_input_stream_raw(
        &config,
        sample_format,
        move |data, _| {
            let audio_encode_frame = AudioEncodeFrame {
                channels,
                sample_format: data.sample_format(),
                sample_rate,
                buffer: data.bytes().to_vec(),
            };

            if tx.blocking_send(audio_encode_frame).is_err() {
                tracing::warn!("audio encode frame tx try send failed!");
            }
        },
        error_handler,
        None,
    )?;

    Ok((stream, rx))
}
