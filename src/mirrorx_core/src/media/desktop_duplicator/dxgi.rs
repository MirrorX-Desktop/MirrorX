use super::shader;
use anyhow::bail;
use log::{as_error, error, info};
use rustc_hash::FxHashMap;
use std::{
    collections::HashMap,
    ffi::{c_void, CStr, CString},
    mem::zeroed,
    ptr::null_mut,
};
use windows::{
    core::IntoParam,
    Win32::{
        Foundation::S_OK,
        Graphics::{
            Direct3D::{
                D3D_DRIVER_TYPE_HARDWARE, D3D_DRIVER_TYPE_REFERENCE, D3D_DRIVER_TYPE_WARP,
                D3D_FEATURE_LEVEL, D3D_FEATURE_LEVEL_10_0, D3D_FEATURE_LEVEL_10_1,
                D3D_FEATURE_LEVEL_11_0, D3D_FEATURE_LEVEL_11_1,
            },
            Direct3D11::{
                D3D11CreateDevice, ID3D11Device, ID3D11DeviceContext,
                D3D11_CREATE_DEVICE_BGRA_SUPPORT, D3D11_CREATE_DEVICE_DEBUG, D3D11_SDK_VERSION,
            },
        },
    },
};

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

pub struct DuplicatonManager {
    d3d_device: ComPtr<ID3D11Device>,
    d3d_device_context: ComPtr<ID3D11DeviceContext>,
    adapter: ComPtr<IDXGIAdapter>,
    output: ComPtr<IDXGIOutput>,
    output_desc: Option<DXGI_OUTPUT_DESC>,
    output_duplication: ComPtr<IDXGIOutputDuplication>,
    back_texture: Option<ComPtr<ID3D11Texture2D>>,
    pixel_shader_y: ComPtr<ID3D11PixelShader>,
    pixel_shader_uv: ComPtr<ID3D11PixelShader>,
    y_plane_viewport: D3D11_VIEWPORT,
    uv_plane_viewport: D3D11_VIEWPORT,
    y_render_texture: Option<ComPtr<ID3D11Texture2D>>,
    uv_render_texture: Option<ComPtr<ID3D11Texture2D>>,
    y_staging_texture: Option<ComPtr<ID3D11Texture2D>>,
    uv_staging_texture: Option<ComPtr<ID3D11Texture2D>>,
    y_render_target_view: Option<ComPtr<ID3D11RenderTargetView>>,
    uv_render_target_view: Option<ComPtr<ID3D11RenderTargetView>>,
    vertex_buffer: ComPtr<ID3D11Buffer>,
    input_layout: ComPtr<ID3D11InputLayout>,
}

impl DuplicatonManager {
    pub fn new() -> anyhow::Result<Self> {
        unsafe {
            let (d3d_device, d3d_device_context, adapter, output, output_duplication) =
                create_dxgi()?;

            let (vertex_buffer, input_layout) =
                init_vertex_shader(&d3d_device, &d3d_device_context)?;
            let pixel_shader_y = init_pixel_shader_y(&d3d_device)?;
            let pixel_shader_uv = init_pixel_shader_uv(&d3d_device)?;

            Ok(DuplicatonManager {
                d3d_device,
                d3d_device_context,
                adapter,
                output,
                output_desc: None,
                output_duplication,
                back_texture: None,
                pixel_shader_y,
                pixel_shader_uv,
                y_plane_viewport: D3D11_VIEWPORT::default(),
                uv_plane_viewport: D3D11_VIEWPORT::default(),
                y_render_texture: None,
                uv_render_texture: None,
                y_staging_texture: None,
                uv_staging_texture: None,
                y_render_target_view: None,
                uv_render_target_view: None,
                vertex_buffer,
                input_layout,
            })
        }
    }

    pub fn capture_frame(&mut self) -> anyhow::Result<()> {
        unsafe {
            if self.back_texture.is_none() {
                self.init_back_texture()?;
            }

            self.capture_raw_frame()?;
            self.process_raw_frame()?;

            self.d3d_device_context.CopyResource(
                self.y_staging_texture
                    .as_ref()
                    .ok_or(anyhow::anyhow!(
                        "capture_frame: self y_staging_texture is None"
                    ))?
                    .as_raw() as *mut ID3D11Resource,
                self.y_render_texture
                    .as_ref()
                    .ok_or(anyhow::anyhow!(
                        "capture_frame: self y_render_texture is None"
                    ))?
                    .as_raw() as *mut ID3D11Resource,
            );

            self.d3d_device_context.CopyResource(
                self.uv_staging_texture
                    .as_ref()
                    .ok_or(anyhow::anyhow!(
                        "capture_frame: self uv_render_texture is None"
                    ))?
                    .as_raw() as *mut ID3D11Resource,
                self.uv_render_texture
                    .as_ref()
                    .ok_or(anyhow::anyhow!(
                        "capture_frame: self uv_render_texture is None"
                    ))?
                    .as_raw() as *mut ID3D11Resource,
            );

            let mut y_plane_mapped_resource: D3D11_MAPPED_SUBRESOURCE = std::mem::zeroed();
            let mut uv_plane_mapped_resource: D3D11_MAPPED_SUBRESOURCE = std::mem::zeroed();

            if let Some(y_staging_texture) = self.y_staging_texture.as_ref() {
                if let Some(uv_staging_texture) = self.uv_staging_texture.as_ref() {
                    self.d3d_device_context.Map(
                        y_staging_texture.as_raw() as *mut ID3D11Resource,
                        0,
                        D3D11_MAP_READ,
                        0,
                        &mut y_plane_mapped_resource,
                    );

                    self.d3d_device_context.Map(
                        uv_staging_texture.as_raw() as *mut ID3D11Resource,
                        0,
                        D3D11_MAP_READ,
                        0,
                        &mut uv_plane_mapped_resource,
                    );

                    self.d3d_device_context
                        .Unmap(y_staging_texture.as_raw() as *mut ID3D11Resource, 0);
                    self.d3d_device_context
                        .Unmap(uv_staging_texture.as_raw() as *mut ID3D11Resource, 0);
                }
            }

            Ok(())
        }
    }

    unsafe fn init_back_texture(&mut self) -> anyhow::Result<()> {
        let mut output_duplication_desc: DXGI_OUTDUPL_DESC = std::mem::zeroed();
        self.output_duplication
            .GetDesc(&mut output_duplication_desc);

        let mut back_texture_desc = D3D11_TEXTURE2D_DESC {
            Width: output_duplication_desc.ModeDesc.Width,
            Height: output_duplication_desc.ModeDesc.Height,
            MipLevels: 1,
            ArraySize: 1,
            Format: output_duplication_desc.ModeDesc.Format,
            SampleDesc: DXGI_SAMPLE_DESC {
                Count: 1,
                Quality: 0,
            },
            Usage: D3D11_USAGE_DEFAULT,
            BindFlags: D3D11_USAGE_DEFAULT,
            CPUAccessFlags: 0,
            MiscFlags: 0,
        };

        let mut back_texture = std::ptr::null_mut();
        let mut hr = self.d3d_device.CreateTexture2D(
            &back_texture_desc,
            std::ptr::null(),
            &mut back_texture,
        );
        if hr < 0 {
            bail!("init_back_texture: create back_texture returns 0x{:X}", hr);
        }

        self.back_texture = Some(ComPtr::from_raw(back_texture));

        // create y plane render texture
        back_texture_desc.Format = DXGI_FORMAT_R8_UNORM;
        back_texture_desc.BindFlags = D3D11_BIND_RENDER_TARGET;

        let mut y_render_texture = std::ptr::null_mut();
        hr = self.d3d_device.CreateTexture2D(
            &back_texture_desc,
            std::ptr::null(),
            &mut y_render_texture,
        );
        if hr < 0 {
            bail!(
                "init_back_texture: create y_render_texture returns 0x{:X}",
                hr
            );
        }

        self.y_render_texture = Some(ComPtr::from_raw(y_render_texture));

        // create y plane staging texture
        back_texture_desc.CPUAccessFlags = D3D11_CPU_ACCESS_READ;
        back_texture_desc.Usage = D3D11_USAGE_STAGING;
        back_texture_desc.BindFlags = 0;

        let mut y_staging_texture = std::ptr::null_mut();
        hr = self.d3d_device.CreateTexture2D(
            &back_texture_desc,
            std::ptr::null(),
            &mut y_staging_texture,
        );
        if hr < 0 {
            bail!(
                "init_back_texture: create y_staging_texture returns 0x{:X}",
                hr
            );
        }

        self.y_staging_texture = Some(ComPtr::from_raw(y_staging_texture));

        self.y_plane_viewport.TopLeftX = 0.0;
        self.y_plane_viewport.TopLeftY = 0.0;
        self.y_plane_viewport.Width = back_texture_desc.Width as f32;
        self.y_plane_viewport.Height = back_texture_desc.Height as f32;
        self.y_plane_viewport.MinDepth = 0.0;
        self.y_plane_viewport.MaxDepth = 1.0;

        let mut y_render_target_view = std::ptr::null_mut();
        hr = self.d3d_device.CreateRenderTargetView(
            y_render_texture.cast(),
            std::ptr::null(),
            &mut y_render_target_view,
        );
        if hr < 0 {
            bail!(
                "init_back_texture: create y_render_target_view returns 0x{:X}",
                hr
            );
        }
        self.y_render_target_view = Some(ComPtr::from_raw(y_render_target_view));

        // create uv plane render texture
        back_texture_desc.Width = output_duplication_desc.ModeDesc.Width / 2;
        back_texture_desc.Height = output_duplication_desc.ModeDesc.Height / 2;
        back_texture_desc.Format = DXGI_FORMAT_R8G8_UNORM;
        back_texture_desc.Usage = D3D11_USAGE_DEFAULT;
        back_texture_desc.CPUAccessFlags = 0;
        back_texture_desc.BindFlags = D3D11_BIND_RENDER_TARGET;

        let mut uv_render_texture = std::ptr::null_mut();
        hr = self.d3d_device.CreateTexture2D(
            &back_texture_desc,
            std::ptr::null(),
            &mut uv_render_texture,
        );
        if hr < 0 {
            bail!(
                "init_back_texture: create uv_render_texture returns 0x{:X}",
                hr
            );
        }

        self.uv_render_texture = Some(ComPtr::from_raw(uv_render_texture));

        // create uv plane staging texture
        back_texture_desc.CPUAccessFlags = D3D11_CPU_ACCESS_READ;
        back_texture_desc.Usage = D3D11_USAGE_STAGING;
        back_texture_desc.BindFlags = 0;

        let mut uv_staging_texture = std::ptr::null_mut();
        hr = self.d3d_device.CreateTexture2D(
            &back_texture_desc,
            std::ptr::null(),
            &mut uv_staging_texture,
        );
        if hr < 0 {
            bail!(
                "init_back_texture: create uv_staging_texture returns 0x{:X}",
                hr
            );
        }

        self.uv_staging_texture = Some(ComPtr::from_raw(uv_staging_texture));

        self.uv_plane_viewport.TopLeftX = 0.0;
        self.uv_plane_viewport.TopLeftY = 0.0;
        self.uv_plane_viewport.Width = back_texture_desc.Width as f32;
        self.uv_plane_viewport.Height = back_texture_desc.Height as f32;
        self.uv_plane_viewport.MinDepth = 0.0;
        self.uv_plane_viewport.MaxDepth = 1.0;

        let mut uv_render_target_view = std::ptr::null_mut();
        hr = self.d3d_device.CreateRenderTargetView(
            uv_render_texture.cast(),
            std::ptr::null(),
            &mut uv_render_target_view,
        );
        if hr < 0 {
            bail!(
                "init_back_texture: create uv_render_target_view returns 0x{:X}",
                hr
            );
        }

        self.uv_render_target_view = Some(ComPtr::from_raw(uv_render_target_view));

        Ok(())
    }

    unsafe fn capture_raw_frame(&self) -> anyhow::Result<()> {
        let mut desktop_resource = std::ptr::null_mut();
        let mut frame_info = std::mem::zeroed();
        let hr = self.output_duplication.AcquireNextFrame(
            INFINITE,
            &mut frame_info,
            &mut desktop_resource,
        );
        if hr < 0 {
            bail!("capture_raw_frame: AcquireNextFrame returns 0x{:X}", hr);
        }

        let desktop_resource_com_ptr = ComPtr::from_raw(desktop_resource);

        let acquired_desktop_image =
            desktop_resource_com_ptr
                .cast::<ID3D11Texture2D>()
                .or_else(|err| {
                    bail!(
                        "capture_raw_frame: cast IDXGIResouce to ID3D11Texture2D failed: {}",
                        err
                    )
                })?;

        if let Some(back_texture) = &self.back_texture {
            self.d3d_device_context.CopyResource(
                back_texture.as_raw() as *mut ID3D11Resource,
                acquired_desktop_image.as_raw() as *mut ID3D11Resource,
            );
            self.output_duplication.ReleaseFrame();
            Ok(())
        } else {
            bail!("capture_raw_frame: self back_texture is None");
        }
    }

    unsafe fn process_raw_frame(&self) -> anyhow::Result<()> {
        let mut shader_texture_desc: D3D11_TEXTURE2D_DESC = std::mem::zeroed();
        let back_texture = self.back_texture.as_ref().ok_or(anyhow::anyhow!(
            "process_raw_frame: self back_texture is None"
        ))?;

        back_texture.GetDesc(&mut shader_texture_desc);

        let mut shader_resouce_view_desc_u: D3D11_SHADER_RESOURCE_VIEW_DESC_u = std::mem::zeroed();
        let mut ppk = shader_resouce_view_desc_u.Texture2D_mut();
        ppk.MostDetailedMip = shader_texture_desc.MipLevels - 1;
        ppk.MipLevels = shader_texture_desc.MipLevels;

        let shader_resouce_view_desc = D3D11_SHADER_RESOURCE_VIEW_DESC {
            Format: shader_texture_desc.Format,
            ViewDimension: D3D11_SRV_DIMENSION_TEXTURE2D,
            u: shader_resouce_view_desc_u,
        };

        let mut shader_resouce_view = std::ptr::null_mut();
        let hr = self.d3d_device.CreateShaderResourceView(
            back_texture.as_raw() as *mut ID3D11Resource,
            &shader_resouce_view_desc,
            &mut shader_resouce_view,
        );
        if hr < 0 {
            bail!(
                "process_raw_frame: CreateShaderResourceView returns 0x{:X}",
                hr
            );
        }

        self.d3d_device_context
            .PSSetShaderResources(0, 1, &shader_resouce_view);

        // draw y plane
        if let Some(y_plane_target_view) = self.y_render_target_view.as_ref() {
            self.d3d_device_context.OMSetRenderTargets(
                1,
                &y_plane_target_view.as_raw(),
                std::ptr::null_mut(),
            );
            self.d3d_device_context.PSSetShader(
                self.pixel_shader_y.as_raw(),
                &std::ptr::null_mut(),
                0,
            );
            self.d3d_device_context
                .RSSetViewports(1, &self.y_plane_viewport);
            self.d3d_device_context.Draw(VERTICES.len() as u32, 0);
        }

        // draw uv plane
        if let Some(uv_plane_target_view) = self.uv_render_target_view.as_ref() {
            self.d3d_device_context.OMSetRenderTargets(
                1,
                &uv_plane_target_view.as_raw(),
                std::ptr::null_mut(),
            );
            self.d3d_device_context.PSSetShader(
                self.pixel_shader_uv.as_raw(),
                &std::ptr::null_mut(),
                0,
            );
            self.d3d_device_context
                .RSSetViewports(1, &self.uv_plane_viewport);
            self.d3d_device_context.Draw(VERTICES.len() as u32, 0);
        }

        (*shader_resouce_view).Release();

        Ok(())
    }
}

unsafe fn enum_adapter_with_outputs(
) -> anyhow::Result<Vec<(ComPtr<IDXGIAdapter>, Vec<ComPtr<IDXGIOutput>>)>> {
    let mut dxgi_factory1_raw_ptr = std::ptr::null_mut();
    let mut hr = CreateDXGIFactory1(&IID_IDXGIFactory1, &mut dxgi_factory1_raw_ptr);
    if hr < 0 {
        bail!(
            "enum_adapter_with_outputs: CreateDXGIFactory1 returns 0x{:X}",
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
    // let h_desk = OpenInputDesktop(0, FALSE, GENERIC_ALL);
    // if h_desk.is_null() {
    //     bail!("create_dxgi: open input desktop failed");
    // }

    // // Attach desktop to this thread (presumably for cases where this is not the main/UI thread)
    // let desk_attached = SetThreadDesktop(h_desk) != 0;
    // CloseDesktop(h_desk);
    // drop(h_desk);

    // if !desk_attached {
    //     bail!("Failed to attach recording thread to desktop");
    // }

    // let adapter_with_outputs = enum_adapter_with_outputs()?;
    // let (primary_adapter_com_ptr, primary_output_com_ptr) =
    //     || -> Option<(ComPtr<IDXGIAdapter>, ComPtr<IDXGIOutput>)> {
    //         for (adapter, outputs) in adapter_with_outputs {
    //             for output in outputs {
    //                 if is_primary_output(&output) {
    //                     return Some((adapter, output));
    //                 }
    //             }
    //         }
    //         return None;
    //     }()
    //     .ok_or(anyhow::anyhow!(
    //         "create_d3d11_device_and_context: no primary output"
    //     ))?;
    let driver_types = [
        D3D_DRIVER_TYPE_HARDWARE,
        D3D_DRIVER_TYPE_WARP,
        D3D_DRIVER_TYPE_REFERENCE,
    ];

    let feature_levels = [
        D3D_FEATURE_LEVEL_11_1,
        D3D_FEATURE_LEVEL_11_0,
        D3D_FEATURE_LEVEL_10_1,
        D3D_FEATURE_LEVEL_10_0,
    ];

    let mut device: Option<ID3D11Device> = None;
    let mut device_context: Option<ID3D11DeviceContext> = None;
    let mut feature_level: D3D_FEATURE_LEVEL;

    let mut hr = S_OK;
    for driver_type in driver_types {
        match D3D11CreateDevice(
            None,
            driver_type,
            None,
            D3D11_CREATE_DEVICE_BGRA_SUPPORT | D3D11_CREATE_DEVICE_DEBUG,
            &feature_levels,
            D3D11_SDK_VERSION,
            &mut device,
            &mut feature_level,
            &mut device_context,
        ) {
            Ok(_) => {
                info!(driver_type = format!("{:?}", driver_type), feature_level = format!("{:?}", feature_level); "create_dxgi: create device success");
                break;
            }
            Err(err) => {
                error!(driver_type = format!("{:?}", driver_type), error = as_error!(err); "create_dxgi: failed to create device")
            }
        };
    }

    if device.is_none() || device_context.is_none() {
        bail!("create_dxgi: create device failed with all driver types");
    }

    
    let dxgi_output1_com_ptr = primary_output_com_ptr
        .cast::<IDXGIOutput1>()
        .or_else(|hr| {
            bail!(
                "create_dxgi: cast IDXGIOutput to IDXGIOutput1 returns 0x{:X}",
                hr
            )
        })?;

    let mut dxgi_output_duplication_raw_ptr = std::ptr::null_mut();

    let hr = dxgi_output1_com_ptr.DuplicateOutput(
        d3d11_device_raw_ptr as *mut IUnknown,
        &mut dxgi_output_duplication_raw_ptr,
    );

    if hr < 0 {
        // error: maybe another session duplicating
        bail!("create_dxgi: DuplicateOutput returns 0x{:X}", hr);
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
) -> anyhow::Result<(ComPtr<ID3D11Buffer>, ComPtr<ID3D11InputLayout>)> {
    let mut vertex_shader = std::ptr::null_mut();
    let mut hr = device.CreateVertexShader(
        shader::VERTEX_SHADER_BYTES.as_ptr() as *const c_void,
        shader::VERTEX_SHADER_BYTES.len(),
        std::ptr::null_mut(),
        &mut vertex_shader,
    );
    if hr < 0 {
        bail!("init_vertex_shader: CreateVertexShader returns 0x{:X}", hr);
    }

    device_context.VSSetShader(vertex_shader, &std::ptr::null_mut(), 0);

    let mut buffer_desc: D3D11_BUFFER_DESC = std::mem::zeroed();
    buffer_desc.Usage = D3D11_USAGE_DEFAULT;
    buffer_desc.ByteWidth = VERTEX_STRIDES * VERTICES.len() as u32;
    buffer_desc.BindFlags = D3D11_BIND_VERTEX_BUFFER;
    buffer_desc.CPUAccessFlags = 0;

    let mut subresource_data: D3D11_SUBRESOURCE_DATA = std::mem::zeroed();
    subresource_data.pSysMem = VERTICES.as_ptr() as *const c_void;

    let mut vertex_buffer = std::ptr::null_mut();
    hr = device.CreateBuffer(&buffer_desc, &subresource_data, &mut vertex_buffer);
    if hr < 0 {
        bail!("init_vertex_shader: CreateBuffer returns 0x{:X}", hr);
    }

    device_context.IASetVertexBuffers(0, 1, &vertex_buffer, &VERTEX_STRIDES, &0);
    device_context.IASetPrimitiveTopology(D3D11_PRIMITIVE_TOPOLOGY_TRIANGLELIST);

    let input_element_descs = [
        D3D11_INPUT_ELEMENT_DESC {
            SemanticName: b"POSITION\0".as_ptr() as *const i8,
            SemanticIndex: 0,
            Format: DXGI_FORMAT_R32G32B32_FLOAT,
            InputSlot: 0,
            AlignedByteOffset: 0,
            InputSlotClass: D3D11_INPUT_PER_VERTEX_DATA,
            InstanceDataStepRate: 0,
        },
        D3D11_INPUT_ELEMENT_DESC {
            SemanticName: b"TEXCOORD\0".as_ptr() as *const i8,
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
        shader::VERTEX_SHADER_BYTES.as_ptr() as *const c_void,
        shader::VERTEX_SHADER_BYTES.len(),
        &mut input_layout,
    );
    if hr < 0 {
        bail!("init_vertex_shader: CreateInputLayout returns 0x{:X}", hr);
    }

    device_context.IASetInputLayout(input_layout);

    Ok((
        ComPtr::from_raw(vertex_buffer),
        ComPtr::from_raw(input_layout),
    ))
}

unsafe fn init_pixel_shader_y(
    device: &ComPtr<ID3D11Device>,
) -> anyhow::Result<ComPtr<ID3D11PixelShader>> {
    let mut pixel_shader_y = std::ptr::null_mut();
    let hr = device.CreatePixelShader(
        shader::PIXEL_SHADER_Y_BYTES.as_ptr() as *const c_void,
        shader::PIXEL_SHADER_Y_BYTES.len(),
        std::ptr::null_mut(),
        &mut pixel_shader_y,
    );
    if hr < 0 {
        bail!("init_pixel_shader_y: CreatePixelShader returns 0x{:X}", hr);
    }

    Ok(ComPtr::from_raw(pixel_shader_y))
}

unsafe fn init_pixel_shader_uv(
    device: &ComPtr<ID3D11Device>,
) -> anyhow::Result<ComPtr<ID3D11PixelShader>> {
    let mut pixel_shader_uv = std::ptr::null_mut();
    let hr = device.CreatePixelShader(
        shader::PIXEL_SHADER_UV_BYTES.as_ptr() as *const c_void,
        shader::PIXEL_SHADER_UV_BYTES.len(),
        std::ptr::null_mut(),
        &mut pixel_shader_uv,
    );
    if hr < 0 {
        bail!("init_pixel_shader_uv: CreatePixelShader returns 0x{:X}", hr);
    }

    Ok(ComPtr::from_raw(pixel_shader_uv))
}
