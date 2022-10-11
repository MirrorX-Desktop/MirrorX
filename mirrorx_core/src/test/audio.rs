use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    InputCallbackInfo, Sample, SampleFormat, StreamConfig, SupportedStreamConfigRange,
};
use tracing::info;

#[test]
pub fn test_audio_device() -> anyhow::Result<()> {
    // tracing_subscriber::fmt::init();

    let host = cpal::default_host();

    let input_devices = host.input_devices().map_err(|err| anyhow::anyhow!(err))?;
    for (index, device) in input_devices.into_iter().enumerate() {
        info!(name=?device.name(),"audio input device {}",index);
    }

    let output_devices = host.output_devices().map_err(|err| anyhow::anyhow!(err))?;
    for (index, device) in output_devices.into_iter().enumerate() {
        info!(name=?device.name(),"audio output device {}",index);
    }

    let device = host
        .default_output_device()
        .ok_or(anyhow::anyhow!("default device is null"))?;

    info!(name=?device.name(),"select default audio output device");

    let supported_configs = device
        .supported_output_configs()
        .map_err(|err| anyhow::anyhow!(err))?;

    let supported_config_vec: Vec<SupportedStreamConfigRange> =
        supported_configs.into_iter().collect();

    if supported_config_vec.len() == 0 {
        info!("no supported audio device output config, exit");
        return Ok(());
    }

    for (index, config) in supported_config_vec.iter().enumerate() {
        info!(
            buffer_size=?config.buffer_size(),
            channels=?config.channels(),
            max_sample_rate=?config.max_sample_rate(),
            min_sample_rate=?config.min_sample_rate(),
            "audio device output config {}", index,
        );
    }

    let output_config = supported_config_vec[0]
        .clone()
        .with_max_sample_rate()
        .config();

    let err_fn = |err| eprintln!("an error occurred on the output audio stream: {}", err);

    let stream = match *&supported_config_vec[0].sample_format() {
        SampleFormat::F32 => {
            device.build_input_stream(&output_config, write_silence::<f32>, err_fn)
        }
        SampleFormat::I16 => {
            device.build_input_stream(&output_config, write_silence::<i16>, err_fn)
        }
        SampleFormat::U16 => {
            device.build_input_stream(&output_config, write_silence::<u16>, err_fn)
        }
    }
    .map_err(|err| anyhow::anyhow!(err))?;

    stream.play().map_err(|err| anyhow::anyhow!(err))?;
    std::thread::sleep(std::time::Duration::from_secs(10));
    stream.pause().map_err(|err| anyhow::anyhow!(err))?;

    Ok(())
}

fn write_silence<T: Sample>(data: &[T], v: &InputCallbackInfo) {
    let buffer: Vec<f32> = data.iter().map(|b| b.to_f32()).collect();
    info!(buffer_len=?buffer.len(),buffer_begin_8=?&buffer[0..8],callback_info=?v, "data receive");
}
