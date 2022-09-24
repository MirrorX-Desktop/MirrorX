// use std::{ffi::c_void, mem::ManuallyDrop, time::Duration};

// use crate::{
//     component::audio_encoder::audio_encoder::AudioEncoder,
//     error::{CoreError, CoreResult},
//     HRESULT,
// };
// use scopeguard::defer;
// use windows::{
//     core::PCWSTR,
//     Win32::{
//         Devices::FunctionDiscovery::PKEY_Device_FriendlyName,
//         Media::Audio::{
//             eCapture, eConsole, eRender, EDataFlow, IAudioCaptureClient, IAudioClient,
//             IAudioRenderClient, IMMDevice, IMMDeviceEnumerator, MMDeviceEnumerator,
//             AUDCLNT_BUFFERFLAGS_SILENT, AUDCLNT_SHAREMODE_SHARED, AUDCLNT_STREAMFLAGS_LOOPBACK,
//             DEVICE_STATE_ACTIVE, WAVEFORMATEX, WAVEFORMATEXTENSIBLE, WAVEFORMATEXTENSIBLE_0,
//         },
//         System::Com::{
//             CoCreateInstance, CoInitialize, CoTaskMemFree, StructuredStorage::PropVariantClear,
//             CLSCTX_ALL, STGM_READ,
//         },
//     },
// };

// unsafe fn render() -> CoreResult<()> {
//     HRESULT!(CoInitialize(None));

//     let p_enumerator: IMMDeviceEnumerator = HRESULT!(CoCreateInstance(
//         &MMDeviceEnumerator as *const _,
//         None,
//         CLSCTX_ALL
//     ));

//     let p_device = HRESULT!(p_enumerator.GetDefaultAudioEndpoint(eRender, eConsole));

//     let p_audio_client: IAudioClient = HRESULT!(p_device.Activate(CLSCTX_ALL, None));

//     let mix_format = HRESULT!(p_audio_client.GetMixFormat());
//     defer! {
//         CoTaskMemFree(Some(mix_format as *const _ as *const c_void));
//     }

//     HRESULT!(p_audio_client.Initialize(AUDCLNT_SHAREMODE_SHARED, 0, 10000000, 0, mix_format, None,));

//     // todo: set format

//     let buffer_frame_count = HRESULT!(p_audio_client.GetBufferSize());

//     let p_render_client: IAudioRenderClient = HRESULT!(p_audio_client.GetService());

//     let p_data = HRESULT!(p_render_client.GetBuffer(buffer_frame_count));

//     // todo: source load data

//     HRESULT!(p_render_client.ReleaseBuffer(buffer_frame_count, AUDCLNT_BUFFERFLAGS_SILENT.0 as u32));

//     HRESULT!(p_audio_client.Start());

//     loop {
//         // todo: sleep

//         let num_frames_padding = HRESULT!(p_audio_client.GetCurrentPadding());
//         let num_frames_available = buffer_frame_count - num_frames_padding;

//         let p_data = HRESULT!(p_render_client.GetBuffer(num_frames_available));

//         // todo: load data

//         p_render_client.ReleaseBuffer(num_frames_available, AUDCLNT_BUFFERFLAGS_SILENT.0 as u32);
//     }

//     Ok(())
// }
