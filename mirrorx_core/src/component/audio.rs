use std::ffi::c_void;

use crate::{
    error::{CoreError, CoreResult},
    HRESULT,
};
use scopeguard::defer;
use windows::Win32::{
    Media::Audio::{
        eCapture, eConsole, eRender, EDataFlow, IAudioCaptureClient, IAudioClient,
        IAudioRenderClient, IMMDeviceEnumerator, MMDeviceEnumerator, AUDCLNT_BUFFERFLAGS_SILENT,
        AUDCLNT_SHAREMODE_SHARED, AUDCLNT_STREAMFLAGS_LOOPBACK,
    },
    System::Com::{CoCreateInstance, CoInitialize, CoTaskMemFree, CLSCTX_ALL},
};

unsafe fn capture() -> CoreResult<()> {
    HRESULT!(CoInitialize(std::ptr::null()));

    let p_enumerator: IMMDeviceEnumerator = HRESULT!(CoCreateInstance(
        &MMDeviceEnumerator as *const _,
        None,
        CLSCTX_ALL
    ));

    let p_device = HRESULT!(p_enumerator.GetDefaultAudioEndpoint(eRender, eConsole));

    let p_audio_client: IAudioClient = HRESULT!(p_device.Activate(CLSCTX_ALL, std::ptr::null()));

    let mix_format = HRESULT!(p_audio_client.GetMixFormat());
    defer! {
        CoTaskMemFree(mix_format as *const _ as *const c_void);
    }

    HRESULT!(p_audio_client.Initialize(
        AUDCLNT_SHAREMODE_SHARED,
        AUDCLNT_STREAMFLAGS_LOOPBACK,
        10000000,
        0,
        mix_format,
        std::ptr::null(),
    ));

    let p_capture_client: IAudioCaptureClient = HRESULT!(p_audio_client.GetService());

    HRESULT!(p_audio_client.Start());

    let mut packet_length = 0;
    loop {
        packet_length = HRESULT!(p_capture_client.GetNextPacketSize());
        while packet_length != 0 {
            let mut p_data = std::ptr::null_mut();
            let mut num_frames_available = 0;
            let mut flags = 0;

            HRESULT!(p_capture_client.GetBuffer(
                &mut p_data,
                &mut num_frames_available,
                &mut flags,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            ));

            if flags & (AUDCLNT_BUFFERFLAGS_SILENT.0 as u32) != 0 {
                // todo: tell stream it's slient
                p_data = std::ptr::null_mut();
            }

            // todo: copy buffer
            if !p_data.is_null() {
                tracing::info!(?num_frames_available, "available frames");
            }

            HRESULT!(p_capture_client.ReleaseBuffer(num_frames_available));

            packet_length = HRESULT!(p_capture_client.GetNextPacketSize());
        }
    }

    HRESULT!(p_audio_client.Stop());
    Ok(())
}

unsafe fn render() -> CoreResult<()> {
    HRESULT!(CoInitialize(std::ptr::null()));

    let p_enumerator: IMMDeviceEnumerator = HRESULT!(CoCreateInstance(
        &MMDeviceEnumerator as *const _,
        None,
        CLSCTX_ALL
    ));

    let p_device = HRESULT!(p_enumerator.GetDefaultAudioEndpoint(eRender, eConsole));

    let p_audio_client: IAudioClient = HRESULT!(p_device.Activate(CLSCTX_ALL, std::ptr::null()));

    let mix_format = HRESULT!(p_audio_client.GetMixFormat());
    defer! {
        CoTaskMemFree(mix_format as *const _ as *const c_void);
    }

    HRESULT!(p_audio_client.Initialize(
        AUDCLNT_SHAREMODE_SHARED,
        0,
        10000000,
        0,
        mix_format,
        std::ptr::null(),
    ));

    // todo: set format

    let buffer_frame_count = HRESULT!(p_audio_client.GetBufferSize());

    let p_render_client: IAudioRenderClient = HRESULT!(p_audio_client.GetService());

    let mut p_data = HRESULT!(p_render_client.GetBuffer(buffer_frame_count));

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
