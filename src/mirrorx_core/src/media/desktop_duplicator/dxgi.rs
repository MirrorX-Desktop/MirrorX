use anyhow::bail;
use log::info;
use rustc_hash::FxHashMap;
use std::{
    collections::HashMap,
    ffi::{c_void, CStr, CString},
    os::windows,
    ptr::null_mut,
};
use winapi::{
    shared::{
        dxgi::*,
        dxgi1_2::{IDXGIOutput1, IDXGIOutputDuplication},
        dxgiformat::{DXGI_FORMAT_R32G32B32_FLOAT, DXGI_FORMAT_R32G32_FLOAT},
        minwindef::TRUE,
    },
    um::{
        d3d11::{
            D3D11CreateDevice, ID3D11InputLayout, ID3D11PixelShader, D3D11_BIND_VERTEX_BUFFER,
            D3D11_BUFFER_DESC, D3D11_CREATE_DEVICE_BGRA_SUPPORT, D3D11_INPUT_ELEMENT_DESC,
            D3D11_INPUT_PER_VERTEX_DATA, D3D11_SDK_VERSION, D3D11_SUBRESOURCE_DATA,
            D3D11_USAGE_DEFAULT,
        },
        d3dcommon::{D3D11_PRIMITIVE_TOPOLOGY_TRIANGLELIST, D3D_DRIVER_TYPE_HARDWARE},
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

use super::shader;

#[repr(C)]
struct XMFLOAT2 {
    pub x: f32,
    pub y: f32,
}

#[repr(C)]
struct XMFLOAT3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[repr(C)]
struct VERTEX {
    pub pos: XMFLOAT3,
    pub tex_coord: XMFLOAT2,
}

static VERTEX_STRIDES: u32 = std::mem::size_of::<VERTEX>() as u32;

static VERTICES: [VERTEX; 6] = [
    VERTEX {
        pos: XMFLOAT3 {
            x: -1.0,
            y: -1.0,
            z: 0.0,
        },
        tex_coord: XMFLOAT2 { x: 0.0, y: 1.0 },
    },
    VERTEX {
        pos: XMFLOAT3 {
            x: -1.0,
            y: 1.0,
            z: 0.0,
        },
        tex_coord: XMFLOAT2 { x: 0.0, y: 0.0 },
    },
    VERTEX {
        pos: XMFLOAT3 {
            x: 1.0,
            y: -1.0,
            z: 0.0,
        },
        tex_coord: XMFLOAT2 { x: 1.0, y: 1.0 },
    },
    VERTEX {
        pos: XMFLOAT3 {
            x: 1.0,
            y: -1.0,
            z: 0.0,
        },
        tex_coord: XMFLOAT2 { x: 1.0, y: 1.0 },
    },
    VERTEX {
        pos: XMFLOAT3 {
            x: -1.0,
            y: 1.0,
            z: 0.0,
        },
        tex_coord: XMFLOAT2 { x: 0.0, y: 0.0 },
    },
    VERTEX {
        pos: XMFLOAT3 {
            x: 1.0,
            y: 1.0,
            z: 0.0,
        },
        tex_coord: XMFLOAT2 { x: 1.0, y: 0.0 },
    },
];

struct DuplicatonManager {
    d3d_device: ComPtr<ID3D11Device>,
    d3d_device_context: ComPtr<ID3D11DeviceContext>,
    adapter: ComPtr<IDXGIAdapter>,
    output: ComPtr<IDXGIOutput>,
    output_duplication: ComPtr<IDXGIOutputDuplication>,
    pixel_shader_y: ComPtr<ID3D11PixelShader>,
    pixel_shader_uv: ComPtr<ID3D11PixelShader>,
}

impl DuplicatonManager {
    fn new() -> anyhow::Result<Self> {
        shader::pixel_shader_y::makesure_compile()?;
        shader::pixel_shader_uv::makesure_compile()?;
        shader::vertex_shader::makesure_compile()?;

        unsafe {
            let (d3d_device, d3d_device_context, adapter, output, output_duplication) =
                create_dxgi()?;

            init_vertex_shader(&d3d_device, &d3d_device_context)?;
            let pixel_shader_y = init_pixel_shader_y(&d3d_device)?;
            let pixel_shader_uv = init_pixel_shader_uv(&d3d_device)?;

            Ok(DuplicatonManager {
                d3d_device,
                d3d_device_context,
                adapter,
                output,
                output_duplication,
                pixel_shader_y,
                pixel_shader_uv,
            })
        }
    }
}

unsafe fn enum_adapter_with_outputs(
) -> anyhow::Result<Vec<(ComPtr<IDXGIAdapter>, Vec<ComPtr<IDXGIOutput>>)>> {
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
        hr = dxgi_factory1_com_ptr.EnumAdapters(dxgi_adapter_enum_index, &mut dxgi_adapter_raw_ptr);
        if hr < 0 {
            break;
        }

        let dxgi_adapter_com_ptr = ComPtr::from_raw(dxgi_adapter_raw_ptr);

        let mut dxgi_output_com_ptrs = Vec::new();
        for dxgi_output_enum_index in 0.. {
            let mut dxgi_output_raw_ptr = std::ptr::null_mut();
            hr = dxgi_adapter_com_ptr.EnumOutputs(dxgi_output_enum_index, &mut dxgi_output_raw_ptr);
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

unsafe fn is_primary_output(output: &ComPtr<IDXGIOutput>) -> bool {
    let mut dxgi_output_desc = std::mem::zeroed();
    let hr = output.GetDesc(&mut dxgi_output_desc);
    if hr < 0 {
        return false;
    }

    let mut monitor_info: MONITORINFO = std::mem::zeroed();
    monitor_info.cbSize = std::mem::size_of::<MONITORINFO>() as u32;
    let success = GetMonitorInfoW(dxgi_output_desc.Monitor, &mut monitor_info);

    success == TRUE && ((monitor_info.dwFlags & 1) != 0)
}

unsafe fn create_dxgi() -> anyhow::Result<(
    ComPtr<ID3D11Device>,
    ComPtr<ID3D11DeviceContext>,
    ComPtr<IDXGIAdapter>,
    ComPtr<IDXGIOutput>,
    ComPtr<IDXGIOutputDuplication>,
)> {
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

unsafe fn init_vertex_shader(
    device: &ComPtr<ID3D11Device>,
    device_context: &ComPtr<ID3D11DeviceContext>,
) -> anyhow::Result<()> {
    let vertex_shader_bytes = shader::vertex_shader::VERTEX_SHADER
        .get()
        .ok_or(anyhow::anyhow!(
            "init_vertex_shader: vertex_shader not compiled"
        ))?;

    let mut vertex_shader = std::ptr::null_mut();
    let mut hr = device.CreateVertexShader(
        vertex_shader_bytes.as_ptr() as *const c_void,
        vertex_shader_bytes.len(),
        std::ptr::null_mut(),
        &mut vertex_shader,
    );
    if hr < 0 {
        bail!("init_vertex_shader: CreateVertexShader returns {:X}", hr);
    }

    device_context.VSSetShader(vertex_shader, &std::ptr::null_mut(), 0);

    let mut buffer_desc: D3D11_BUFFER_DESC = std::mem::zeroed();
    buffer_desc.Usage = D3D11_USAGE_DEFAULT;
    buffer_desc.ByteWidth = (std::mem::size_of::<VERTEX>() * VERTICES.len()) as u32;
    buffer_desc.BindFlags = D3D11_BIND_VERTEX_BUFFER;
    buffer_desc.CPUAccessFlags = 0;

    let mut subresource_data: D3D11_SUBRESOURCE_DATA = std::mem::zeroed();
    subresource_data.pSysMem = VERTICES.as_ptr() as *const c_void;

    let mut vertex_buffer = std::ptr::null_mut();
    hr = device.CreateBuffer(&buffer_desc, &subresource_data, &mut vertex_buffer);
    if hr < 0 {
        bail!("init_vertex_shader: CreateBuffer returns {:X}", hr);
    }

    device_context.IASetVertexBuffers(0, 1, &vertex_buffer, &VERTEX_STRIDES, &0);

    device_context.IASetPrimitiveTopology(D3D11_PRIMITIVE_TOPOLOGY_TRIANGLELIST);

    let input_element_descs = [
        D3D11_INPUT_ELEMENT_DESC {
            SemanticName: CString::new("POSITION")?.as_ptr(),
            SemanticIndex: 0,
            Format: DXGI_FORMAT_R32G32B32_FLOAT,
            InputSlot: 0,
            AlignedByteOffset: 0,
            InputSlotClass: D3D11_INPUT_PER_VERTEX_DATA,
            InstanceDataStepRate: 0,
        },
        D3D11_INPUT_ELEMENT_DESC {
            SemanticName: CString::new("TEXCOORD")?.as_ptr(),
            SemanticIndex: 0,
            Format: DXGI_FORMAT_R32G32_FLOAT,
            InputSlot: 0,
            AlignedByteOffset: 12,
            InputSlotClass: D3D11_INPUT_PER_VERTEX_DATA,
            InstanceDataStepRate: 0,
        },
    ];

    let mut input_layout = std::ptr::null_mut();
    hr = device.CreateInputLayout(
        input_element_descs.as_ptr(),
        input_element_descs.len() as u32,
        vertex_shader_bytes.as_ptr() as *const c_void,
        vertex_shader_bytes.len(),
        &mut input_layout,
    );
    if hr < 0 {
        bail!("init_vertex_shader: CreateInputLayout returns {:X}", hr);
    }

    device_context.IASetInputLayout(input_layout);

    Ok(())
}

unsafe fn init_pixel_shader_y(
    device: &ComPtr<ID3D11Device>,
) -> anyhow::Result<ComPtr<ID3D11PixelShader>> {
    let pixel_shader_y_bytes =
        shader::pixel_shader_y::PIXEL_SHADER_Y
            .get()
            .ok_or(anyhow::anyhow!(
                "init_pixel_shader_y: pixel_shader_y not compiled"
            ))?;

    let mut pixel_shader_y = std::ptr::null_mut();
    let hr = device.CreatePixelShader(
        pixel_shader_y_bytes.as_ptr() as *const c_void,
        pixel_shader_y_bytes.len(),
        std::ptr::null_mut(),
        &mut pixel_shader_y,
    );
    if hr < 0 {
        bail!("init_pixel_shader_y: CreatePixelShader returns {:X}", hr);
    }

    Ok(ComPtr::from_raw(pixel_shader_y))
}

unsafe fn init_pixel_shader_uv(
    device: &ComPtr<ID3D11Device>,
) -> anyhow::Result<ComPtr<ID3D11PixelShader>> {
    let pixel_shader_uv_bytes =
        shader::pixel_shader_uv::PIXEL_SHADER_UV
            .get()
            .ok_or(anyhow::anyhow!(
                "init_pixel_shader_uv: pixel_shader_uv not compiled"
            ))?;

    let mut pixel_shader_uv = std::ptr::null_mut();
    let hr = device.CreatePixelShader(
        pixel_shader_uv_bytes.as_ptr() as *const c_void,
        pixel_shader_uv_bytes.len(),
        std::ptr::null_mut(),
        &mut pixel_shader_uv,
    );
    if hr < 0 {
        bail!("init_pixel_shader_uv: CreatePixelShader returns {:X}", hr);
    }

    Ok(ComPtr::from_raw(pixel_shader_uv))
}
