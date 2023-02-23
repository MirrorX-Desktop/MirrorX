use super::Display;
use crate::{core_error, error::CoreResult, HRESULT};
use windows::{
    core::PCWSTR,
    Win32::Graphics::{
        Dxgi::{CreateDXGIFactory1, IDXGIAdapter1, IDXGIFactory1, IDXGIOutput},
        Gdi::{EnumDisplaySettingsW, DEVMODEW, ENUM_CURRENT_SETTINGS},
    },
};

pub fn enum_all_available_displays() -> CoreResult<Vec<Display>> {
    unsafe {
        let mut displays = Vec::new();
        let factory: IDXGIFactory1 = HRESULT!(CreateDXGIFactory1());

        for adapter_index in 0..u32::MAX {
            if let Ok(adapter) = factory.EnumAdapters1(adapter_index) {
                for output_index in 0..u32::MAX {
                    if let Ok(output) = adapter.EnumOutputs(output_index) {
                        let mut desc = std::mem::zeroed();
                        if output.GetDesc(&mut desc).is_err() {
                            continue;
                        }

                        if !desc.AttachedToDesktop.as_bool() {
                            continue;
                        }

                        let device_name = PCWSTR::from_raw(desc.DeviceName.as_ptr());
                        let mut display_settings: DEVMODEW = std::mem::zeroed();
                        display_settings.dmSize = std::mem::size_of::<DEVMODEW>() as u16;

                        if !EnumDisplaySettingsW(
                            device_name,
                            ENUM_CURRENT_SETTINGS,
                            &mut display_settings as *mut _,
                        )
                        .as_bool()
                        {
                            continue;
                        }

                        let Ok(id) = device_name.to_string() else {
                            continue;
                        };

                        displays.push(Display {
                            id,
                            left: desc.DesktopCoordinates.left,
                            top: desc.DesktopCoordinates.top,
                            width: display_settings.dmPelsWidth,
                            height: display_settings.dmPelsHeight,
                            refresh_rate: display_settings.dmDisplayFrequency as _,
                        })
                    } else {
                        break;
                    }
                }
            } else {
                break;
            }
        }

        Ok(displays)
    }
}

pub fn query_display(display_id: &str) -> CoreResult<(Display, IDXGIAdapter1, IDXGIOutput)> {
    unsafe {
        let factory: IDXGIFactory1 = HRESULT!(CreateDXGIFactory1());

        for adapter_index in 0..u32::MAX {
            if let Ok(adapter) = factory.EnumAdapters1(adapter_index) {
                for output_index in 0..u32::MAX {
                    if let Ok(output) = adapter.EnumOutputs(output_index) {
                        let mut desc = std::mem::zeroed();
                        if output.GetDesc(&mut desc).is_err() {
                            continue;
                        }

                        if !desc.AttachedToDesktop.as_bool() {
                            continue;
                        }

                        let device_name = PCWSTR::from_raw(desc.DeviceName.as_ptr());
                        let mut display_settings: DEVMODEW = std::mem::zeroed();
                        display_settings.dmSize = std::mem::size_of::<DEVMODEW>() as u16;

                        if !EnumDisplaySettingsW(
                            device_name,
                            ENUM_CURRENT_SETTINGS,
                            &mut display_settings as *mut _,
                        )
                        .as_bool()
                        {
                            continue;
                        }

                        let Ok(id) = device_name.to_string() else {
                            continue;
                        };

                        if id == display_id {
                            return Ok((
                                Display {
                                    id,
                                    left: desc.DesktopCoordinates.left,
                                    top: desc.DesktopCoordinates.top,
                                    width: display_settings.dmPelsWidth,
                                    height: display_settings.dmPelsHeight,
                                    refresh_rate: display_settings.dmDisplayFrequency as _,
                                },
                                adapter,
                                output,
                            ));
                        }
                    } else {
                        break;
                    }
                }
            } else {
                break;
            }
        }

        Err(core_error!("query specified display failed"))
    }
}
