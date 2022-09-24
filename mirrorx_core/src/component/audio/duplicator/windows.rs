use crate::{
    component::{
        audio::{REFTIMES_PER_MILLISEC, REFTIMES_PER_SEC, SLIENT_F32_BUFFER},
        frame::AudioEncodeFrame,
    },
    error::{CoreError, CoreResult},
    HRESULT,
};
use scopeguard::defer;
use std::{ffi::c_void, mem::ManuallyDrop, time::Duration};
use windows::{
    core::PCWSTR,
    Win32::{
        Media::Audio::{
            eConsole, eRender, IAudioCaptureClient, IAudioClient, IMMDevice, IMMDeviceEnumerator,
            MMDeviceEnumerator, AUDCLNT_BUFFERFLAGS_SILENT, AUDCLNT_SHAREMODE_SHARED,
            AUDCLNT_STREAMFLAGS_LOOPBACK,
        },
        System::Com::{CoCreateInstance, CoInitialize, CoTaskMemFree, CoUninitialize, CLSCTX_ALL},
    },
};

pub struct Duplicator {
    audio_client: IAudioClient,
    capture_client: IAudioCaptureClient,
    sample_rate: u32,
    channels: u8,
    hns_actual_duration: u64,
    bits_per_sample: u16,
}

impl Duplicator {
    pub fn new(device_id: Option<String>) -> CoreResult<Self> {
        unsafe {
            HRESULT!(CoInitialize(None));

            let device_enumerator: IMMDeviceEnumerator = HRESULT!(CoCreateInstance(
                &MMDeviceEnumerator as *const _,
                None,
                CLSCTX_ALL
            ));

            let audio_device = {
                let specified_device = device_id.and_then(|device_id| {
                    let device_id_hstring =
                        ManuallyDrop::new(windows::core::HSTRING::from(&device_id));
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

            let format = HRESULT!(audio_client.GetMixFormat());
            defer!(CoTaskMemFree(Some(format as *const _ as *const c_void)));

            let format_tag_addr = std::ptr::addr_of!((*format).wFormatTag);
            let format_tag = std::ptr::read_unaligned(format_tag_addr);

            let channels_addr = std::ptr::addr_of!((*format).nChannels);
            let channels = std::ptr::read_unaligned(channels_addr);

            let samples_per_sec_addr = std::ptr::addr_of!((*format).nSamplesPerSec);
            let samples_per_sec = std::ptr::read_unaligned(samples_per_sec_addr);

            let avg_bytes_addr = std::ptr::addr_of!((*format).nAvgBytesPerSec);
            let avg_bytes = std::ptr::read_unaligned(avg_bytes_addr);

            let block_align_addr = std::ptr::addr_of!((*format).nBlockAlign);
            let block_align = std::ptr::read_unaligned(block_align_addr);

            let bits_per_sample_addr = std::ptr::addr_of!((*format).wBitsPerSample);
            let bits_per_sample = std::ptr::read_unaligned(bits_per_sample_addr);

            tracing::info!("audio format tag: {}", format_tag);
            tracing::info!("audio channels: {}", channels);
            tracing::info!("audio sample_per_sec: {}", samples_per_sec); // sample rate
            tracing::info!("audio avg_bytes: {}", avg_bytes);
            tracing::info!("audio block_align: {}", block_align);
            tracing::info!("audio bits_per_sample: {}", bits_per_sample); // 16bit or 32bit

            HRESULT!(audio_client.Initialize(
                AUDCLNT_SHAREMODE_SHARED,
                AUDCLNT_STREAMFLAGS_LOOPBACK,
                REFTIMES_PER_SEC as i64,
                0,
                format,
                None,
            ));

            let buffer_frame_count = HRESULT!(audio_client.GetBufferSize());

            let capture_client: IAudioCaptureClient = HRESULT!(audio_client.GetService());

            let hns_actual_duration =
                REFTIMES_PER_SEC * (buffer_frame_count as u64) / (samples_per_sec as u64);

            Ok(Duplicator {
                audio_client,
                capture_client,
                sample_rate: samples_per_sec,
                channels: channels as u8,
                hns_actual_duration,
                bits_per_sample,
            })
        }
    }

    pub fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    pub fn channels(&self) -> u8 {
        self.channels
    }

    pub fn should_sleep_duration(&self) -> Duration {
        Duration::from_millis(self.hns_actual_duration / REFTIMES_PER_MILLISEC / 2)
    }

    pub fn start(&self) -> CoreResult<()> {
        unsafe {
            HRESULT!(self.audio_client.Start());
            Ok(())
        }
    }

    pub fn stop(&self) -> CoreResult<()> {
        unsafe {
            HRESULT!(self.audio_client.Stop());
            Ok(())
        }
    }

    pub fn capture(&self) -> CoreResult<Vec<AudioEncodeFrame>> {
        unsafe {
            let mut frames = Vec::new();
            let mut packet_length = HRESULT!(self.capture_client.GetNextPacketSize());
            while packet_length != 0 {
                let mut p_data = std::ptr::null_mut();
                let mut num_frames_available = 0;
                let mut flags = 0;

                HRESULT!(self.capture_client.GetBuffer(
                    &mut p_data,
                    &mut num_frames_available,
                    &mut flags,
                    None,
                    None,
                ));

                let buffer = if flags & (AUDCLNT_BUFFERFLAGS_SILENT.0 as u32) != 0 {
                    // slient stream
                    &SLIENT_F32_BUFFER[0..num_frames_available as usize]
                } else {
                    let float_slice: *mut f32 = std::mem::transmute(p_data);
                    std::slice::from_raw_parts(float_slice, num_frames_available as usize)
                };

                frames.push(AudioEncodeFrame {
                    format_f32: self.bits_per_sample == 32,
                    bytes: buffer.to_vec(),
                });

                HRESULT!(self.capture_client.ReleaseBuffer(num_frames_available));

                packet_length = HRESULT!(self.capture_client.GetNextPacketSize());
            }

            Ok(frames)
        }
    }
}

impl Drop for Duplicator {
    fn drop(&mut self) {
        unsafe {
            CoUninitialize();
        }
    }
}
