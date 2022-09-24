use std::{ffi::c_void, mem::ManuallyDrop, time::Duration};

use crate::{
    component::audio_encoder::audio_encoder::AudioEncoder,
    error::{CoreError, CoreResult},
    HRESULT,
};
use scopeguard::defer;
use windows::{
    core::PCWSTR,
    Win32::{
        Devices::FunctionDiscovery::PKEY_Device_FriendlyName,
        Media::Audio::{
            eCapture, eConsole, eRender, EDataFlow, IAudioCaptureClient, IAudioClient,
            IAudioRenderClient, IMMDevice, IMMDeviceEnumerator, MMDeviceEnumerator,
            AUDCLNT_BUFFERFLAGS_SILENT, AUDCLNT_SHAREMODE_SHARED, AUDCLNT_STREAMFLAGS_LOOPBACK,
            DEVICE_STATE_ACTIVE, WAVEFORMATEX, WAVEFORMATEXTENSIBLE, WAVEFORMATEXTENSIBLE_0,
        },
        System::Com::{
            CoCreateInstance, CoInitialize, CoTaskMemFree, StructuredStorage::PropVariantClear,
            CLSCTX_ALL, STGM_READ,
        },
    },
};

const REFTIMES_PER_SEC: u32 = 10000000;
const REFTIMES_PER_MILLISEC: i64 = 10000;

pub fn enum_audio_output_device() -> CoreResult<Vec<(String, String)>> {
    unsafe {
        HRESULT!(CoInitialize(None));

        let device_enumerator: IMMDeviceEnumerator = HRESULT!(CoCreateInstance(
            &MMDeviceEnumerator as *const _,
            None,
            CLSCTX_ALL
        ));

        let device_collection =
            HRESULT!(device_enumerator.EnumAudioEndpoints(eRender, DEVICE_STATE_ACTIVE));
        let device_count = HRESULT!(device_collection.GetCount());
        let mut device_descriptions = Vec::new();
        for i in 0..device_count {
            let device = HRESULT!(device_collection.Item(i));

            let device_id_pwstr = HRESULT!(device.GetId());
            let device_id = device_id_pwstr.to_string()?;

            let prop = HRESULT!(device.OpenPropertyStore(STGM_READ));
            let mut device_friendly_name_prop_value =
                HRESULT!(prop.GetValue(&PKEY_Device_FriendlyName));
            let device_friendly_name = device_friendly_name_prop_value
                .Anonymous
                .Anonymous
                .Anonymous
                .pwszVal
                .to_string()?;

            device_descriptions.push((device_id, device_friendly_name));

            CoTaskMemFree(Some(device_id_pwstr.as_ptr() as *const _ as *const c_void));
            HRESULT!(PropVariantClear(&mut device_friendly_name_prop_value));
        }

        Ok(device_descriptions)
    }
}

pub fn capture_output_audio(device_id: Option<String>) -> CoreResult<()> {
    unsafe {
        HRESULT!(CoInitialize(None));

        let device_enumerator: IMMDeviceEnumerator = HRESULT!(CoCreateInstance(
            &MMDeviceEnumerator as *const _,
            None,
            CLSCTX_ALL
        ));

        let audio_device = {
            let specified_device = device_id.and_then(|device_id| {
                let device_id_hstring = ManuallyDrop::new(windows::core::HSTRING::from(&device_id));
                let device_id_pcwstr = PCWSTR(device_id_hstring.as_ptr());
                defer! {
                    let _ = ManuallyDrop::into_inner(device_id_hstring);
                }

                device_enumerator.GetDevice(device_id_pcwstr).ok()
            });

            if let Some(device) = specified_device {
                CoreResult::<IMMDevice>::Ok(device)
            } else {
                Ok(HRESULT!(
                    device_enumerator.GetDefaultAudioEndpoint(eRender, eConsole)
                ))
            }
        }?;

        let audio_client: IAudioClient = HRESULT!(audio_device.Activate(CLSCTX_ALL, None));

        let raw_format = HRESULT!(audio_client.GetMixFormat());
        let format = raw_format;
        defer!(CoTaskMemFree(Some(raw_format as *const _ as *const c_void)));

        let mix_format_format_tag_addr = std::ptr::addr_of!((*format).wFormatTag);
        let mix_format_format_tag = std::ptr::read_unaligned(mix_format_format_tag_addr);

        let mix_format_channels_addr = std::ptr::addr_of!((*format).nChannels);
        let mix_format_channels = std::ptr::read_unaligned(mix_format_channels_addr);

        let mix_format_samples_per_sec_addr = std::ptr::addr_of!((*format).nSamplesPerSec);
        let mix_format_samples_per_sec = std::ptr::read_unaligned(mix_format_samples_per_sec_addr);

        let mix_format_average_bytes_addr = std::ptr::addr_of!((*format).nAvgBytesPerSec);
        let mix_format_average_bytes = std::ptr::read_unaligned(mix_format_average_bytes_addr);

        let mix_format_block_align_addr = std::ptr::addr_of!((*format).nBlockAlign);
        let mix_format_block_align = std::ptr::read_unaligned(mix_format_block_align_addr);

        let mix_format_bits_per_sample_addr = std::ptr::addr_of!((*format).wBitsPerSample);
        let mix_format_bits_per_sample = std::ptr::read_unaligned(mix_format_bits_per_sample_addr);

        tracing::info!("format: {}", mix_format_format_tag);
        tracing::info!("channels: {}", mix_format_channels);
        tracing::info!("sample_per_sec: {}", mix_format_samples_per_sec); // sample rate
        tracing::info!("avg_bytes: {}", mix_format_average_bytes);
        tracing::info!("block_align: {}", mix_format_block_align);
        tracing::info!("bits_per_sample: {}", mix_format_bits_per_sample); // 16bit or 32bit

        let mut audio_encoder = AudioEncoder::new(
            mix_format_samples_per_sec as i32,
            mix_format_channels as isize,
        )?;

        HRESULT!(audio_client.Initialize(
            AUDCLNT_SHAREMODE_SHARED,
            AUDCLNT_STREAMFLAGS_LOOPBACK,
            REFTIMES_PER_SEC as i64,
            0,
            raw_format,
            None,
        ));

        let buffer_frame_count = HRESULT!(audio_client.GetBufferSize());
        let capture_client: IAudioCaptureClient = HRESULT!(audio_client.GetService());

        let hns_actual_duration = (REFTIMES_PER_SEC as u64) * (buffer_frame_count as u64)
            / (mix_format_samples_per_sec as u64);

        HRESULT!(audio_client.Start());

        let mut packet_length = 0;
        loop {
            std::thread::sleep(Duration::from_millis(hns_actual_duration));

            packet_length = HRESULT!(capture_client.GetNextPacketSize());
            while packet_length != 0 {
                let mut p_data = std::ptr::null_mut();
                let mut num_frames_available = 0;
                let mut flags = 0;

                HRESULT!(capture_client.GetBuffer(
                    &mut p_data,
                    &mut num_frames_available,
                    &mut flags,
                    None,
                    None,
                ));

                let buffer: *mut f32 = if flags & (AUDCLNT_BUFFERFLAGS_SILENT.0 as u32) != 0 {
                    // todo: tell stream it's slient
                    std::ptr::null_mut()
                } else {
                    std::mem::transmute(p_data)
                };

                let encoded_buffer = audio_encoder.encode(std::slice::from_raw_parts(
                    buffer,
                    num_frames_available as usize,
                ))?;

                tracing::info!(len=?encoded_buffer.len(),"encoded buffer");

                HRESULT!(capture_client.ReleaseBuffer(num_frames_available));

                packet_length = HRESULT!(capture_client.GetNextPacketSize());
            }
        }

        HRESULT!(audio_client.Stop());
        Ok(())
    }
}

unsafe fn render() -> CoreResult<()> {
    HRESULT!(CoInitialize(None));

    let p_enumerator: IMMDeviceEnumerator = HRESULT!(CoCreateInstance(
        &MMDeviceEnumerator as *const _,
        None,
        CLSCTX_ALL
    ));

    let p_device = HRESULT!(p_enumerator.GetDefaultAudioEndpoint(eRender, eConsole));

    let p_audio_client: IAudioClient = HRESULT!(p_device.Activate(CLSCTX_ALL, None));

    let mix_format = HRESULT!(p_audio_client.GetMixFormat());
    defer! {
        CoTaskMemFree(Some(mix_format as *const _ as *const c_void));
    }

    HRESULT!(p_audio_client.Initialize(AUDCLNT_SHAREMODE_SHARED, 0, 10000000, 0, mix_format, None,));

    // todo: set format

    let buffer_frame_count = HRESULT!(p_audio_client.GetBufferSize());

    let p_render_client: IAudioRenderClient = HRESULT!(p_audio_client.GetService());

    let p_data = HRESULT!(p_render_client.GetBuffer(buffer_frame_count));

    // todo: source load data

    HRESULT!(p_render_client.ReleaseBuffer(buffer_frame_count, AUDCLNT_BUFFERFLAGS_SILENT.0 as u32));

    HRESULT!(p_audio_client.Start());

    loop {
        // todo: sleep

        let num_frames_padding = HRESULT!(p_audio_client.GetCurrentPadding());
        let num_frames_available = buffer_frame_count - num_frames_padding;

        let p_data = HRESULT!(p_render_client.GetBuffer(num_frames_available));

        // todo: load data

        p_render_client.ReleaseBuffer(num_frames_available, AUDCLNT_BUFFERFLAGS_SILENT.0 as u32);
    }

    Ok(())
}

#[test]
fn test_audio_capture() {
    tracing_subscriber::fmt::init();

    unsafe { capture_output_audio(None).unwrap() }
}
