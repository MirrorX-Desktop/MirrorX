use crate::error::{CoreError, CoreResult};
use scopeguard::defer;
use std::ffi::c_void;

pub mod decoder;
pub mod duplicator;
pub mod encoder;
pub mod render;

const REFTIMES_PER_SEC: u64 = 10000000;
const REFTIMES_PER_MILLISEC: u64 = 10000;
const SLIENT_F32_BUFFER: &[f32] = &[0f32; 4096];

#[cfg(target_os = "windows")]
pub fn enum_audio_device(input: bool) -> CoreResult<Vec<(String, String)>> {
    use crate::HRESULT;
    use windows::Win32::{
        Devices::FunctionDiscovery::PKEY_Device_FriendlyName,
        Media::Audio::{
            eCapture, eRender, IMMDeviceEnumerator, MMDeviceEnumerator, DEVICE_STATE_ACTIVE,
        },
        System::Com::{
            CoCreateInstance, CoInitialize, CoTaskMemFree, CoUninitialize,
            StructuredStorage::PropVariantClear, CLSCTX_ALL, STGM_READ,
        },
    };

    unsafe {
        HRESULT!(CoInitialize(None));
        defer! {
            let _ = CoUninitialize();
        }

        let device_enumerator: IMMDeviceEnumerator = HRESULT!(CoCreateInstance(
            &MMDeviceEnumerator as *const _,
            None,
            CLSCTX_ALL
        ));

        let device_collection = HRESULT!(device_enumerator
            .EnumAudioEndpoints(if input { eCapture } else { eRender }, DEVICE_STATE_ACTIVE));
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
