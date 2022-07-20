use super::{
    dx::DX,
    dx_math::{BPP, VERTEX, VERTICES},
};
use crate::{component::desktop::Frame, error::MirrorXError, utility::wide_char::FromWide};
use anyhow::bail;
use scopeguard::defer;
use std::{
    ffi::OsString,
    mem::zeroed,
    ops::{Shr, ShrAssign},
    os::raw::c_void,
    ptr::null,
};
use tracing::info;
use windows::{
    core::Interface,
    Win32::{
        Graphics::{
            Direct3D::*,
            Direct3D11::*,
            Dxgi::{Common::*, *},
            Gdi::*,
        },
        System::{StationsAndDesktops::*, SystemServices::*, WindowsProgramming::INFINITE},
        UI::WindowsAndMessaging::*,
    },
};

pub struct Duplicator {
    dx: DX,
    output_desc: DXGI_OUTPUT_DESC,
    output_duplication: IDXGIOutputDuplication,
    backend_texture: ID3D11Texture2D,
    // backend_render_target_view: ID3D11RenderTargetView,
    render_texture_lumina: ID3D11Texture2D,
    render_texture_chrominance: ID3D11Texture2D,
    staging_texture_lumina: ID3D11Texture2D,
    staging_texture_chrominance: ID3D11Texture2D,
    view_port_lumina: D3D11_VIEWPORT,
    view_port_chrominance: D3D11_VIEWPORT,
    render_target_view_lumina: ID3D11RenderTargetView,
    render_target_view_chrominance: ID3D11RenderTargetView,
    sampler_linear: [Option<ID3D11SamplerState>; 1],
    blend_state: ID3D11BlendState,
}

unsafe impl Send for Duplicator {}

impl Duplicator {
    pub fn new(monitor_id: &str) -> Result<Duplicator, MirrorXError> {
        unsafe {
            let current_desktop = OpenInputDesktop(0, false, GENERIC_ALL).map_err(|err| {
                MirrorXError::Other(anyhow::anyhow!("OpenInputDesktop failed ({})", err))
            })?;

            defer! {
                let _ = CloseDesktop(current_desktop);
            }

            if !SetThreadDesktop(current_desktop).as_bool() {
                return Err(MirrorXError::Other(anyhow::anyhow!(
                    "SetThreadDesktop failed"
                )));
            }

            let dx = DX::new()?;
            let (output_desc, output_duplication) = init_output_duplication(&dx, monitor_id)?;

            let mut dxgi_outdupl_desc = zeroed();
            output_duplication.GetDesc(&mut dxgi_outdupl_desc);

            let mut texture_desc = D3D11_TEXTURE2D_DESC {
                Width: dxgi_outdupl_desc.ModeDesc.Width,
                Height: dxgi_outdupl_desc.ModeDesc.Height,
                MipLevels: 1,
                ArraySize: 1,
                Format: dxgi_outdupl_desc.ModeDesc.Format,
                SampleDesc: DXGI_SAMPLE_DESC {
                    Count: 1,
                    Quality: 0,
                },
                Usage: D3D11_USAGE_DEFAULT,
                BindFlags: D3D11_BIND_SHADER_RESOURCE | D3D11_BIND_RENDER_TARGET,
                CPUAccessFlags: D3D11_CPU_ACCESS_FLAG::default(),
                MiscFlags: D3D11_RESOURCE_MISC_GDI_COMPATIBLE,
            };

            let backend_texture = dx.device()
                .CreateTexture2D(&texture_desc, null())
                .map_err(|err| {
                    anyhow::anyhow!(
                        r#"Duplication: ID3D11Device::CreateTexture2D failed {{"texture_name":"{}", "error": "{:?}"}}"#,
                        "backend_texture",
                        err.code()
                    )
                })?;

            // let backend_render_target_view = dx.device().CreateRenderTargetView(&backend_texture,null()).map_err(|err|{
            //         anyhow::anyhow!(
            //             r#"Duplication: ID3D11Device::CreateRenderTargetView failed {{"texture_name":"{}", "error": "{:?}"}}"#,
            //             "backend_texture",
            //             err.code()
            //         )
            //     })?;

            // create lumina plane resource

            texture_desc.Format = DXGI_FORMAT_R8_UNORM;
            texture_desc.BindFlags = D3D11_BIND_RENDER_TARGET;
            texture_desc.MiscFlags = D3D11_RESOURCE_MISC_FLAG::default();
            let render_texture_lumina = dx. device()
                .CreateTexture2D(&texture_desc, null())
                .map_err(|err| {
                    anyhow::anyhow!(
                        r#"Duplication: ID3D11Device::CreateTexture2D failed {{"texture_name":"{}", "error": "{:?}"}}"#,
                        "render_texture_lumina",
                        err.code()
                    )
                })?;

            texture_desc.CPUAccessFlags = D3D11_CPU_ACCESS_READ;
            texture_desc.Usage = D3D11_USAGE_STAGING;
            texture_desc.BindFlags = D3D11_BIND_FLAG(0);
            let staging_texture_lumina = dx.device().CreateTexture2D(&texture_desc, null()).map_err(|err|{
                anyhow::anyhow!(
                    r#"Duplication: ID3D11Device::CreateTexture2D failed {{"texture_name":"{}", "error": "{:?}"}}"#,
                    "staging_texture_lumina",
                    err.code()
                )
            })?;

            let view_port_lumina = D3D11_VIEWPORT {
                TopLeftX: 0.0,
                TopLeftY: 0.0,
                Width: texture_desc.Width as f32,
                Height: texture_desc.Height as f32,
                MinDepth: 0.0,
                MaxDepth: 1.0,
            };

            let render_target_view_lumina = dx.device().CreateRenderTargetView(&render_texture_lumina,null()).map_err(|err|{
                anyhow::anyhow!(
                    r#"Duplication: ID3D11Device::CreateRenderTargetView failed {{"texture_name":"{}", "error": "{:?}"}}"#,
                    "render_texture_lumina",
                    err.code()
                )
            })?;

            // create chrominance plane resource

            texture_desc.Width = dxgi_outdupl_desc.ModeDesc.Width / 2;
            texture_desc.Height = dxgi_outdupl_desc.ModeDesc.Height / 2;
            texture_desc.Format = DXGI_FORMAT_R8G8_UNORM;
            texture_desc.Usage = D3D11_USAGE_DEFAULT;
            texture_desc.CPUAccessFlags = D3D11_CPU_ACCESS_FLAG(0);
            texture_desc.BindFlags = D3D11_BIND_RENDER_TARGET;

            let render_texture_chrominance = dx.device()
                .CreateTexture2D(&texture_desc, null())
                .map_err(|err| {
                    anyhow::anyhow!(
                        r#"Duplication: ID3D11Device::CreateTexture2D failed {{"texture_name":"{}", "error": "{:?}"}}"#,
                        "render_texture_chrominance",
                        err.code()
                    )
                })?;

            texture_desc.CPUAccessFlags = D3D11_CPU_ACCESS_READ;
            texture_desc.Usage = D3D11_USAGE_STAGING;
            texture_desc.BindFlags = D3D11_BIND_FLAG(0);
            let staging_texture_chrominance =dx. device().CreateTexture2D(&texture_desc, null()).map_err(|err|{
                anyhow::anyhow!(
                    r#"Duplication: ID3D11Device::CreateTexture2D failed {{"texture_name":"{}", "error": "{:?}"}}"#,
                    "staging_texture_chrominance",
                    err.code()
                )
            })?;

            let view_port_chrominance = D3D11_VIEWPORT {
                TopLeftX: 0.0,
                TopLeftY: 0.0,
                Width: texture_desc.Width as f32,
                Height: texture_desc.Height as f32,
                MinDepth: 0.0,
                MaxDepth: 1.0,
            };

            let render_target_view_chrominance = dx.device().CreateRenderTargetView(&render_texture_chrominance,null()).map_err(|err|{
                anyhow::anyhow!(
                    r#"Duplication: ID3D11Device::CreateRenderTargetView failed {{"texture_name":"{}", "error": "{:?}"}}"#,
                    "render_texture_chrominance",
                    err.code()
                )
            })?;

            let mut samp_desc: D3D11_SAMPLER_DESC = zeroed();
            samp_desc.Filter = D3D11_FILTER_MIN_MAG_MIP_LINEAR;
            samp_desc.AddressU = D3D11_TEXTURE_ADDRESS_CLAMP;
            samp_desc.AddressV = D3D11_TEXTURE_ADDRESS_CLAMP;
            samp_desc.AddressW = D3D11_TEXTURE_ADDRESS_CLAMP;
            samp_desc.ComparisonFunc = D3D11_COMPARISON_NEVER;
            samp_desc.MinLOD = 0f32;
            samp_desc.MaxLOD = D3D11_FLOAT32_MAX;

            let sampler_linear = dx
                .device()
                .CreateSamplerState(&samp_desc)
                .map_err(|err| anyhow::anyhow!(err))?;

            let mut blend_state_desc: D3D11_BLEND_DESC = zeroed();
            blend_state_desc.AlphaToCoverageEnable = false.into();
            blend_state_desc.IndependentBlendEnable = false.into();
            blend_state_desc.RenderTarget[0].BlendEnable = true.into();
            blend_state_desc.RenderTarget[0].SrcBlend = D3D11_BLEND_SRC_ALPHA;
            blend_state_desc.RenderTarget[0].DestBlend = D3D11_BLEND_INV_SRC_ALPHA;
            blend_state_desc.RenderTarget[0].BlendOp = D3D11_BLEND_OP_ADD;
            blend_state_desc.RenderTarget[0].SrcBlendAlpha = D3D11_BLEND_ONE;
            blend_state_desc.RenderTarget[0].DestBlendAlpha = D3D11_BLEND_ZERO;
            blend_state_desc.RenderTarget[0].BlendOpAlpha = D3D11_BLEND_OP_ADD;
            blend_state_desc.RenderTarget[0].RenderTargetWriteMask =
                D3D11_COLOR_WRITE_ENABLE_ALL.0 as u8;

            let blend_state = dx
                .device()
                .CreateBlendState(&blend_state_desc)
                .map_err(|err| anyhow::anyhow!(err))?;

            Ok(Duplicator {
                dx,
                output_desc,
                output_duplication,
                backend_texture,
                // backend_render_target_view,
                render_texture_lumina,
                render_texture_chrominance,
                staging_texture_lumina,
                staging_texture_chrominance,
                view_port_lumina,
                view_port_chrominance,
                render_target_view_lumina,
                render_target_view_chrominance,
                sampler_linear: [Some(sampler_linear); 1],
                blend_state,
            })
        }
    }

    pub fn capture(&mut self) -> anyhow::Result<Frame> {
        unsafe {
            self.acquire_frame()?;
            self.process_frame()?;

            self.dx
                .device_context()
                .CopyResource(&self.staging_texture_lumina, &self.render_texture_lumina);

            self.dx.device_context().CopyResource(
                &self.staging_texture_chrominance,
                &self.render_texture_chrominance,
            );

            let mapped_resource_lumina = self.dx.device_context().Map(&self.staging_texture_lumina, 0, D3D11_MAP_READ, 0).map_err(|err|{
                anyhow::anyhow!(
                    r#"Duplication: ID3D11DeviceContext::Map failed {{"resource_name": "{}", "error": "{:?}"}}"#,
                    "staging_texture_lumina",
                    err.code()
                )
            })?;

            defer! {
                self.dx
                .device_context()
                .Unmap(&self.staging_texture_lumina, 0);
            }

            let mapped_resource_chrominance = self.dx.device_context().Map(&self.staging_texture_chrominance, 0, D3D11_MAP_READ, 0).map_err(|err|{
                anyhow::anyhow!(
                    r#"Duplication: ID3D11DeviceContext::Map failed {{"resource_name": "{}", "error": "{:?}"}}"#,
                    "staging_texture_chrominance",
                    err.code()
                )
            })?;

            defer! {
                self.dx
                .device_context()
                .Unmap(&self.staging_texture_chrominance, 0);
            }

            let width = self.output_desc.DesktopCoordinates.right
                - self.output_desc.DesktopCoordinates.left;

            let height = self.output_desc.DesktopCoordinates.bottom
                - self.output_desc.DesktopCoordinates.top;

            let luminance_buffer_size = (height as u32) * mapped_resource_lumina.RowPitch;
            let chrominance_buffer_size =
                (height as u32) / 2 * mapped_resource_chrominance.RowPitch;

            let capture_frame = Frame::new(
                width as u16,
                height as u16,
                std::slice::from_raw_parts(
                    mapped_resource_lumina.pData as *mut u8,
                    luminance_buffer_size as usize,
                )
                .to_vec(),
                mapped_resource_lumina.RowPitch as u16,
                std::slice::from_raw_parts(
                    mapped_resource_chrominance.pData as *mut u8,
                    chrominance_buffer_size as usize,
                )
                .to_vec(),
                mapped_resource_chrominance.RowPitch as u16,
                0,
            );

            Ok(capture_frame)
        }
    }

    unsafe fn acquire_frame(&mut self) -> anyhow::Result<()> {
        let mut dxgi_resource = None;
        let mut dxgi_outdupl_frame_info = zeroed();

        let mut failures = 0;
        while failures < 10 {
            let hr = match self.output_duplication.AcquireNextFrame(
                INFINITE,
                &mut dxgi_outdupl_frame_info,
                &mut dxgi_resource,
            ) {
                Ok(_) => break,
                Err(err) => {
                    failures += 1;
                    err.code()
                }
            };

            if failures > 10 {
                bail!(
                    r#"Duplication: IDXGIOutputDuplication::AcquireNextFrame too many failures, {{"last_error": "{:?}"}}"#,
                    hr
                );
            }

            if hr == DXGI_ERROR_ACCESS_LOST {
                tracing::warn!("Duplication: IDXGIOutputDuplication::AcquireNextFrame returns DXGI_ERROR_ACCESS_LOST, re-init DXGIOutputDuplication");

                // todo

                // let _ = self.output_duplication.ReleaseFrame();

                // std::ptr::drop_in_place(&mut self.output_duplication);

                // let (dxgi_output_desc, dxgi_output_duplication) =
                //     init_output_duplication(&self.dx, 0)?;

                // self.output_duplication = dxgi_output_duplication;
                // self.output_desc = dxgi_output_desc;
            }
        }

        let desktop_texture :ID3D11Texture2D = match dxgi_resource{
                Some(resource)=>resource.cast().map_err(|err|{
                    anyhow::anyhow!(
                        r#"Duplication: IDXGIResource::QueryInterface for ID3D11Texture2D failed {{"error": "{:?}"}}"#,         
                        err.code()
                    )
                })?,
                None=>bail!("Duplication: IDXGIOutputDuplication::AcquireNextFrame success but referenced IDXGIResource is null"),
            };

        self.dx
            .device_context()
            .CopyResource(&self.backend_texture, desktop_texture);

        // // draw mouse
        // self.draw_mouse(&dxgi_outdupl_frame_info)?;
        self.output_duplication.ReleaseFrame().map_err(|err| {
            anyhow::anyhow!(
                r#"Duplication: IDXGIOutputDuplication::ReleaseFrame failed {{"error": "{:?}"}}"#,
                err.code()
            )
        })?;

        let backend_surface: IDXGISurface1 = self
            .backend_texture
            .cast()
            .map_err(|err| anyhow::anyhow!(err))?;

        let mut cursor_info: CURSORINFO = zeroed();
        cursor_info.cbSize = std::mem::size_of::<CURSORINFO>() as u32;

        if GetCursorInfo(&mut cursor_info).as_bool() {
            if cursor_info.flags == CURSOR_SHOWING {
                let cursorPosition = cursor_info.ptScreenPos;
                let lCursorSize = cursor_info.cbSize;

                let hdc = backend_surface
                    .GetDC(false)
                    .map_err(|err| anyhow::anyhow!(err))?;

                DrawIconEx(
                    hdc,
                    cursorPosition.x,
                    cursorPosition.y,
                    cursor_info.hCursor,
                    0,
                    0,
                    0,
                    None,
                    DI_NORMAL | DI_DEFAULTSIZE,
                );

                backend_surface.ReleaseDC(null());
            }
        }

        Ok(())
    }

    unsafe fn process_frame(&self) -> anyhow::Result<()> {
        let mut texture2d_desc = zeroed();
        self.backend_texture.GetDesc(&mut texture2d_desc);

        let shader_resouce_view_desc = D3D11_SHADER_RESOURCE_VIEW_DESC {
            Format: texture2d_desc.Format,
            ViewDimension: D3D11_SRV_DIMENSION_TEXTURE2D,
            Anonymous: D3D11_SHADER_RESOURCE_VIEW_DESC_0 {
                Texture2D: D3D11_TEX2D_SRV {
                    MostDetailedMip: texture2d_desc.MipLevels - 1,
                    MipLevels: texture2d_desc.MipLevels,
                },
            },
        };

        let shader_resouce_view = self.dx.device().CreateShaderResourceView(&self.backend_texture,&shader_resouce_view_desc).map_err(|err|{
            anyhow::anyhow!(
                r#"Duplication: ID3D11Device::CreateShaderResourceView failed {{"error": "{:?}"}}"#,
                err.code()
            )
        })?;

        self.dx
            .device_context()
            .PSSetShaderResources(0, &vec![Some(shader_resouce_view)]);

        // draw lumina plane
        self.dx
            .device_context()
            .OMSetRenderTargets(&[Some(self.render_target_view_lumina.clone())], None);
        self.dx
            .device_context()
            .PSSetShader(self.dx.pixel_shader_lumina(), &[]);
        self.dx
            .device_context()
            .RSSetViewports(&[self.view_port_lumina]);
        self.dx.device_context().Draw(VERTICES.len() as u32, 0);

        // draw chrominance plane
        self.dx
            .device_context()
            .OMSetRenderTargets(&[Some(self.render_target_view_chrominance.clone())], None);
        self.dx
            .device_context()
            .PSSetShader(self.dx.pixel_shader_chrominance(), &[]);
        self.dx
            .device_context()
            .RSSetViewports(&[self.view_port_chrominance]);
        self.dx.device_context().Draw(VERTICES.len() as u32, 0);

        Ok(())
    }

    // unsafe fn draw_mouse(
    //     &self,
    //     dxgi_outdupl_frame_info: &DXGI_OUTDUPL_FRAME_INFO,
    // ) -> anyhow::Result<()> {
    //     let mut cursor_shape_buffer =
    //         Vec::<u8>::with_capacity(dxgi_outdupl_frame_info.PointerShapeBufferSize as usize);
    //     let mut cursor_shape_buffer_length = 0u32;
    //     let mut cursor_shape_info: DXGI_OUTDUPL_POINTER_SHAPE_INFO = zeroed();

    //     self.output_duplication
    //         .GetFramePointerShape(
    //             dxgi_outdupl_frame_info.PointerShapeBufferSize,
    //             &mut cursor_shape_buffer as *mut _ as *mut std::os::raw::c_void,
    //             &mut cursor_shape_buffer_length,
    //             &mut cursor_shape_info,
    //         )
    //         .map_err(|err| {
    //             anyhow::anyhow!(
    //                 "IDXGIOutputDuplication::GetFramePointerShape failed ({})",
    //                 err
    //             )
    //         })?;

    //     cursor_shape_buffer.set_len(dxgi_outdupl_frame_info.PointerShapeBufferSize as usize);

    //     let mut desktop_texture_desc: D3D11_TEXTURE2D_DESC = zeroed();

    //     self.backend_texture.GetDesc(&mut desktop_texture_desc);

    //     let desktop_width = desktop_texture_desc.Width;
    //     let desktop_height = desktop_texture_desc.Height;

    //     // Center of desktop dimensions
    //     let center_x = (desktop_width as f32) / 2f32;
    //     let center_y = (desktop_height as f32) / 2f32;

    //     // Buffer used if necessary (in case of monochrome or masked pointer)
    //     let mut init_buffer = Vec::<u8>::new();

    //     let mut cursor_texture_desc: D3D11_TEXTURE2D_DESC = zeroed();
    //     cursor_texture_desc.MipLevels = 1;
    //     cursor_texture_desc.ArraySize = 1;
    //     cursor_texture_desc.Format = DXGI_FORMAT_B8G8R8A8_UNORM;
    //     cursor_texture_desc.SampleDesc.Count = 1;
    //     cursor_texture_desc.SampleDesc.Quality = 0;
    //     cursor_texture_desc.Usage = D3D11_USAGE_DEFAULT;
    //     cursor_texture_desc.BindFlags = D3D11_BIND_SHADER_RESOURCE;
    //     cursor_texture_desc.CPUAccessFlags = D3D11_CPU_ACCESS_FLAG::default();
    //     cursor_texture_desc.MiscFlags = D3D11_RESOURCE_MISC_FLAG::default();

    //     // Set shader resource properties
    //     let mut cursor_shader_resource_view_desc: D3D11_SHADER_RESOURCE_VIEW_DESC = zeroed();
    //     cursor_shader_resource_view_desc.Format = cursor_texture_desc.Format;
    //     cursor_shader_resource_view_desc.ViewDimension = D3D11_SRV_DIMENSION_TEXTURE2D;
    //     cursor_shader_resource_view_desc
    //         .Anonymous
    //         .Texture2D
    //         .MostDetailedMip = cursor_texture_desc.MipLevels - 1;
    //     cursor_shader_resource_view_desc
    //         .Anonymous
    //         .Texture2D
    //         .MipLevels = cursor_texture_desc.MipLevels;

    //     let cursor_left = dxgi_outdupl_frame_info.PointerPosition.Position.x as u32;
    //     let cursor_top = dxgi_outdupl_frame_info.PointerPosition.Position.y as u32;

    //     info!(
    //         "width: {}, height: {}",
    //         cursor_shape_info.Width, cursor_shape_info.Height
    //     );
    //     let cursor_width = cursor_shape_info.Width;
    //     let mut cursor_height = cursor_shape_info.Height;

    //     // Used for copying pixels
    //     let mut cursor_box: D3D11_BOX = zeroed();
    //     cursor_box.front = 0;
    //     cursor_box.back = 1;

    //     if cursor_shape_info.Type == DXGI_OUTDUPL_POINTER_SHAPE_TYPE_MONOCHROME.0 as u32 {
    //         cursor_height /= 2;

    //         self.process_mono_mask(
    //             true,
    //             cursor_left,
    //             cursor_top,
    //             cursor_width,
    //             cursor_height,
    //             cursor_shape_info.Pitch,
    //             &cursor_shape_buffer,
    //             &mut cursor_box,
    //             &mut init_buffer,
    //         )?;
    //     } else if cursor_shape_info.Type == DXGI_OUTDUPL_POINTER_SHAPE_TYPE_MASKED_COLOR.0 as u32 {
    //         self.process_mono_mask(
    //             false,
    //             cursor_left,
    //             cursor_top,
    //             cursor_width,
    //             cursor_height,
    //             cursor_shape_info.Pitch,
    //             &cursor_shape_buffer,
    //             &mut cursor_box,
    //             &mut init_buffer,
    //         )?;
    //     }

    //     // Position will be changed based on mouse position
    //     let mut vertices = VERTICES.clone();

    //     vertices[0].pos.x = (cursor_left as f32 - center_x) / center_x;
    //     vertices[0].pos.y = -1f32 * ((cursor_top + cursor_height) as f32 - center_y) / center_y;

    //     vertices[1].pos.x = (cursor_left as f32 - center_x) / center_x;
    //     vertices[1].pos.y = -1f32 * (cursor_top as f32 - center_y) / center_y;

    //     vertices[2].pos.x = ((cursor_left + cursor_width) as f32 - center_x) / center_x;
    //     vertices[2].pos.y = -1f32 * ((cursor_top + cursor_height) as f32 - center_y) / center_y;

    //     vertices[3].pos.x = vertices[2].pos.x;
    //     vertices[3].pos.y = vertices[2].pos.y;

    //     vertices[4].pos.x = vertices[1].pos.x;
    //     vertices[4].pos.y = vertices[1].pos.y;

    //     vertices[5].pos.x = ((cursor_left + cursor_width) as f32 - center_x) / center_x;
    //     vertices[5].pos.y = -1f32 * (cursor_top as f32 - center_y) / center_y;

    //     // Set texture properties
    //     cursor_texture_desc.Width = cursor_width;
    //     cursor_texture_desc.Height = cursor_height;

    //     // Set up init data
    //     let mut init_data: D3D11_SUBRESOURCE_DATA = zeroed();

    //     init_data.pSysMem =
    //         if cursor_shape_info.Type == DXGI_OUTDUPL_POINTER_SHAPE_TYPE_COLOR.0 as u32 {
    //             cursor_shape_buffer.as_ptr() as *const c_void
    //         } else {
    //             &init_buffer as *const _ as *const c_void
    //         };

    //     init_data.SysMemPitch =
    //         if cursor_shape_info.Type == DXGI_OUTDUPL_POINTER_SHAPE_TYPE_COLOR.0 as u32 {
    //             cursor_shape_info.Pitch
    //         } else {
    //             info!("cursor_width: {}", cursor_width);
    //             cursor_width * BPP
    //         };

    //     init_data.SysMemSlicePitch = 0;

    //     // Create mouseshape as texture
    //     let mouse_tex = self
    //         .dx
    //         .device()
    //         .CreateTexture2D(&cursor_texture_desc, &init_data)
    //         .map_err(|err| anyhow::anyhow!("ID3D11Device::CreateTexture2D failed ({})", err))?;

    //     // Create shader resource from texture
    //     let shader_res = self
    //         .dx
    //         .device()
    //         .CreateShaderResourceView(mouse_tex, &cursor_shader_resource_view_desc)
    //         .map_err(|err| {
    //             anyhow::anyhow!("ID3D11Device::CreateShaderResourceView failed ({})", err)
    //         })?;

    //     let mut buffer_desc: D3D11_BUFFER_DESC = zeroed();
    //     buffer_desc.Usage = D3D11_USAGE_DEFAULT;
    //     buffer_desc.ByteWidth = (std::mem::size_of::<VERTEX>() * vertices.len()) as u32;
    //     buffer_desc.BindFlags = D3D11_BIND_VERTEX_BUFFER.0;
    //     buffer_desc.CPUAccessFlags = 0;

    //     init_data = zeroed();
    //     init_data.pSysMem = vertices.as_ptr() as *const c_void;

    //     // Create vertex buffer
    //     let mouse_vertex_buffer = self
    //         .dx
    //         .device()
    //         .CreateBuffer(&buffer_desc, &init_data)
    //         .map_err(|err| anyhow::anyhow!("ID3D11Device::CreateBuffer failed ({})", err))?;

    //     // Set resources
    //     let blend_factor = [0f32; 4];
    //     let stride = std::mem::size_of::<VERTEX>() as u32;
    //     let offset = 0;

    //     self.dx.device_context().IASetVertexBuffers(
    //         0,
    //         1,
    //         [Some(mouse_vertex_buffer)].as_ptr(),
    //         &stride,
    //         &offset,
    //     );

    //     self.dx.device_context().OMSetBlendState(
    //         &self.blend_state,
    //         blend_factor.as_ptr(),
    //         0xFFFFFFFF,
    //     );

    //     self.dx
    //         .device_context()
    //         .OMSetRenderTargets(&[Some(self.backend_render_target_view.clone())], None);

    //     self.dx
    //         .device_context()
    //         .VSSetShader(self.dx.vertex_shader(), &[]);

    //     self.dx
    //         .device_context()
    //         .PSSetShader(self.dx.pixel_shader(), &[]);

    //     self.dx
    //         .device_context()
    //         .PSSetShaderResources(0, &[Some(shader_res)]);

    //     self.dx
    //         .device_context()
    //         .PSSetSamplers(0, self.sampler_linear.as_ref());

    //     self.dx.device_context().Draw(vertices.len() as u32, 0);

    //     Ok(())
    // }

    // unsafe fn process_mono_mask(
    //     &self,
    //     is_mono: bool,
    //     cursor_position_x: u32,
    //     cursor_position_y: u32,
    //     cursor_width: u32,
    //     cursor_height: u32,
    //     cursor_shape_pitch: u32,
    //     cursor_shape_buffer: &[u8],
    //     cursor_box: &mut D3D11_BOX,
    //     init_buffer: &mut Vec<u8>,
    // ) -> anyhow::Result<()> {
    //     let mut copy_buffer_desc: D3D11_TEXTURE2D_DESC = zeroed();
    //     copy_buffer_desc.Width = cursor_width;
    //     copy_buffer_desc.Height = cursor_height;
    //     copy_buffer_desc.MipLevels = 1;
    //     copy_buffer_desc.ArraySize = 1;
    //     copy_buffer_desc.Format = DXGI_FORMAT_B8G8R8A8_UNORM;
    //     copy_buffer_desc.SampleDesc.Count = 1;
    //     copy_buffer_desc.SampleDesc.Quality = 0;
    //     copy_buffer_desc.Usage = D3D11_USAGE_STAGING;
    //     copy_buffer_desc.BindFlags = D3D11_BIND_FLAG::default();
    //     copy_buffer_desc.CPUAccessFlags = D3D11_CPU_ACCESS_READ;
    //     copy_buffer_desc.MiscFlags = D3D11_RESOURCE_MISC_FLAG::default();

    //     let copy_buffer = self
    //         .dx
    //         .device()
    //         .CreateTexture2D(&copy_buffer_desc, null())
    //         .map_err(|err| anyhow::anyhow!("ID3D11Device::CreateTexture2D failed ({})", err))?;

    //     cursor_box.left = cursor_position_x;
    //     cursor_box.top = cursor_position_y;
    //     cursor_box.right = cursor_position_x + cursor_width;
    //     cursor_box.bottom = cursor_position_y + cursor_height;

    //     self.dx.device_context().CopySubresourceRegion(
    //         &copy_buffer,
    //         0,
    //         0,
    //         0,
    //         0,
    //         &self.backend_texture,
    //         0,
    //         cursor_box,
    //     );

    //     let copy_surface: IDXGISurface = copy_buffer.cast().map_err(|err| anyhow::anyhow!(err))?;

    //     let mut mapped_surface: DXGI_MAPPED_RECT = zeroed();

    //     copy_surface
    //         .Map(&mut mapped_surface, DXGI_MAP_READ)
    //         .map_err(|err| anyhow::anyhow!("IDXGISurface::Map failed ({})", err))?;

    //     init_buffer.resize((cursor_width * cursor_height * BPP) as usize, 0);

    //     let init_buffer_32 = init_buffer.as_mut_ptr() as *mut _ as *mut u32;
    //     let desktop_32 = mapped_surface.pBits as *mut _ as *mut u32;
    //     let desktop_pitch_in_pixels = mapped_surface.Pitch as u32 / 4;

    //     if is_mono {
    //         for x in 0..cursor_height {
    //             let mut mask = (0x80 >> 8) as u8;

    //             for y in 0..cursor_width {
    //                 let and_mask =
    //                     cursor_shape_buffer[(y / 8 + x * cursor_shape_pitch) as usize] & mask;

    //                 let xor_mask = cursor_shape_buffer
    //                     [(y / 8 + (x + cursor_width) * cursor_shape_pitch) as usize]
    //                     & mask;

    //                 let and_mask_32: u32 = if and_mask > 0 { 0xFFFFFFFF } else { 0xFF000000 };

    //                 let xor_mask_32: u32 = if xor_mask > 0 { 0x00FFFFFF } else { 0x00000000 };

    //                 let mut val = *desktop_32.add((x * desktop_pitch_in_pixels + y) as usize);
    //                 val &= and_mask_32;
    //                 val ^= xor_mask_32;

    //                 *init_buffer_32.add((x * cursor_width + y) as usize) = val;

    //                 if mask == 0x01 {
    //                     mask = 0x80;
    //                 } else {
    //                     mask = mask >> 1;
    //                 }
    //             }
    //         }
    //     } else {
    //         let buffer_32: &[u32] = std::mem::transmute(cursor_shape_buffer);

    //         for x in 0..cursor_height {
    //             for y in 0..cursor_width {
    //                 let mask_val: u32 =
    //                     0xFF000000 & buffer_32[(y + x * cursor_shape_pitch / 4) as usize];

    //                 if mask_val > 0 {
    //                     // Mask was 0xFF
    //                     let mut val = *desktop_32.add((x * desktop_pitch_in_pixels + y) as usize);
    //                     val ^= buffer_32[(y + x * cursor_shape_pitch / 4) as usize];
    //                     val |= 0xFF000000;

    //                     *init_buffer_32.add((x * cursor_width + y) as usize) = val;
    //                 } else {
    //                     // Mask was 0x00
    //                     let mut val = buffer_32[(y + x * cursor_shape_pitch / 4) as usize];
    //                     val |= 0xFF000000;

    //                     *init_buffer_32.add((x * cursor_width + y) as usize) = val;
    //                 }
    //             }
    //         }
    //     }

    //     copy_surface
    //         .Unmap()
    //         .map_err(|err| anyhow::anyhow!("IDXGISurface::Unmap failed ({})", err))?;

    //     Ok(())
    // }
}

unsafe fn init_output_duplication(
    dx: &DX,
    monitor_id: &str,
) -> Result<(DXGI_OUTPUT_DESC, IDXGIOutputDuplication), MirrorXError> {
    let dxgi_device: IDXGIDevice = dx.device().cast().map_err(|err| {
        MirrorXError::Other(anyhow::anyhow!(
            "ID3D11Device::QueryInterface for IDXGIDevice failed ({})",
            err
        ))
    })?;

    let dxgi_adapter = dxgi_device.GetParent::<IDXGIAdapter>().map_err(|err| {
        MirrorXError::Other(anyhow::anyhow!(
            "IDXGIDevice::GetParent for IDXGIAdapter failed ({})",
            err
        ))
    })?;

    let adapter_desc = dxgi_adapter.GetDesc().map_err(|err| {
        MirrorXError::Other(anyhow::anyhow!("IDXGIAdapter::GetDesc failed ({})", err))
    })?;

    info!("{:?}", &adapter_desc.AdapterLuid);
    info!("{:?}", OsString::from_wide_null(&adapter_desc.Description));

    let mut output_index = 0;

    while let Ok(dxgi_output) = dxgi_adapter.EnumOutputs(output_index) {
        output_index += 1;

        let dxgi_output_desc = dxgi_output.GetDesc().map_err(|err| {
            MirrorXError::Other(anyhow::anyhow!("IDXGIOutput::GetDesc failed ({})", err))
        })?;

        if !dxgi_output_desc.AttachedToDesktop.as_bool() {
            continue;
        }

        let mut dev_index = 0u32;
        loop {
            let origin_device_name = OsString::from_wide_null(&dxgi_output_desc.DeviceName);

            let mut display_device: DISPLAY_DEVICEW = std::mem::zeroed();
            display_device.cb = std::mem::size_of::<DISPLAY_DEVICEW>() as u32;

            let success = EnumDisplayDevicesW(
                &*origin_device_name,
                dev_index,
                &mut display_device as *mut _,
                EDD_GET_DEVICE_INTERFACE_NAME,
            )
            .as_bool();

            dev_index += 1;

            if !success {
                break;
            }

            if (display_device.StateFlags & DISPLAY_DEVICE_ATTACHED_TO_DESKTOP) != 0 {
                let device_id = OsString::from_wide_null(&display_device.DeviceID)
                    .into_string()
                    .map_err(|_| {
                        MirrorXError::Other(anyhow::anyhow!("convert OsString to String failed"))
                    })?;

                if device_id == monitor_id {
                    let dxgi_output1: IDXGIOutput1 = dxgi_output.cast().map_err(|err| {
                        MirrorXError::Other(anyhow::anyhow!(
                            "IDXGIOutput::QueryInterface for IDXGIOutput1 failed ({})",
                            err
                        ))
                    })?;

                    let dxgi_output_duplication =
                        dxgi_output1.DuplicateOutput(dx.device()).map_err(|err| {
                            MirrorXError::Other(anyhow::anyhow!(
                                "Duplication: IDXGIOutput1::DuplicateOutput failed ({})",
                                err
                            ))
                        })?;

                    return Ok((dxgi_output_desc, dxgi_output_duplication));
                }
            }
        }
    }

    Err(MirrorXError::Other(anyhow::anyhow!(
        "init duplication failed"
    )))
}
