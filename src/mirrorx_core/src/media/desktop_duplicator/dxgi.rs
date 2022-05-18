use anyhow::bail;
use log::info;
use rustc_hash::FxHashMap;
use std::{collections::HashMap, os::windows, ptr::null_mut};
use winapi::{
    shared::{
        dxgi::*,
        dxgi1_2::{IDXGIOutput1, IDXGIOutputDuplication},
        minwindef::TRUE,
    },
    um::{
        d3d11::{D3D11CreateDevice, D3D11_CREATE_DEVICE_BGRA_SUPPORT, D3D11_SDK_VERSION},
        d3dcommon::D3D_DRIVER_TYPE_HARDWARE,
        unknwnbase::IUnknown,
        winuser::GetMonitorInfoW,
    },
    um::{
        d3d11::{ID3D11Device, ID3D11DeviceContext},
        winuser::MONITORINFO,
    },
    Interface,
};
use wio::com::ComPtr;

struct DuplicatonManager {
    d3d_device: ComPtr<ID3D11Device>,
    d3d_device_context: ComPtr<ID3D11DeviceContext>,
    adapter: ComPtr<IDXGIAdapter>,
    output: ComPtr<IDXGIOutput>,
    output_duplication: ComPtr<IDXGIOutputDuplication>,
}

impl DuplicatonManager {
    fn new() -> anyhow::Result<Self> {
        let (d3d_device, d3d_device_context, adapter, output, output_duplication) = create_dxgi()?;
        Ok(DuplicatonManager {
            d3d_device,
            d3d_device_context,
            adapter,
            output,
            output_duplication,
        })
    }
}

fn enum_adapter_with_outputs(
) -> anyhow::Result<Vec<(ComPtr<IDXGIAdapter>, Vec<ComPtr<IDXGIOutput>>)>> {
    unsafe {
        let mut dxgi_factory1_raw_ptr = std::ptr::null_mut();
        let mut hr = CreateDXGIFactory1(&IID_IDXGIFactory1, &mut dxgi_factory1_raw_ptr);
        if hr < 0 {
            bail!(
                "enum_adapter_with_outputs: CreateDXGIFactory1 returns {:X}",
                hr
            );
        }

        let dxgi_factory1_com_ptr = ComPtr::from_raw(dxgi_factory1_raw_ptr as *mut IDXGIFactory1);

        let mut new_adapter_with_outputs = Vec::new();
        for dxgi_adapter_enum_index in 0.. {
            let mut dxgi_adapter_raw_ptr = std::ptr::null_mut();
            hr = dxgi_factory1_com_ptr
                .EnumAdapters(dxgi_adapter_enum_index, &mut dxgi_adapter_raw_ptr);
            if hr < 0 {
                break;
            }

            let dxgi_adapter_com_ptr = ComPtr::from_raw(dxgi_adapter_raw_ptr);

            let mut dxgi_output_com_ptrs = Vec::new();
            for dxgi_output_enum_index in 0.. {
                let mut dxgi_output_raw_ptr = std::ptr::null_mut();
                hr = dxgi_adapter_com_ptr
                    .EnumOutputs(dxgi_output_enum_index, &mut dxgi_output_raw_ptr);
                if hr < 0 {
                    break;
                }

                let dxgi_output_com_ptr = ComPtr::from_raw(dxgi_output_raw_ptr);

                let mut dxgi_output_desc = std::mem::zeroed();
                hr = dxgi_output_com_ptr.GetDesc(&mut dxgi_output_desc);
                if hr < 0 {
                    break;
                }

                if dxgi_output_desc.AttachedToDesktop != 0 {
                    dxgi_output_com_ptrs.push(dxgi_output_com_ptr);
                }
            }

            new_adapter_with_outputs.push((dxgi_adapter_com_ptr, dxgi_output_com_ptrs));
        }

        Ok(new_adapter_with_outputs)
    }
}

fn is_primary_output(output: &ComPtr<IDXGIOutput>) -> bool {
    unsafe {
        let mut dxgi_output_desc = std::mem::zeroed();
        let hr = output.GetDesc(&mut dxgi_output_desc);
        if hr < 0 {
            return false;
        }

        let mut monitor_info: MONITORINFO = std::mem::zeroed();
        monitor_info.cbSize = std::mem::size_of::<MONITORINFO>() as u32;
        let success = GetMonitorInfoW(dxgi_output_desc.Monitor, &mut monitor_info);

        return success == TRUE && ((monitor_info.dwFlags & 1) != 0);
    }
}

fn create_dxgi() -> anyhow::Result<(
    ComPtr<ID3D11Device>,
    ComPtr<ID3D11DeviceContext>,
    ComPtr<IDXGIAdapter>,
    ComPtr<IDXGIOutput>,
    ComPtr<IDXGIOutputDuplication>,
)> {
    unsafe {
        let adapter_with_outputs = enum_adapter_with_outputs()?;
        let (primary_adapter_com_ptr, primary_output_com_ptr) =
            || -> Option<(ComPtr<IDXGIAdapter>, ComPtr<IDXGIOutput>)> {
                for (adapter, outputs) in adapter_with_outputs {
                    for output in outputs {
                        if is_primary_output(&output) {
                            return Some((adapter, output));
                        }
                    }
                }
                return None;
            }()
            .ok_or(anyhow::anyhow!(
                "create_d3d11_device_and_context: no primary output"
            ))?;

        let mut d3d11_device_raw_ptr = std::ptr::null_mut();
        let mut d3d11_device_context_raw_ptr = std::ptr::null_mut();
        let d3d_feature_level = std::ptr::null_mut();

        let mut hr = D3D11CreateDevice(
            primary_adapter_com_ptr.as_raw(),
            D3D_DRIVER_TYPE_HARDWARE,
            std::ptr::null_mut(),
            D3D11_CREATE_DEVICE_BGRA_SUPPORT,
            std::ptr::null(),
            0,
            D3D11_SDK_VERSION,
            &mut d3d11_device_raw_ptr,
            d3d_feature_level,
            &mut d3d11_device_context_raw_ptr,
        );

        if hr < 0 {
            bail!("create_dxgi: D3D11CreateDevice returns {:X}", hr);
        }

        info!(
            "create_dxgi: create d3d11 device, feature_level: {}",
            *d3d_feature_level
        );

        let dxgi_output1_com_ptr = primary_output_com_ptr
            .cast::<IDXGIOutput1>()
            .or_else(|hr| {
                bail!(
                    "create_dxgi: cast IDXGIOutput to IDXGIOutput1 returns {:X}",
                    hr
                )
            })?;

        let mut dxgi_output_duplication_raw_ptr = std::ptr::null_mut();
        hr = dxgi_output1_com_ptr.DuplicateOutput(
            d3d11_device_raw_ptr as *mut IUnknown,
            &mut dxgi_output_duplication_raw_ptr,
        );

        if hr < 0 {
            bail!(
                "create_dxgi: IDXGIOutput1::DuplicateOutput returns {:X}",
                hr
            );
        }

        Ok((
            ComPtr::from_raw(d3d11_device_raw_ptr),
            ComPtr::from_raw(d3d11_device_context_raw_ptr),
            primary_adapter_com_ptr,
            primary_output_com_ptr,
            ComPtr::from_raw(dxgi_output_duplication_raw_ptr),
        ))
    }
}
