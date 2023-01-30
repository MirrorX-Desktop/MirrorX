use crate::{component::frame::AudioEncodeFrame, core_error, error::CoreResult};
use cpal::{
    traits::{DeviceTrait, HostTrait},
    SizedSample, Stream, StreamConfig,
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

    let supported_output_config = device.default_output_config()?;
    tracing::info!(?supported_output_config, "select audio config");

    let channels = supported_output_config.channels();
    let sample_rate = supported_output_config.sample_rate().0;

    let output_config = StreamConfig {
        channels: supported_output_config.channels(),
        sample_rate: supported_output_config.sample_rate(),
        buffer_size: cpal::BufferSize::Fixed(960),
    };

    let (audio_encode_frame_tx, audio_encode_frame_rx) = tokio::sync::mpsc::channel(180);
    let err_fn = |err| tracing::error!(?err, "error occurred on the output input stream");

    let stream = match supported_output_config.sample_format() {
        cpal::SampleFormat::I8 => device.build_input_stream(
            &output_config,
            move |data, _| {
                send_audio_frame::<i8>(data, channels, sample_rate, &audio_encode_frame_tx)
            },
            err_fn,
            None,
        ),
        cpal::SampleFormat::U8 => device.build_input_stream(
            &output_config,
            move |data, _| {
                send_audio_frame::<u8>(data, channels, sample_rate, &audio_encode_frame_tx)
            },
            err_fn,
            None,
        ),
        cpal::SampleFormat::I16 => device.build_input_stream(
            &output_config,
            move |data, _| {
                send_audio_frame::<i16>(data, channels, sample_rate, &audio_encode_frame_tx)
            },
            err_fn,
            None,
        ),
        cpal::SampleFormat::U16 => device.build_input_stream(
            &output_config,
            move |data, _| {
                send_audio_frame::<u16>(data, channels, sample_rate, &audio_encode_frame_tx)
            },
            err_fn,
            None,
        ),
        cpal::SampleFormat::I32 => device.build_input_stream(
            &output_config,
            move |data, _| {
                send_audio_frame::<i32>(data, channels, sample_rate, &audio_encode_frame_tx)
            },
            err_fn,
            None,
        ),
        cpal::SampleFormat::U32 => device.build_input_stream(
            &output_config,
            move |data, _| {
                send_audio_frame::<u32>(data, channels, sample_rate, &audio_encode_frame_tx)
            },
            err_fn,
            None,
        ),
        cpal::SampleFormat::I64 => device.build_input_stream(
            &output_config,
            move |data, _| {
                send_audio_frame::<i64>(data, channels, sample_rate, &audio_encode_frame_tx)
            },
            err_fn,
            None,
        ),
        cpal::SampleFormat::U64 => device.build_input_stream(
            &output_config,
            move |data, _| {
                send_audio_frame::<u64>(data, channels, sample_rate, &audio_encode_frame_tx)
            },
            err_fn,
            None,
        ),
        cpal::SampleFormat::F32 => device.build_input_stream(
            &output_config,
            move |data, _| {
                send_audio_frame::<f32>(data, channels, sample_rate, &audio_encode_frame_tx)
            },
            err_fn,
            None,
        ),
        cpal::SampleFormat::F64 => device.build_input_stream(
            &output_config,
            move |data, _| {
                send_audio_frame::<f64>(data, channels, sample_rate, &audio_encode_frame_tx)
            },
            err_fn,
            None,
        ),
        _ => {
            return Err(core_error!(
                "unsupported sample format: {}",
                supported_output_config.sample_format()
            ))
        }
    }?;

    Ok((stream, audio_encode_frame_rx))
}

fn send_audio_frame<T>(data: &[T], channels: u16, sample_rate: u32, tx: &Sender<AudioEncodeFrame>)
where
    T: SizedSample,
{
    let buffer = unsafe {
        std::slice::from_raw_parts(
            data.as_ptr() as *const u8,
            data.len() * T::FORMAT.sample_size(),
        )
    };

    let audio_encode_frame = AudioEncodeFrame {
        channels,
        sample_format: T::FORMAT,
        sample_rate,
        buffer: buffer.to_vec(),
    };

    if tx.blocking_send(audio_encode_frame).is_err() {
        tracing::warn!("audio encode frame tx try send failed!");
    }
}
