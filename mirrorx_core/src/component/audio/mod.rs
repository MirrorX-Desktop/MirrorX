pub mod decoder;
pub mod duplicator;
pub mod encoder;
pub mod player;
pub mod resampler;

// pub fn enum_audio_device(input: bool) -> CoreResult<Vec<String>> {
//     let host = cpal::default_host();
//     let devices = host.output_devices()?;
//     let device_names = devices.filter_map(|device| device.name().ok()).collect();
//     Ok(device_names)
// }
