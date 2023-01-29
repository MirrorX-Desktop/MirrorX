use crate::{component::desktop::monitor::Monitor, core_error, error::CoreResult, HRESULT};
use image::ColorType;
use scopeguard::defer;
use std::{collections::HashMap, io::Cursor, os::raw::c_void};
use windows::{
    core::{Interface, PCWSTR},
    Win32::{
        Devices::Display::*,
        Foundation::ERROR_SUCCESS,
        Graphics::{Dxgi::*, Gdi::*},
        UI::WindowsAndMessaging::{EDD_GET_DEVICE_INTERFACE_NAME, MONITORINFOF_PRIMARY},
    },
};

pub fn get_primary_monitor_params() -> CoreResult<Monitor> {
    let monitors = get_active_monitors(false)?;
    for monitor in monitors.into_iter() {
        if monitor.is_primary {
            return Ok(monitor);
        }
    }

    Err(core_error!("no primary display"))
}

pub fn get_active_monitors(take_screen_shot: bool) -> CoreResult<Vec<Monitor>> {
    unsafe {
        let all_monitors = enum_all_monitors_path_and_name()?;
        let dxgi_output_monitors = enum_dxgi_outputs(all_monitors, take_screen_shot)?;

        Ok(dxgi_output_monitors)
    }
}

unsafe fn enum_dxgi_outputs(
    all_monitors: HashMap<String, String>,
    need_screen_shot: bool,
) -> CoreResult<Vec<Monitor>> {
    let (device, _) = crate::component::desktop::windows::util::init_directx()?;

    let dxgi_device: IDXGIDevice = HRESULT!(device.cast());

    let dxgi_adapter: IDXGIAdapter = HRESULT!(dxgi_device.GetParent());

    let adapter_desc = HRESULT!(dxgi_adapter.GetDesc());

    tracing::info!(
        adapter_name = ?PCWSTR::from_raw(adapter_desc.Description.as_ptr()).to_string()?,
        "DXGI OUTPUTS ADAPTER"
    );

    let mut displays = Vec::new();
    let mut output_index = 0u32;

    while let Ok(dxgi_output) = dxgi_adapter.EnumOutputs(output_index) {
        output_index += 1;

        let output_desc = HRESULT!(dxgi_output.GetDesc());

        if !output_desc.AttachedToDesktop.as_bool() {
            continue;
        }

        let mut monitor_info: MONITORINFO = std::mem::zeroed();
        monitor_info.cbSize = std::mem::size_of::<MONITORINFO>() as u32;

        let mut monitor_is_primary = false;
        // let mut screen_width = 0;
        // let mut screen_height = 0;
        if GetMonitorInfoW(output_desc.Monitor, &mut monitor_info as *mut _).as_bool() {
            monitor_is_primary = (monitor_info.dwFlags & MONITORINFOF_PRIMARY) != 0;
            // screen_width = monitor_info.rcMonitor.right - monitor_info.rcMonitor.left;
            // screen_height = monitor_info.rcMonitor.bottom - monitor_info.rcMonitor.top;
        }

        let mut dev_index = 0u32;
        loop {
            let origin_device_name = PCWSTR::from_raw(output_desc.DeviceName.as_ptr());
            tracing::info!("origin device name {}", origin_device_name.to_string()?);

            let mut display_device: DISPLAY_DEVICEW = std::mem::zeroed();
            display_device.cb = std::mem::size_of::<DISPLAY_DEVICEW>() as u32;

            let success = EnumDisplayDevicesW(
                origin_device_name,
                dev_index,
                &mut display_device as *mut _,
                EDD_GET_DEVICE_INTERFACE_NAME,
            )
            .as_bool();

            dev_index += 1;

            if !success {
                break;
            }

            let mut display_mode: DEVMODEW = std::mem::zeroed();
            display_mode.dmSize = std::mem::size_of::<DEVMODEW>() as u16;

            let mut refresh_rate = 0;

            // monitor resolution is resolution in pixels without scaling, and it equals 'Desktop Resolution'
            // at 'Windows Settings' - 'Screen'
            let mut monitor_resolution_width = 0;
            let mut monitor_resolution_height = 0;

            if EnumDisplaySettingsW(
                origin_device_name,
                ENUM_CURRENT_SETTINGS,
                &mut display_mode as *mut _,
            )
            .as_bool()
            {
                refresh_rate = display_mode.dmDisplayFrequency;
                monitor_resolution_width = display_mode.dmPelsWidth;
                monitor_resolution_height = display_mode.dmPelsHeight;
            }

            if (display_device.StateFlags & DISPLAY_DEVICE_ATTACHED_TO_DESKTOP) != 0 {
                let screent_shot_buffer = if need_screen_shot {
                    Some(take_screen_shot(
                        &origin_device_name,
                        monitor_info.rcMonitor.left,
                        monitor_info.rcMonitor.top,
                        monitor_resolution_width,
                        monitor_resolution_height,
                    )?)
                } else {
                    None
                };

                let device_id = PCWSTR::from_raw(display_device.DeviceID.as_ptr()).to_string()?;

                let name = all_monitors
                    .get(&device_id)
                    .map_or(String::default(), |name| name.clone());

                displays.push(Monitor {
                    id: device_id,
                    name,
                    refresh_rate: (refresh_rate.min(u8::MAX as u32) as u8),
                    width: (monitor_info.rcMonitor.right - monitor_info.rcMonitor.left) as u16,
                    height: (monitor_info.rcMonitor.bottom - monitor_info.rcMonitor.top) as u16,
                    is_primary: monitor_is_primary,
                    screen_shot: screent_shot_buffer,
                    left: monitor_info.rcMonitor.left as u16,
                    top: monitor_info.rcMonitor.top as u16,
                });
            }
        }
    }

    Ok(displays)
}

unsafe fn enum_all_monitors_path_and_name() -> CoreResult<HashMap<String, String>> {
    let mut path_count: u32 = 0;
    let mut mode_count: u32 = 0;
    let mut err_code = GetDisplayConfigBufferSizes(
        QDC_ALL_PATHS,
        &mut path_count as *mut _,
        &mut mode_count as *mut _,
    );

    if err_code != (ERROR_SUCCESS.0 as i32) {
        return Err(core_error!(
            "GetDisplayConfigBufferSizes returns error code: {}",
            err_code
        ));
    }

    let mut display_paths = Vec::<DISPLAYCONFIG_PATH_INFO>::with_capacity(path_count as usize);
    let mut display_modes = Vec::<DISPLAYCONFIG_MODE_INFO>::with_capacity(mode_count as usize);

    err_code = QueryDisplayConfig(
        QDC_ALL_PATHS,
        &mut path_count as *mut _,
        display_paths.as_mut_ptr() as *mut _,
        &mut mode_count as *mut _,
        display_modes.as_mut_ptr() as *mut _,
        std::ptr::null_mut(),
    );

    if err_code != (ERROR_SUCCESS.0 as i32) {
        return Err(core_error!(
            "QueryDisplayConfig returns error code: {}",
            err_code
        ));
    }

    display_paths.set_len(path_count as usize);
    display_modes.set_len(mode_count as usize);

    let mut all_monitors_path_and_name = HashMap::new();

    for mode_info in display_modes {
        if mode_info.infoType == DISPLAYCONFIG_MODE_INFO_TYPE_TARGET {
            let mut device_name: DISPLAYCONFIG_TARGET_DEVICE_NAME = std::mem::zeroed();
            device_name.header = DISPLAYCONFIG_DEVICE_INFO_HEADER {
                size: std::mem::size_of::<DISPLAYCONFIG_TARGET_DEVICE_NAME>() as u32,
                adapterId: mode_info.adapterId,
                id: mode_info.id,
                r#type: DISPLAYCONFIG_DEVICE_INFO_GET_TARGET_NAME,
            };

            let device_name_header_ptr = std::mem::transmute::<
                *mut DISPLAYCONFIG_TARGET_DEVICE_NAME,
                *mut DISPLAYCONFIG_DEVICE_INFO_HEADER,
            >(&mut device_name as *mut _);

            let err_code = DisplayConfigGetDeviceInfo(device_name_header_ptr);
            if err_code != (ERROR_SUCCESS.0 as i32) {
                return Err(core_error!(
                    "DisplayConfigGetDeviceInfo returns error code: {}",
                    err_code
                ));
            }

            let device_path =
                PCWSTR::from_raw(device_name.monitorDevicePath.as_ptr()).to_string()?;

            let device_friendly_name =
                PCWSTR::from_raw(device_name.monitorFriendlyDeviceName.as_ptr()).to_string()?;

            all_monitors_path_and_name.insert(device_path, device_friendly_name);
        }
    }

    Ok(all_monitors_path_and_name)
}

unsafe fn take_screen_shot(
    device_name: &PCWSTR,
    monitor_coord_x: i32,
    monitor_coord_y: i32,
    monitor_resolution_width: u32,
    monitor_resolution_height: u32,
) -> CoreResult<Vec<u8>> {
    let src_dc = CreateDCW(None, *device_name, None, None);
    if src_dc.is_invalid() {
        return Err(core_error!("CreateDCW returns invalid dc"));
    }

    defer! {
        ReleaseDC(None, src_dc);
    }

    let dst_dc = CreateCompatibleDC(src_dc);
    if dst_dc.is_invalid() {
        return Err(core_error!("CreateCompatibleDC returns invalid dc"));
    }

    defer! {
        DeleteObject(HGDIOBJ(dst_dc.0));
    }

    let src_bitmap = CreateCompatibleBitmap(
        src_dc,
        monitor_resolution_width as i32,
        monitor_resolution_height as i32,
    );

    defer! {
        DeleteObject(src_bitmap);
    }

    SelectObject(dst_dc, src_bitmap);

    if !BitBlt(
        dst_dc,
        0,
        0,
        monitor_resolution_width as i32,
        monitor_resolution_height as i32,
        src_dc,
        monitor_coord_x,
        monitor_coord_y,
        ROP_CODE(SRCCOPY.0 | CAPTUREBLT.0),
    )
    .as_bool()
    {
        return Err(core_error!("BitBlt failed"));
    }

    let mut bitmap: BITMAP = std::mem::zeroed();

    if GetObjectW(
        src_bitmap,
        std::mem::size_of::<BITMAP>() as i32,
        Some(&mut bitmap as *mut _ as *mut c_void),
    ) == 0
    {
        return Err(core_error!("GetObjectW failed"));
    }

    let is_32_bit = bitmap.bmBitsPixel == 32;
    let mut bmp_bytes = Vec::<u8>::with_capacity((bitmap.bmWidthBytes * bitmap.bmHeight) as usize);

    if GetBitmapBits(
        src_bitmap,
        bitmap.bmWidthBytes * bitmap.bmHeight,
        bmp_bytes.as_mut_ptr() as *mut c_void,
    ) == 0
    {
        return Err(core_error!("GetBitmapBits failed"));
    }

    bmp_bytes.set_len((bitmap.bmWidthBytes * bitmap.bmHeight) as usize);

    // swap blue(at index 0) and red(at index 2) color byte to convert BGRA(or BGR) order to RGBA(or RGB) order
    // every chunk size is 4 for BGRA or 3 for BGR
    let chunk_size = if is_32_bit { 4 } else { 3 };

    for chunk in bmp_bytes.chunks_mut(chunk_size) {
        chunk.swap(0, 2)
    }

    let mut png_bytes: Vec<u8> = Vec::with_capacity(bmp_bytes.len());

    if let Err(err) = image::write_buffer_with_format(
        &mut Cursor::new(&mut png_bytes),
        &bmp_bytes,
        bitmap.bmWidth as u32,
        bitmap.bmHeight as u32,
        if is_32_bit {
            ColorType::Rgba8
        } else {
            ColorType::Rgb8
        },
        image::ImageOutputFormat::Png,
    ) {
        return Err(core_error!(
            "write desktop screenshot image buffer failed ({})",
            err
        ));
    }

    Ok(png_bytes)
}
