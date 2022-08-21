use super::{dx::DX, dx_math::VERTICES};
use crate::{
    check_if_failed,
    component::{
        capture_frame::CaptureFrame,
        desktop::windows::dx_math::{BPP, VERTEX},
    },
    error::{CoreResult, MirrorXError},
    utility::wide_char::FromWide,
};
use anyhow::bail;
use scopeguard::defer;
use std::{ffi::OsString, mem::zeroed, os::raw::c_void, ptr::null};
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
    staging_backend_texture: ID3D11Texture2D,
    view_port_backend: D3D11_VIEWPORT,
    render_target_view_backend: ID3D11RenderTargetView,
    // backend_render_target_view: ID3D11RenderTargetView,
    // render_texture_lumina: ID3D11Texture2D,
    // render_texture_chrominance: ID3D11Texture2D,
    // staging_texture_lumina: ID3D11Texture2D,
    // staging_texture_chrominance: ID3D11Texture2D,
    // view_port_lumina: D3D11_VIEWPORT,
    // view_port_chrominance: D3D11_VIEWPORT,
    // render_target_view_lumina: ID3D11RenderTargetView,
    // render_target_view_chrominance: ID3D11RenderTargetView,
    sampler_linear: [Option<ID3D11SamplerState>; 1],
    blend_state: ID3D11BlendState,
    keyed_mutex: IDXGIKeyedMutex,
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

            let mut texture_desc: D3D11_TEXTURE2D_DESC = std::mem::zeroed();
            texture_desc.Width = dxgi_outdupl_desc.ModeDesc.Width;
            texture_desc.Height = dxgi_outdupl_desc.ModeDesc.Height;
            texture_desc.MipLevels = 1;
            texture_desc.ArraySize = 1;
            texture_desc.Format = dxgi_outdupl_desc.ModeDesc.Format;
            texture_desc.SampleDesc.Count = 1;
            texture_desc.Usage = D3D11_USAGE_DEFAULT;
            texture_desc.BindFlags = D3D11_BIND_SHADER_RESOURCE | D3D11_BIND_RENDER_TARGET;
            texture_desc.CPUAccessFlags = D3D11_CPU_ACCESS_FLAG::default();
            texture_desc.MiscFlags = D3D11_RESOURCE_MISC_SHARED_KEYEDMUTEX;

            let backend_texture =
                check_if_failed!(dx.device().CreateTexture2D(&texture_desc, null()));

            let keyed_mutex: IDXGIKeyedMutex = check_if_failed!(backend_texture.cast());

            texture_desc.CPUAccessFlags = D3D11_CPU_ACCESS_READ;
            texture_desc.Usage = D3D11_USAGE_STAGING;
            texture_desc.BindFlags = D3D11_BIND_FLAG(0);
            texture_desc.MiscFlags = D3D11_RESOURCE_MISC_FLAG::default();
            let staging_backend_texture =
                check_if_failed!(dx.device().CreateTexture2D(&texture_desc, null()));

            let view_port_backend = D3D11_VIEWPORT {
                TopLeftX: 0.0,
                TopLeftY: 0.0,
                Width: texture_desc.Width as f32,
                Height: texture_desc.Height as f32,
                MinDepth: 0.0,
                MaxDepth: 1.0,
            };

            let render_target_view_backend =
                check_if_failed!(dx.device().CreateRenderTargetView(&backend_texture, null()));

            // let backend_render_target_view = dx.device().CreateRenderTargetView(&backend_texture,null()).map_err(|err|{
            //         anyhow::anyhow!(
            //             r#"Duplication: ID3D11Device::CreateRenderTargetView failed {{"texture_name":"{}", "error": "{:?}"}}"#,
            //             "backend_texture",
            //             err.code()
            //         )
            //     })?;

            // create lumina plane resource

            // texture_desc.Format = DXGI_FORMAT_R8_UNORM;
            // texture_desc.BindFlags = D3D11_BIND_RENDER_TARGET;
            // texture_desc.MiscFlags = D3D11_RESOURCE_MISC_FLAG::default();
            // let render_texture_lumina = dx. device()
            //     .CreateTexture2D(&texture_desc, null())
            //     .map_err(|err| {
            //         anyhow::anyhow!(
            //             r#"Duplication: ID3D11Device::CreateTexture2D failed {{"texture_name":"{}", "error": "{:?}"}}"#,
            //             "render_texture_lumina",
            //             err.code()
            //         )
            //     })?;

            // texture_desc.CPUAccessFlags = D3D11_CPU_ACCESS_READ;
            // texture_desc.Usage = D3D11_USAGE_STAGING;
            // texture_desc.BindFlags = D3D11_BIND_FLAG(0);
            // let staging_texture_lumina = dx.device().CreateTexture2D(&texture_desc, null()).map_err(|err|{
            //     anyhow::anyhow!(
            //         r#"Duplication: ID3D11Device::CreateTexture2D failed {{"texture_name":"{}", "error": "{:?}"}}"#,
            //         "staging_texture_lumina",
            //         err.code()
            //     )
            // })?;

            // let view_port_lumina = D3D11_VIEWPORT {
            //     TopLeftX: 0.0,
            //     TopLeftY: 0.0,
            //     Width: texture_desc.Width as f32,
            //     Height: texture_desc.Height as f32,
            //     MinDepth: 0.0,
            //     MaxDepth: 1.0,
            // };

            // let render_target_view_lumina = dx.device().CreateRenderTargetView(&render_texture_lumina,null()).map_err(|err|{
            //     anyhow::anyhow!(
            //         r#"Duplication: ID3D11Device::CreateRenderTargetView failed {{"texture_name":"{}", "error": "{:?}"}}"#,
            //         "render_texture_lumina",
            //         err.code()
            //     )
            // })?;

            // // create chrominance plane resource

            // texture_desc.Width = dxgi_outdupl_desc.ModeDesc.Width / 2;
            // texture_desc.Height = dxgi_outdupl_desc.ModeDesc.Height / 2;
            // texture_desc.Format = DXGI_FORMAT_R8G8_UNORM;
            // texture_desc.Usage = D3D11_USAGE_DEFAULT;
            // texture_desc.CPUAccessFlags = D3D11_CPU_ACCESS_FLAG(0);
            // texture_desc.BindFlags = D3D11_BIND_RENDER_TARGET;

            // let render_texture_chrominance = dx.device()
            //     .CreateTexture2D(&texture_desc, null())
            //     .map_err(|err| {
            //         anyhow::anyhow!(
            //             r#"Duplication: ID3D11Device::CreateTexture2D failed {{"texture_name":"{}", "error": "{:?}"}}"#,
            //             "render_texture_chrominance",
            //             err.code()
            //         )
            //     })?;

            // texture_desc.CPUAccessFlags = D3D11_CPU_ACCESS_READ;
            // texture_desc.Usage = D3D11_USAGE_STAGING;
            // texture_desc.BindFlags = D3D11_BIND_FLAG(0);
            // let staging_texture_chrominance =dx. device().CreateTexture2D(&texture_desc, null()).map_err(|err|{
            //     anyhow::anyhow!(
            //         r#"Duplication: ID3D11Device::CreateTexture2D failed {{"texture_name":"{}", "error": "{:?}"}}"#,
            //         "staging_texture_chrominance",
            //         err.code()
            //     )
            // })?;

            // let view_port_chrominance = D3D11_VIEWPORT {
            //     TopLeftX: 0.0,
            //     TopLeftY: 0.0,
            //     Width: texture_desc.Width as f32,
            //     Height: texture_desc.Height as f32,
            //     MinDepth: 0.0,
            //     MaxDepth: 1.0,
            // };

            // let render_target_view_chrominance = dx.device().CreateRenderTargetView(&render_texture_chrominance,null()).map_err(|err|{
            //     anyhow::anyhow!(
            //         r#"Duplication: ID3D11Device::CreateRenderTargetView failed {{"texture_name":"{}", "error": "{:?}"}}"#,
            //         "render_texture_chrominance",
            //         err.code()
            //     )
            // })?;

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
                staging_backend_texture,
                view_port_backend,
                render_target_view_backend,
                // backend_render_target_view,
                // render_texture_lumina,
                // render_texture_chrominance,
                // staging_texture_lumina,
                // staging_texture_chrominance,
                // view_port_lumina,
                // view_port_chrominance,
                // render_target_view_lumina,
                // render_target_view_chrominance,
                sampler_linear: [Some(sampler_linear); 1],
                blend_state,
                keyed_mutex,
            })
        }
    }

    pub fn capture(&mut self) -> anyhow::Result<CaptureFrame> {
        unsafe {
            check_if_failed!(self.keyed_mutex.AcquireSync(1, 100));

            self.acquire_frame()?;
            // self.process_frame()?;

            self.dx
                .device_context()
                .CopyResource(&self.staging_backend_texture, &self.backend_texture);

            let mapped_resource_backend = check_if_failed!(self.dx.device_context().Map(
                &self.staging_backend_texture,
                0,
                D3D11_MAP_READ,
                0
            ));

            defer! {
                self.dx
                .device_context()
                .Unmap(&self.staging_backend_texture, 0);
            }

            // self.dx
            //     .device_context()
            //     .CopyResource(&self.staging_texture_lumina, &self.render_texture_lumina);

            // self.dx.device_context().CopyResource(
            //     &self.staging_texture_chrominance,
            //     &self.render_texture_chrominance,
            // );

            // let mapped_resource_lumina = self.dx.device_context().Map(&self.staging_texture_lumina, 0, D3D11_MAP_READ, 0).map_err(|err|{
            //     anyhow::anyhow!(
            //         r#"Duplication: ID3D11DeviceContext::Map failed {{"resource_name": "{}", "error": "{:?}"}}"#,
            //         "staging_texture_lumina",
            //         err.code()
            //     )
            // })?;

            // defer! {
            //     self.dx
            //     .device_context()
            //     .Unmap(&self.staging_texture_lumina, 0);
            // }

            // let mapped_resource_chrominance = self.dx.device_context().Map(&self.staging_texture_chrominance, 0, D3D11_MAP_READ, 0).map_err(|err|{
            //     anyhow::anyhow!(
            //         r#"Duplication: ID3D11DeviceContext::Map failed {{"resource_name": "{}", "error": "{:?}"}}"#,
            //         "staging_texture_chrominance",
            //         err.code()
            //     )
            // })?;

            // defer! {
            //     self.dx
            //     .device_context()
            //     .Unmap(&self.staging_texture_chrominance, 0);
            // }

            let width = self.output_desc.DesktopCoordinates.right
                - self.output_desc.DesktopCoordinates.left;

            let height = self.output_desc.DesktopCoordinates.bottom
                - self.output_desc.DesktopCoordinates.top;

            let buffer_size = (height as u32) * mapped_resource_backend.RowPitch;

            // let luminance_buffer_size = (height as u32) * mapped_resource_lumina.RowPitch;
            // let chrominance_buffer_size =
            //     (height as u32) / 2 * mapped_resource_chrominance.RowPitch;

            let capture_frame = CaptureFrame {
                width: width as u16,
                height: height as u16,
                bytes: std::slice::from_raw_parts(
                    mapped_resource_backend.pData as *mut u8,
                    buffer_size as usize,
                )
                .to_vec(),
                stride: mapped_resource_backend.RowPitch as u16,
            };

            check_if_failed!(self.keyed_mutex.ReleaseSync(0));
zhux
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

        check_if_failed!(self.output_duplication.ReleaseFrame());

        // let backend_surface: IDXGISurface1 = self
        //     .backend_texture
        //     .cast()
        //     .map_err(|err| anyhow::anyhow!(err))?;

        // let mut cursor_info: CURSORINFO = zeroed();
        // cursor_info.cbSize = std::mem::size_of::<CURSORINFO>() as u32;

        // if GetCursorInfo(&mut cursor_info).as_bool() {
        //     if cursor_info.flags == CURSOR_SHOWING {
        //         let cursorPosition = cursor_info.ptScreenPos;
        //         let lCursorSize = cursor_info.cbSize;

        //         let hdc = backend_surface
        //             .GetDC(false)
        //             .map_err(|err| anyhow::anyhow!(err))?;

        //         DrawIconEx(
        //             hdc,
        //             cursorPosition.x,
        //             cursorPosition.y,
        //             cursor_info.hCursor,
        //             0,
        //             0,
        //             0,
        //             None,
        //             DI_NORMAL | DI_DEFAULTSIZE,
        //         );

        //         backend_surface.ReleaseDC(null());
        //     }
        // }

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
            .OMSetRenderTargets(&[Some(self.render_target_view_backend.clone())], None);
        self.dx
            .device_context()
            .PSSetShader(self.dx.pixel_shader(), &[]);
        self.dx
            .device_context()
            .RSSetViewports(&[self.view_port_backend]);
        self.dx.device_context().Draw(VERTICES.len() as u32, 0);

        // draw chrominance plane
        // self.dx
        //     .device_context()
        //     .OMSetRenderTargets(&[Some(self.render_target_view_chrominance.clone())], None);
        // self.dx
        //     .device_context()
        //     .PSSetShader(self.dx.pixel_shader_chrominance(), &[]);
        // self.dx
        //     .device_context()
        //     .RSSetViewports(&[self.view_port_chrominance]);
        // self.dx.device_context().Draw(VERTICES.len() as u32, 0);

        Ok(())
    }

    unsafe fn draw_mouse(
        &self,
        dxgi_outdupl_frame_info: &DXGI_OUTDUPL_FRAME_INFO,
    ) -> anyhow::Result<()> {
        let mut cursor_shape_buffer =
            Vec::<u8>::with_capacity(dxgi_outdupl_frame_info.PointerShapeBufferSize as usize);
        let mut cursor_shape_buffer_length = 0u32;
        let mut cursor_shape_info: DXGI_OUTDUPL_POINTER_SHAPE_INFO = zeroed();

        check_if_failed!(self.output_duplication.GetFramePointerShape(
            dxgi_outdupl_frame_info.PointerShapeBufferSize,
            cursor_shape_buffer.as_mut_ptr() as *mut _,
            &mut cursor_shape_buffer_length,
            &mut cursor_shape_info,
        ));

        cursor_shape_buffer.set_len(dxgi_outdupl_frame_info.PointerShapeBufferSize as usize);

        let mut full_desc: D3D11_TEXTURE2D_DESC = std::mem::zeroed();
        self.backend_texture.GetDesc(&mut full_desc);

        let desktop_width = full_desc.Width;
        let desktop_height = full_desc.Height;

        let center_x = (desktop_width / 2) as i32;
        let center_y = (desktop_height / 2) as i32;

        let (mut pointer_width, mut pointer_height, mut pointer_left, mut pointer_top) =
            (0i32, 0i32, 0i32, 0i32);

        let mut d3d_box: D3D11_BOX = std::mem::zeroed();
        d3d_box.front = 0;
        d3d_box.back = 1;

        let mut desc: D3D11_TEXTURE2D_DESC = std::mem::zeroed();
        desc.MipLevels = 1;
        desc.ArraySize = 1;
        desc.Format = DXGI_FORMAT_B8G8R8A8_UNORM;
        desc.SampleDesc.Count = 1;
        desc.SampleDesc.Quality = 0;
        desc.Usage = D3D11_USAGE_DEFAULT;
        desc.BindFlags = D3D11_BIND_SHADER_RESOURCE;
        desc.CPUAccessFlags = D3D11_CPU_ACCESS_FLAG::default();
        desc.MiscFlags = D3D11_RESOURCE_MISC_FLAG::default();

        let mut s_desc: D3D11_SHADER_RESOURCE_VIEW_DESC = std::mem::zeroed();
        s_desc.Format = desc.Format;
        s_desc.ViewDimension = D3D11_SRV_DIMENSION_TEXTURE2D;
        s_desc.Anonymous.Texture2D.MostDetailedMip = desc.MipLevels - 1;
        s_desc.Anonymous.Texture2D.MipLevels = desc.MipLevels;

        let mut init_buffer = std::ptr::null();

        match DXGI_OUTDUPL_POINTER_SHAPE_TYPE(cursor_shape_info.Type as i32) {
            DXGI_OUTDUPL_POINTER_SHAPE_TYPE_COLOR => {
                pointer_left = dxgi_outdupl_frame_info.PointerPosition.Position.x as i32;
                pointer_top = dxgi_outdupl_frame_info.PointerPosition.Position.y as i32;
                pointer_width = cursor_shape_info.Width as i32;
                pointer_height = cursor_shape_info.Height as i32;
            }
            DXGI_OUTDUPL_POINTER_SHAPE_TYPE_MONOCHROME => {
                let buffer = self.process_mono_mask(
                    true,
                    &mut cursor_shape_info,
                    dxgi_outdupl_frame_info,
                    &mut cursor_shape_buffer,
                    &mut pointer_width,
                    &mut pointer_height,
                    &mut pointer_left,
                    &mut pointer_top,
                    &mut d3d_box,
                )?;

                init_buffer = buffer.as_ptr()
            }
            DXGI_OUTDUPL_POINTER_SHAPE_TYPE_MASKED_COLOR => {
                let buffer = self.process_mono_mask(
                    false,
                    &mut cursor_shape_info,
                    dxgi_outdupl_frame_info,
                    &mut cursor_shape_buffer,
                    &mut pointer_width,
                    &mut pointer_height,
                    &mut pointer_left,
                    &mut pointer_top,
                    &mut d3d_box,
                )?;

                init_buffer = buffer.as_ptr()
            }
            _ => {}
        };

        let mut vertices = VERTICES.clone();
        vertices[0].pos.x = (pointer_left - center_x) as f32 / center_x as f32;
        vertices[0].pos.y =
            -1f32 * (pointer_top + pointer_height - center_y) as f32 / center_y as f32;

        vertices[1].pos.x = (pointer_left - center_x) as f32 / center_x as f32;
        vertices[1].pos.y = -1f32 * (pointer_top - center_y) as f32 / center_y as f32;

        vertices[2].pos.x = (pointer_left + pointer_width - center_x) as f32 / center_x as f32;
        vertices[2].pos.y =
            -1f32 * (pointer_top + pointer_height - center_y) as f32 / center_y as f32;

        vertices[3].pos.x = vertices[2].pos.x;
        vertices[3].pos.y = vertices[2].pos.y;

        vertices[4].pos.x = vertices[1].pos.x;
        vertices[4].pos.y = vertices[1].pos.y;

        vertices[5].pos.x = (pointer_left + pointer_width - center_x) as f32 / center_x as f32;
        vertices[5].pos.y = -1f32 * (pointer_top - center_y) as f32 / center_y as f32;

        desc.Width = pointer_width as u32;
        desc.Height = pointer_height as u32;

        let mut init_data: D3D11_SUBRESOURCE_DATA = std::mem::zeroed();
        init_data.pSysMem =
            if cursor_shape_info.Type == DXGI_OUTDUPL_POINTER_SHAPE_TYPE_COLOR.0 as u32 {
                cursor_shape_buffer.as_ptr() as *const _
            } else {
                init_buffer as *const _
            };
        init_data.SysMemPitch =
            if cursor_shape_info.Type == DXGI_OUTDUPL_POINTER_SHAPE_TYPE_COLOR.0 as u32 {
                cursor_shape_info.Pitch
            } else {
                pointer_width as u32 * BPP
            };
        init_data.SysMemSlicePitch = 0;

        let mouse_texture = check_if_failed!(self.dx.device().CreateTexture2D(&desc, &init_data));

        let shader_res = check_if_failed!(self
            .dx
            .device()
            .CreateShaderResourceView(&mouse_texture, &s_desc));

        let mut b_desc: D3D11_BUFFER_DESC = std::mem::zeroed();
        b_desc.Usage = D3D11_USAGE_DEFAULT;
        b_desc.ByteWidth = (std::mem::size_of::<VERTEX>() * VERTICES.len()) as u32;
        b_desc.BindFlags = D3D11_BIND_VERTEX_BUFFER.0;
        b_desc.CPUAccessFlags = 0;

        init_data = std::mem::zeroed();
        init_data.pSysMem = vertices.as_ptr() as *const _;

        let vertex_buffer = Some(check_if_failed!(self
            .dx
            .device()
            .CreateBuffer(&b_desc, &init_data)));

        let blend_factor = [0f32; 4];
        let stride = std::mem::size_of::<VERTEX>() as u32;
        let offset = 0;

        self.dx.device_context().IASetVertexBuffers(
            0,
            1,
            [vertex_buffer].as_ptr(),
            &stride,
            &offset,
        );

        self.dx.device_context().OMSetBlendState(
            &self.blend_state,
            blend_factor.as_ptr(),
            0xFFFFFFFF,
        );

        self.dx
            .device_context()
            .OMSetRenderTargets(&[Some(self.render_target_view_backend.clone())], None);

        self.dx
            .device_context()
            .VSSetShader(self.dx.vertex_shader(), &[]);

        self.dx
            .device_context()
            .PSSetShader(self.dx.pixel_shader(), &[]);

        self.dx
            .device_context()
            .PSSetShaderResources(0, &[Some(shader_res)]);

        self.dx
            .device_context()
            .PSSetSamplers(0, &self.sampler_linear);

        self.dx.device_context().Draw(VERTICES.len() as u32, 0);

        Ok(())
    }

    unsafe fn process_mono_mask(
        &self,
        is_mono: bool,
        pointer_info: &mut DXGI_OUTDUPL_POINTER_SHAPE_INFO,
        frame_info: &DXGI_OUTDUPL_FRAME_INFO,
        pointer_shape_buffer: &mut [u8],
        pointer_width: &mut i32,
        pointer_height: &mut i32,
        pointer_left: &mut i32,
        pointer_top: &mut i32,
        d3d_box: &mut D3D11_BOX,
    ) -> CoreResult<Vec<u8>> {
        let mut full_desc: D3D11_TEXTURE2D_DESC = std::mem::zeroed();
        self.backend_texture.GetDesc(&mut full_desc);

        let desktop_width = full_desc.Width as i32;
        let desktop_height = full_desc.Height as i32;

        let given_left = frame_info.PointerPosition.Position.x;
        let given_top = frame_info.PointerPosition.Position.y;

        if given_left < 0 {
            *pointer_width = given_left + pointer_info.Width as i32;
        } else if given_left + (pointer_info.Width as i32) > desktop_width {
            *pointer_width = desktop_width - given_left;
        } else {
            *pointer_width = pointer_info.Width as i32;
        }

        if is_mono {
            pointer_info.Height /= 2;
        }

        if given_top < 0 {
            *pointer_height = given_top + pointer_info.Height as i32;
        } else if given_top + (pointer_info.Height as i32) > desktop_height {
            *pointer_height = desktop_height - given_top;
        } else {
            *pointer_height = pointer_info.Height as i32;
        }

        if is_mono {
            pointer_info.Height *= 2;
        }

        *pointer_left = given_left.max(0);
        *pointer_top = given_top.max(0);

        let mut copy_buffer_desc: D3D11_TEXTURE2D_DESC = std::mem::zeroed();
        copy_buffer_desc.Width = *pointer_width as u32;
        copy_buffer_desc.Height = *pointer_height as u32;
        copy_buffer_desc.MipLevels = 1;
        copy_buffer_desc.ArraySize = 1;
        copy_buffer_desc.Format = DXGI_FORMAT_B8G8R8A8_UNORM;
        copy_buffer_desc.SampleDesc.Count = 1;
        copy_buffer_desc.SampleDesc.Quality = 0;
        copy_buffer_desc.Usage = D3D11_USAGE_STAGING;
        copy_buffer_desc.BindFlags = D3D11_BIND_FLAG::default();
        copy_buffer_desc.CPUAccessFlags = D3D11_CPU_ACCESS_READ;
        copy_buffer_desc.MiscFlags = D3D11_RESOURCE_MISC_FLAG::default();

        let copy_buffer = check_if_failed!(self
            .dx
            .device()
            .CreateTexture2D(&copy_buffer_desc, std::ptr::null()));

        (*d3d_box).left = *pointer_left as u32;
        (*d3d_box).top = *pointer_top as u32;
        (*d3d_box).right = (*pointer_left + *pointer_width) as u32;
        (*d3d_box).bottom = (*pointer_top + *pointer_height) as u32;

        self.dx.device_context().CopySubresourceRegion(
            &copy_buffer,
            0,
            0,
            0,
            0,
            &self.backend_texture,
            0,
            d3d_box,
        );

        let copy_surface: IDXGISurface = check_if_failed!(copy_buffer.cast());

        let mut mapped_surface: DXGI_MAPPED_RECT = std::mem::zeroed();
        check_if_failed!(copy_surface.Map(&mut mapped_surface, DXGI_MAP_READ));

        let mut init_buffer = Vec::<u8>::new();
        init_buffer.resize(
            ((*pointer_width as u32) * (*pointer_height as u32) * BPP) as usize,
            0,
        );

        let init_buffer_32: *mut u32 = std::mem::transmute(init_buffer.as_mut_ptr());
        let desktop_32: *mut u32 = std::mem::transmute(mapped_surface.pBits);
        let desktop_pitch_in_pixels = mapped_surface.Pitch / 4;

        let skip_x = if given_left < 0 { -1 * given_left } else { 0 };
        let skip_y = if given_top < 0 { -1 * given_top } else { 0 };

        if is_mono {
            for row in 0..*pointer_height {
                let mut mask = 0x80u8;
                mask = mask.wrapping_shr((skip_x % 8) as u32);

                for col in 0..*pointer_width {
                    let and_mask: u8 = pointer_shape_buffer[((col + skip_x) / 8
                        + (row + skip_y) * (pointer_info.Pitch as i32))
                        as usize]
                        & mask;

                    let xor_mask: u8 = pointer_shape_buffer[((col + skip_x) / 8
                        + (row + skip_y + ((pointer_info.Height / 2) as i32))
                            * (pointer_info.Pitch as i32))
                        as usize]
                        & mask;

                    let and_mask_32: u32 = if and_mask > 0 { 0xFFFFFFFF } else { 0xFF000000 };
                    let xor_mask_32: u32 = if xor_mask > 0 { 0x00FFFFFF } else { 0x00000000 };

                    (*init_buffer_32.add(((row * *pointer_width) + col) as usize)) = (*desktop_32
                        .add(((row * desktop_pitch_in_pixels) + col) as usize)
                        & and_mask_32)
                        ^ xor_mask_32;

                    if mask == 0x01 {
                        mask = 0x80;
                    } else {
                        mask = mask.wrapping_shr(1);
                    }
                }
            }
        } else {
            let buffer_32: *mut u32 = std::mem::transmute(pointer_shape_buffer.as_mut_ptr());
            for row in 0..*pointer_height {
                for col in 0..*pointer_width {
                    let mask_val: u32 = 0xFF000000
                        & *buffer_32.add(
                            ((col + skip_x) + ((row + skip_y) * (pointer_info.Pitch as i32 / 4)))
                                as usize,
                        );

                    if mask_val > 0 {
                        // Mask was 0xFF
                        *buffer_32.add(((row * *pointer_width) + col) as usize) = (*desktop_32
                            .add(((row * desktop_pitch_in_pixels) + col) as usize)
                            ^ *buffer_32.add(
                                ((col + skip_x)
                                    + ((row + skip_y) * (pointer_info.Pitch as i32 / 4)))
                                    as usize,
                            ))
                            | 0xFF000000;
                    } else {
                        // Mask was 0x00
                        *buffer_32.add(((row * *pointer_width) + col) as usize) = *buffer_32.add(
                            ((col + skip_x) + ((row + skip_y) * (pointer_info.Pitch as i32 / 4)))
                                as usize,
                        ) | 0xFF000000;
                    }
                }
            }
        }

        check_if_failed!(copy_surface.Unmap());

        Ok(init_buffer)
    }
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
