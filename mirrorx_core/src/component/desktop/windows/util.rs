use crate::{core_error, error::CoreResult, HRESULT};
use scopeguard::defer;
use windows::Win32::{
    Graphics::{Direct3D::*, Direct3D11::*},
    System::{
        StationsAndDesktops::{
            CloseDesktop, OpenInputDesktop, SetThreadDesktop, DESKTOP_CONTROL_FLAGS,
        },
        SystemServices::GENERIC_ALL,
    },
};

pub unsafe fn prepare_desktop() -> CoreResult<()> {
    let current_desktop = HRESULT!(OpenInputDesktop(
        DESKTOP_CONTROL_FLAGS::default(),
        false,
        GENERIC_ALL
    ));

    defer! {
        let _ = CloseDesktop(current_desktop);
    }

    if !SetThreadDesktop(current_desktop).as_bool() {
        return Err(core_error!("SetThreadDesktop set current desktop failed"));
    }

    Ok(())
}

pub unsafe fn init_directx() -> CoreResult<(ID3D11Device, ID3D11DeviceContext)> {
    let driver_types = [
        D3D_DRIVER_TYPE_HARDWARE,
        D3D_DRIVER_TYPE_WARP,
        D3D_DRIVER_TYPE_REFERENCE,
        D3D_DRIVER_TYPE_SOFTWARE,
    ];

    let mut device = None;
    let mut device_context = None;
    let mut feature_level = std::mem::zeroed();

    for driver_type in driver_types {
        match D3D11CreateDevice(
            None,
            driver_type,
            None,
            D3D11_CREATE_DEVICE_BGRA_SUPPORT | D3D11_CREATE_DEVICE_DEBUG,
            None,
            D3D11_SDK_VERSION,
            Some(&mut device),
            Some(&mut feature_level),
            Some(&mut device_context),
        ) {
            Ok(_) => {
                let driver_type_name = match driver_type {
                    D3D_DRIVER_TYPE_UNKNOWN => "D3D_DRIVER_TYPE_UNKNOWN",
                    D3D_DRIVER_TYPE_HARDWARE => "D3D_DRIVER_TYPE_HARDWARE",
                    D3D_DRIVER_TYPE_REFERENCE => "D3D_DRIVER_TYPE_REFERENCE",
                    D3D_DRIVER_TYPE_NULL => "D3D_DRIVER_TYPE_NULL",
                    D3D_DRIVER_TYPE_SOFTWARE => "D3D_DRIVER_TYPE_SOFTWARE",
                    D3D_DRIVER_TYPE_WARP => "D3D_DRIVER_TYPE_WARP",
                    _ => "Unknown",
                };

                let feature_level_name = match feature_level {
                    D3D_FEATURE_LEVEL_12_2 => "D3D_FEATURE_LEVEL_12_2",
                    D3D_FEATURE_LEVEL_12_1 => "D3D_FEATURE_LEVEL_12_1",
                    D3D_FEATURE_LEVEL_12_0 => "D3D_FEATURE_LEVEL_12_0",
                    D3D_FEATURE_LEVEL_11_1 => "D3D_FEATURE_LEVEL_11_1",
                    D3D_FEATURE_LEVEL_11_0 => "D3D_FEATURE_LEVEL_11_0",
                    D3D_FEATURE_LEVEL_10_1 => "D3D_FEATURE_LEVEL_10_1",
                    D3D_FEATURE_LEVEL_10_0 => "D3D_FEATURE_LEVEL_10_0",
                    D3D_FEATURE_LEVEL_9_3 => "D3D_FEATURE_LEVEL_9_3",
                    D3D_FEATURE_LEVEL_9_2 => "D3D_FEATURE_LEVEL_9_2",
                    D3D_FEATURE_LEVEL_9_1 => "D3D_FEATURE_LEVEL_9_1",
                    D3D_FEATURE_LEVEL_1_0_CORE => "D3D_FEATURE_LEVEL_1_0_CORE",
                    _ => "Unknown",
                };

                tracing::info!(
                    ?driver_type_name,
                    ?feature_level_name,
                    "create DirectX device successfully"
                );

                break;
            }
            Err(err) => {
                let driver_type_name = match driver_type {
                    D3D_DRIVER_TYPE_UNKNOWN => "D3D_DRIVER_TYPE_UNKNOWN",
                    D3D_DRIVER_TYPE_HARDWARE => "D3D_DRIVER_TYPE_HARDWARE",
                    D3D_DRIVER_TYPE_REFERENCE => "D3D_DRIVER_TYPE_REFERENCE",
                    D3D_DRIVER_TYPE_NULL => "D3D_DRIVER_TYPE_NULL",
                    D3D_DRIVER_TYPE_SOFTWARE => "D3D_DRIVER_TYPE_SOFTWARE",
                    D3D_DRIVER_TYPE_WARP => "D3D_DRIVER_TYPE_WARP",
                    _ => "Unknown",
                };

                tracing::info!(
                    ?driver_type_name,
                    ?err,
                    "create DirectX device failed, try next one"
                );
            }
        };
    }

    if let (Some(device), Some(device_context)) = (device, device_context) {
        Ok((device, device_context))
    } else {
        Err(core_error!(
            "create DirectX device failed, all driver types had tried"
        ))
    }
}
