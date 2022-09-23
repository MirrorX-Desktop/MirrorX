use std::ffi::c_void;

use crate::{
    error::{CoreError, CoreResult},
    HRESULT,
};
use scopeguard::defer;
use windows::Win32::{
    Devices::FunctionDiscovery::PKEY_Device_FriendlyName,
    Media::Audio::{
        eCapture, eConsole, eRender, EDataFlow, IAudioCaptureClient, IAudioClient,
        IAudioRenderClient, IMMDeviceEnumerator, MMDeviceEnumerator, AUDCLNT_BUFFERFLAGS_SILENT,
        AUDCLNT_SHAREMODE_SHARED, AUDCLNT_STREAMFLAGS_LOOPBACK, DEVICE_STATE_ACTIVE, WAVEFORMATEX,
        WAVEFORMATEXTENSIBLE, WAVEFORMATEXTENSIBLE_0,
    },
    System::Com::{
        CoCreateInstance, CoInitialize, CoTaskMemFree, StructuredStorage::PropVariantClear,
        CLSCTX_ALL, STGM_READ,
    },
};

unsafe fn capture() -> CoreResult<()> {
    HRESULT!(CoInitialize(None));

    let p_enumerator: IMMDeviceEnumerator = HRESULT!(CoCreateInstance(
        &MMDeviceEnumerator as *const _,
        None,
        CLSCTX_ALL
    ));

    let collection = HRESULT!(p_enumerator.EnumAudioEndpoints(eRender, DEVICE_STATE_ACTIVE));
    let device_count = HRESULT!(collection.GetCount());
    for i in 0..device_count {
        let device = HRESULT!(collection.Item(i));

        let device_id = HRESULT!(device.GetId());
        let device_id_str = device_id.to_string()?;

        let prop = HRESULT!(device.OpenPropertyStore(STGM_READ));
        let mut friendly_name = HRESULT!(prop.GetValue(&PKEY_Device_FriendlyName));
        let friendly_name_str = friendly_name
            .Anonymous
            .Anonymous
            .Anonymous
            .pwszVal
            .to_string()?;

        tracing::info!("device: id: {} name: {}", device_id_str, friendly_name_str);

        CoTaskMemFree(Some(device_id.as_ptr() as *const _ as *const c_void));
        HRESULT!(PropVariantClear(&mut friendly_name));
    }

    // let p_device = HRESULT!(p_enumerator.GetDefaultAudioEndpoint(eRender, eConsole));

    // let p_audio_client: IAudioClient = HRESULT!(p_device.Activate(CLSCTX_ALL, None));

    // let mix_format = HRESULT!(p_audio_client.GetMixFormat());
    // defer! {
    //     CoTaskMemFree(Some(mix_format as *const _ as *const c_void));
    // }

    // let mix_format_format_tag_addr = std::ptr::addr_of!((*mix_format).wFormatTag);
    // let mix_format_foramt_tag = std::ptr::read_unaligned(mix_format_format_tag_addr);

    // let mix_format_channels_addr = std::ptr::addr_of!((*mix_format).nChannels);
    // let mix_format_channels = std::ptr::read_unaligned(mix_format_channels_addr);

    // let mix_format_samples_per_sec_addr = std::ptr::addr_of!((*mix_format).nSamplesPerSec);
    // let mix_format_samples_per_sec = std::ptr::read_unaligned(mix_format_samples_per_sec_addr);

    // let mix_format_average_bytes_addr = std::ptr::addr_of!((*mix_format).nAvgBytesPerSec);
    // let mix_format_average_bytes = std::ptr::read_unaligned(mix_format_average_bytes_addr);

    // let mix_format_block_align_addr = std::ptr::addr_of!((*mix_format).nBlockAlign);
    // let mix_format_block_align = std::ptr::read_unaligned(mix_format_block_align_addr);

    // let mix_format_bits_per_sample_addr = std::ptr::addr_of!((*mix_format).wBitsPerSample);
    // let mix_format_bits_per_sample = std::ptr::read_unaligned(mix_format_bits_per_sample_addr);

    // tracing::info!("format: {}", mix_format_foramt_tag);
    // tracing::info!("channels: {}", mix_format_channels);
    // tracing::info!("sample_per_sec: {}", mix_format_samples_per_sec); // sample rate
    // tracing::info!("avg_bytes: {}", mix_format_average_bytes);
    // tracing::info!("block_align: {}", mix_format_block_align);
    // tracing::info!("bits_per_sample: {}", mix_format_bits_per_sample); // 16bit or 32bit

    // HRESULT!(p_audio_client.Initialize(
    //     AUDCLNT_SHAREMODE_SHARED,
    //     AUDCLNT_STREAMFLAGS_LOOPBACK,
    //     10000000,
    //     0,
    //     mix_format,
    //     None,
    // ));

    // let p_capture_client: IAudioCaptureClient = HRESULT!(p_audio_client.GetService());

    // HRESULT!(p_audio_client.Start());

    // let mut packet_length = 0;
    // loop {
    //     packet_length = HRESULT!(p_capture_client.GetNextPacketSize());
    //     while packet_length != 0 {
    //         let mut p_data = std::ptr::null_mut();
    //         let mut num_frames_available = 0;
    //         let mut flags = 0;

    //         HRESULT!(p_capture_client.GetBuffer(
    //             &mut p_data,
    //             &mut num_frames_available,
    //             &mut flags,
    //             None,
    //             None,
    //         ));

    //         if flags & (AUDCLNT_BUFFERFLAGS_SILENT.0 as u32) != 0 {
    //             // todo: tell stream it's slient
    //             p_data = std::ptr::null_mut();
    //         }

    //         // todo: copy buffer
    //         if !p_data.is_null() {
    //             tracing::info!(?num_frames_available, "available frames");
    //         }

    //         HRESULT!(p_capture_client.ReleaseBuffer(num_frames_available));

    //         packet_length = HRESULT!(p_capture_client.GetNextPacketSize());
    //     }
    // }

    // HRESULT!(p_audio_client.Stop());
    Ok(())
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

    unsafe { capture().unwrap() }
}
