use anyhow::bail;
use coreaudio::audio_unit::{
    audio_format::LinearPcmFlags, render_callback::data::Raw, Element, SampleFormat, Scope,
    StreamFormat,
};
use tracing::info;

#[test]
pub fn test_audio_device() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let audio_device_ids = coreaudio::audio_unit::macos_helpers::get_audio_device_ids()
        .map_err(|err| anyhow::anyhow!(err))?;

    for (index, audio_device_id) in audio_device_ids.iter().enumerate() {
        let name = coreaudio::audio_unit::macos_helpers::get_device_name(*audio_device_id)
            .map_err(|err| anyhow::anyhow!(err))?;

        info!(?name, "audio device {}", index);
    }

    let default_output_audio_device_id =
        match coreaudio::audio_unit::macos_helpers::get_default_device_id(false) {
            Some(default_output_audio_device_id) => {
                let name = coreaudio::audio_unit::macos_helpers::get_device_name(
                    default_output_audio_device_id,
                )
                .map_err(|err| anyhow::anyhow!(err))?;

                info!(?name, "default output audio device");
                default_output_audio_device_id
            }
            None => {
                bail!("non default output audio device")
            }
        };

    let mut au = coreaudio::audio_unit::macos_helpers::audio_unit_from_device_id(
        default_output_audio_device_id,
        true,
    )
    .map_err(|err| anyhow::anyhow!(err))?;

    let format_flag = match coreaudio::audio_unit::SampleFormat::F32 {
        SampleFormat::F32 => LinearPcmFlags::IS_FLOAT,
        SampleFormat::I32 | SampleFormat::I16 | SampleFormat::I8 => {
            LinearPcmFlags::IS_SIGNED_INTEGER
        }
        _ => {
            unimplemented!("Other formats are not implemented for this example.");
        }
    };

    let in_stream_format = StreamFormat {
        sample_rate: 48000f64,
        sample_format: coreaudio::audio_unit::SampleFormat::F32,
        flags: format_flag | LinearPcmFlags::IS_PACKED,
        // audio_unit.set_input_callback is hardcoded to 1 buffer, and when using non_interleaved
        // we are forced to 1 channel
        channels: 2,
    };

    let asbd = in_stream_format.to_asbd();
    au.set_property(
        coreaudio::sys::kAudioUnitProperty_StreamFormat,
        Scope::Output,
        Element::Input,
        Some(&asbd),
    )
    .map_err(|err| anyhow::anyhow!(err))?;

    au.set_input_callback::<_, Raw>(|args| {
        info!(num_frames = ?args.num_frames, sample_time = ?args.time_stamp.mSampleTime, "audio callback");
        Ok(())
    })
    .map_err(|err| anyhow::anyhow!(err))?;

    // au.initialize().map_err(|err| anyhow::anyhow!(err))?;

    au.start().map_err(|err| anyhow::anyhow!(err))?;

    std::thread::sleep(std::time::Duration::from_secs(10));

    au.stop().map_err(|err| anyhow::anyhow!(err))?;

    Ok(())
}
