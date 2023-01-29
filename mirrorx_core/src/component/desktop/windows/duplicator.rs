use super::{
    dx_math::{VERTEX_STRIDES, VERTICES},
    shader,
    util::{init_directx, prepare_desktop},
};
use crate::{
    component::{desktop::windows::dx_math::Vertex, frame::DesktopEncodeFrame},
    core_error,
    error::{CoreError, CoreResult},
    HRESULT,
};
use scopeguard::defer;
use std::os::raw::c_void;
use tracing::info;
use windows::{
    core::{Interface, PCSTR, PCWSTR},
    Win32::{
        Graphics::{
            Direct3D::*,
            Direct3D11::*,
            Dxgi::{Common::*, *},
            Gdi::*,
        },
        System::WindowsProgramming::INFINITE,
        UI::WindowsAndMessaging::*,
    },
};

pub struct Duplicator {
    device: ID3D11Device,
    device_context: ID3D11DeviceContext,

    vertex_shader: ID3D11VertexShader,
    vertex_buffer: Option<ID3D11Buffer>,

    pixel_shader: ID3D11PixelShader,
    pixel_shader_luminance: ID3D11PixelShader,
    pixel_shader_chrominance: ID3D11PixelShader,

    duplication: IDXGIOutputDuplication,
    dxgi_outdupl_desc: DXGI_OUTDUPL_DESC,

    backend_texture: ID3D11Texture2D,
    backend_viewport: [D3D11_VIEWPORT; 1],
    backend_rtv: [Option<ID3D11RenderTargetView>; 1],

    luminance_render_texture: ID3D11Texture2D,
    luminance_staging_texture: ID3D11Texture2D,
    luminance_viewport: [D3D11_VIEWPORT; 1],
    luminance_rtv: [Option<ID3D11RenderTargetView>; 1],

    chrominance_render_texture: ID3D11Texture2D,
    chrominance_staging_texture: ID3D11Texture2D,
    chrominance_viewport: [D3D11_VIEWPORT; 1],
    chrominance_rtv: [Option<ID3D11RenderTargetView>; 1],

    sampler_state: [Option<ID3D11SamplerState>; 1],
    blend_state: ID3D11BlendState,

    mouse_position_x: i32,
    mouse_position_y: i32,
    mouse_last_timestamp: i64,
    mouse_visible: bool,
    mouse_shape_buffer: Vec<u8>,
    mouse_shape_info: DXGI_OUTDUPL_POINTER_SHAPE_INFO,

    epoch: once_cell::unsync::OnceCell<std::time::Instant>,
}

unsafe impl Send for Duplicator {}

impl Duplicator {
    pub fn new(monitor_id: Option<String>) -> CoreResult<(Duplicator, String)> {
        unsafe {
            prepare_desktop()?;

            let (device, device_context) = init_directx()?;

            let (
                vertex_shader,
                vertex_buffer,
                pixel_shader,
                pixel_shader_lumina,
                pixel_shader_chrominance,
            ) = init_shaders(&device)?;

            let (duplication, monitor_id) = init_output_duplication(&device, monitor_id)?;

            let mut dxgi_outdupl_desc = std::mem::zeroed();
            duplication.GetDesc(&mut dxgi_outdupl_desc);

            let (backend_texture, backend_rtv, backend_viewport) =
                init_backend_resources(&device, &dxgi_outdupl_desc)?;

            let (lumina_render_texture, lumina_staging_texture, lumina_viewport, lumina_rtv) =
                init_lumina_resources(&device, &dxgi_outdupl_desc)?;

            let (
                chrominance_render_texture,
                chrominance_staging_texture,
                chrominance_viewport,
                chrominance_rtv,
            ) = init_chrominance_resources(&device, &dxgi_outdupl_desc)?;

            let sampler_state = init_sampler_state(&device)?;

            let blend_state = init_blend_state(&device)?;

            let input_layout = init_input_layout(&device)?;

            device_context.IASetInputLayout(&input_layout);

            Ok((
                Duplicator {
                    device,
                    device_context,
                    vertex_shader,
                    vertex_buffer: Some(vertex_buffer),
                    pixel_shader,
                    pixel_shader_luminance: pixel_shader_lumina,
                    pixel_shader_chrominance,
                    duplication,
                    dxgi_outdupl_desc,
                    backend_texture,
                    backend_viewport: [backend_viewport],
                    backend_rtv: [Some(backend_rtv)],
                    luminance_render_texture: lumina_render_texture,
                    luminance_staging_texture: lumina_staging_texture,
                    luminance_viewport: [lumina_viewport],
                    luminance_rtv: [Some(lumina_rtv)],
                    chrominance_render_texture,
                    chrominance_staging_texture,
                    chrominance_viewport: [chrominance_viewport],
                    chrominance_rtv: [Some(chrominance_rtv)],
                    sampler_state: [Some(sampler_state)],
                    blend_state,
                    mouse_position_x: 0,
                    mouse_position_y: 0,
                    mouse_last_timestamp: 0,
                    mouse_visible: false,
                    mouse_shape_buffer: Vec::new(),
                    mouse_shape_info: std::mem::zeroed(),
                    epoch: once_cell::unsync::OnceCell::new(),
                },
                monitor_id,
            ))
        }
    }

    pub fn capture(&mut self) -> CoreResult<DesktopEncodeFrame> {
        unsafe {
            if let Err(err) = self.acquire_frame() {
                if let CoreError::HResultError {
                    ref error,
                    file: _,
                    line: _,
                } = err
                {
                    if error.code() == DXGI_ERROR_ACCESS_LOST {
                        // todo: re-init dxig
                        tracing::warn!("DXGI ACCESS LOST");
                    }
                }
                return Err(err);
            }

            self.draw_lumina_and_chrominance_texture()?;
            self.create_capture_frame()
        }
    }

    unsafe fn acquire_frame(&mut self) -> CoreResult<()> {
        let mut dxgi_resource = None;
        let mut dxgi_outdupl_frame_info = std::mem::zeroed();

        loop {
            HRESULT!(self.duplication.AcquireNextFrame(
                INFINITE,
                &mut dxgi_outdupl_frame_info,
                &mut dxgi_resource,
            ));

            self.update_mouse(&dxgi_outdupl_frame_info)?;

            if dxgi_outdupl_frame_info.LastPresentTime == 0 {
                HRESULT!(self.duplication.ReleaseFrame());
                continue;
            }

            break;
        }

        if let Some(resource) = dxgi_resource {
            let desktop_texture: ID3D11Texture2D = HRESULT!(resource.cast());

            self.device_context
                .CopyResource(&self.backend_texture, &desktop_texture);

            if self.mouse_visible {
                self.draw_mouse()?;
            }
        }

        HRESULT!(self.duplication.ReleaseFrame());
        Ok(())
    }

    unsafe fn draw_lumina_and_chrominance_texture(&self) -> CoreResult<()> {
        let mut backend_texture_desc = std::mem::zeroed();
        self.backend_texture.GetDesc(&mut backend_texture_desc);

        let shader_resouce_view_desc = D3D11_SHADER_RESOURCE_VIEW_DESC {
            Format: backend_texture_desc.Format,
            ViewDimension: D3D11_SRV_DIMENSION_TEXTURE2D,
            Anonymous: D3D11_SHADER_RESOURCE_VIEW_DESC_0 {
                Texture2D: D3D11_TEX2D_SRV {
                    MostDetailedMip: backend_texture_desc.MipLevels - 1,
                    MipLevels: backend_texture_desc.MipLevels,
                },
            },
        };

        let shader_resource_view = HRESULT!(self
            .device
            .CreateShaderResourceView(&self.backend_texture, Some(&shader_resouce_view_desc)));

        let shader_resource_view = [Some(shader_resource_view)];

        self.device_context.IASetVertexBuffers(
            0,
            1,
            Some(&self.vertex_buffer),
            Some(&VERTEX_STRIDES),
            Some(&0),
        );

        self.device_context.VSSetShader(&self.vertex_shader, None);

        // draw lumina plane

        self.device_context
            .OMSetRenderTargets(Some(&self.luminance_rtv), None);

        self.device_context
            .PSSetShaderResources(0, Some(&shader_resource_view));

        self.device_context
            .PSSetShader(&self.pixel_shader_luminance, None);

        self.device_context
            .RSSetViewports(Some(&self.luminance_viewport));

        self.device_context.Draw(VERTICES.len() as u32, 0);

        // draw chrominance plane

        self.device_context
            .OMSetRenderTargets(Some(&self.chrominance_rtv), None);

        self.device_context
            .PSSetShaderResources(0, Some(&shader_resource_view));

        self.device_context
            .PSSetShader(&self.pixel_shader_chrominance, None);

        self.device_context
            .RSSetViewports(Some(&self.chrominance_viewport));

        self.device_context.Draw(VERTICES.len() as u32, 0);

        Ok(())
    }

    unsafe fn create_capture_frame(&self) -> CoreResult<DesktopEncodeFrame> {
        self.device_context.CopyResource(
            &self.luminance_staging_texture,
            &self.luminance_render_texture,
        );

        self.device_context.CopyResource(
            &self.chrominance_staging_texture,
            &self.chrominance_render_texture,
        );

        let lumina_mapped_resource = HRESULT!(self.device_context.Map(
            &self.luminance_staging_texture,
            0,
            D3D11_MAP_READ,
            0
        ));

        let luminance_stride = lumina_mapped_resource.RowPitch;

        let luminance_bytes = std::slice::from_raw_parts(
            lumina_mapped_resource.pData as *mut u8,
            (self.dxgi_outdupl_desc.ModeDesc.Height * luminance_stride) as usize,
        )
        .to_vec();

        self.device_context
            .Unmap(&self.luminance_staging_texture, 0);

        let chrominance_mapped_resource = HRESULT!(self.device_context.Map(
            &self.chrominance_staging_texture,
            0,
            D3D11_MAP_READ,
            0
        ));

        let chrominance_stride = chrominance_mapped_resource.RowPitch;

        let chrominance_bytes = std::slice::from_raw_parts(
            chrominance_mapped_resource.pData as *mut u8,
            (self.dxgi_outdupl_desc.ModeDesc.Height / 2 * chrominance_stride) as usize,
        )
        .to_vec();

        self.device_context
            .Unmap(&self.chrominance_staging_texture, 0);

        let capture_time = if let Some(instant) = self.epoch.get() {
            instant.elapsed()
        } else {
            let _ = self.epoch.set(std::time::Instant::now());
            std::time::Duration::ZERO
        };

        Ok(DesktopEncodeFrame {
            capture_time,
            width: self.dxgi_outdupl_desc.ModeDesc.Width as i32,
            height: self.dxgi_outdupl_desc.ModeDesc.Height as i32,
            luminance_bytes,
            luminance_stride: luminance_stride as i32,
            chrominance_bytes,
            chrominance_stride: chrominance_stride as i32,
        })
    }

    unsafe fn update_mouse(
        &mut self,
        desktop_frame_info: &DXGI_OUTDUPL_FRAME_INFO,
    ) -> CoreResult<()> {
        if desktop_frame_info.LastMouseUpdateTime == 0 {
            return Ok(());
        }

        let mut update_position = true;

        if !desktop_frame_info.PointerPosition.Visible.as_bool() {
            update_position = false;
        }

        if desktop_frame_info.PointerPosition.Visible.as_bool()
            && self.mouse_visible
            && self.mouse_last_timestamp > desktop_frame_info.LastMouseUpdateTime
        {
            update_position = false;
        }

        if update_position {
            self.mouse_position_x = desktop_frame_info.PointerPosition.Position.x;
            self.mouse_position_y = desktop_frame_info.PointerPosition.Position.y;
            self.mouse_last_timestamp = desktop_frame_info.LastMouseUpdateTime;
            self.mouse_visible = desktop_frame_info.PointerPosition.Visible.as_bool();
        }

        if desktop_frame_info.PointerShapeBufferSize == 0 {
            return Ok(());
        }

        if (desktop_frame_info.PointerShapeBufferSize as usize) > self.mouse_shape_buffer.capacity()
        {
            self.mouse_shape_buffer
                .resize(desktop_frame_info.PointerShapeBufferSize as usize, 0);
        }

        let mut buffer_size_required = 0;
        HRESULT!(self.duplication.GetFramePointerShape(
            desktop_frame_info.PointerShapeBufferSize,
            self.mouse_shape_buffer.as_mut_ptr() as *mut _,
            &mut buffer_size_required,
            &mut self.mouse_shape_info,
        ));

        Ok(())
    }

    unsafe fn draw_mouse(&mut self) -> CoreResult<()> {
        let mut full_desc: D3D11_TEXTURE2D_DESC = std::mem::zeroed();
        self.backend_texture.GetDesc(&mut full_desc);

        let desktop_width = full_desc.Width;
        let desktop_height = full_desc.Height;

        let center_x = (desktop_width / 2) as i32;
        let center_y = (desktop_height / 2) as i32;

        let (mut pointer_width, mut pointer_height, mut pointer_left, mut pointer_top) =
            (0i32, 0i32, 0i32, 0i32);

        let mut pointer_box: D3D11_BOX = std::mem::zeroed();
        pointer_box.front = 0;
        pointer_box.back = 1;

        let mut pointer_texture_desc: D3D11_TEXTURE2D_DESC = std::mem::zeroed();
        pointer_texture_desc.MipLevels = 1;
        pointer_texture_desc.ArraySize = 1;
        pointer_texture_desc.Format = DXGI_FORMAT_B8G8R8A8_UNORM;
        pointer_texture_desc.SampleDesc.Count = 1;
        pointer_texture_desc.SampleDesc.Quality = 0;
        pointer_texture_desc.Usage = D3D11_USAGE_DEFAULT;
        pointer_texture_desc.BindFlags = D3D11_BIND_SHADER_RESOURCE;
        pointer_texture_desc.CPUAccessFlags = D3D11_CPU_ACCESS_FLAG(0);
        pointer_texture_desc.MiscFlags = D3D11_RESOURCE_MISC_FLAG(0);

        let mut shader_resource_view_desc: D3D11_SHADER_RESOURCE_VIEW_DESC = std::mem::zeroed();
        shader_resource_view_desc.Format = pointer_texture_desc.Format;
        shader_resource_view_desc.ViewDimension = D3D11_SRV_DIMENSION_TEXTURE2D;
        shader_resource_view_desc
            .Anonymous
            .Texture2D
            .MostDetailedMip = pointer_texture_desc.MipLevels - 1;
        shader_resource_view_desc.Anonymous.Texture2D.MipLevels = pointer_texture_desc.MipLevels;

        let mut init_buffer = std::ptr::null();

        match DXGI_OUTDUPL_POINTER_SHAPE_TYPE(self.mouse_shape_info.Type as i32) {
            DXGI_OUTDUPL_POINTER_SHAPE_TYPE_COLOR => {
                tracing::trace!(
                    "DXGI_OUTDUPL_POINTER_SHAPE_INFO: DXGI_OUTDUPL_POINTER_SHAPE_TYPE_COLOR"
                );

                tracing::trace!(
                    "visible,{},{},{},{:?}",
                    self.mouse_position_x,
                    self.mouse_position_y,
                    self.mouse_visible,
                    self.mouse_shape_buffer
                );

                std::fs::write(r"F:\ddd_image", &self.mouse_shape_buffer).unwrap();

                pointer_left = self.mouse_position_x;
                pointer_top = self.mouse_position_y;
                pointer_width = self.mouse_shape_info.Width as i32;
                pointer_height = self.mouse_shape_info.Height as i32;
            }
            DXGI_OUTDUPL_POINTER_SHAPE_TYPE_MONOCHROME => {
                tracing::trace!(
                    "DXGI_OUTDUPL_POINTER_SHAPE_INFO: DXGI_OUTDUPL_POINTER_SHAPE_TYPE_MONOCHROME"
                );

                let buffer = self.process_mono_mask(
                    true,
                    &full_desc,
                    &mut pointer_width,
                    &mut pointer_height,
                    &mut pointer_left,
                    &mut pointer_top,
                    &mut pointer_box,
                )?;

                init_buffer = buffer.as_ptr()
            }
            DXGI_OUTDUPL_POINTER_SHAPE_TYPE_MASKED_COLOR => {
                tracing::trace!(
                    "DXGI_OUTDUPL_POINTER_SHAPE_INFO: DXGI_OUTDUPL_POINTER_SHAPE_TYPE_MASKED_COLOR"
                );

                let buffer = self.process_mono_mask(
                    false,
                    &full_desc,
                    &mut pointer_width,
                    &mut pointer_height,
                    &mut pointer_left,
                    &mut pointer_top,
                    &mut pointer_box,
                )?;

                init_buffer = buffer.as_ptr()
            }
            _ => {
                tracing::error!(
                    "duplicator mouse shape type is unknown: {}",
                    self.mouse_shape_info.Type
                );
            }
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

        pointer_texture_desc.Width = pointer_width as u32;
        if pointer_texture_desc.Width == 0 {
            tracing::error!("pointer width == 0, {:?}", pointer_width);
        }
        pointer_texture_desc.Height = pointer_height as u32;
        if pointer_texture_desc.Height == 0 {
            tracing::error!("pointer height == 0, {:?}", pointer_height);
        }

        let mut init_data: D3D11_SUBRESOURCE_DATA = std::mem::zeroed();
        init_data.pSysMem =
            if self.mouse_shape_info.Type & (DXGI_OUTDUPL_POINTER_SHAPE_TYPE_COLOR.0 as u32) != 0 {
                self.mouse_shape_buffer.as_ptr() as *const _
            } else {
                init_buffer as *const _
            };
        init_data.SysMemPitch =
            if self.mouse_shape_info.Type & (DXGI_OUTDUPL_POINTER_SHAPE_TYPE_COLOR.0 as u32) != 0 {
                self.mouse_shape_info.Pitch
            } else {
                (pointer_width * 4) as u32
            };
        init_data.SysMemSlicePitch = 0;

        let pointer_texture = HRESULT!(self
            .device
            .CreateTexture2D(&pointer_texture_desc, Some(&init_data)));

        let shader_res = HRESULT!(self
            .device
            .CreateShaderResourceView(&pointer_texture, Some(&shader_resource_view_desc)));

        let mut buffer_desc: D3D11_BUFFER_DESC = std::mem::zeroed();
        buffer_desc.Usage = D3D11_USAGE_DEFAULT;
        buffer_desc.ByteWidth = (std::mem::size_of::<Vertex>() * VERTICES.len()) as u32;
        buffer_desc.BindFlags = D3D11_BIND_VERTEX_BUFFER;
        buffer_desc.CPUAccessFlags = D3D11_CPU_ACCESS_FLAG(0);

        init_data = std::mem::zeroed();
        init_data.pSysMem = vertices.as_ptr() as *const _;

        let mouse_vertex_buffer = Some(HRESULT!(self
            .device
            .CreateBuffer(&buffer_desc, Some(&init_data))));

        let blend_factor = [0f32; 4];
        let stride = std::mem::size_of::<Vertex>() as u32;
        let offset = 0;

        self.device_context
            .IASetPrimitiveTopology(D3D11_PRIMITIVE_TOPOLOGY_TRIANGLELIST);

        self.device_context.IASetVertexBuffers(
            0,
            1,
            Some(&mouse_vertex_buffer),
            Some(&stride),
            Some(&offset),
        );

        self.device_context.OMSetBlendState(
            &self.blend_state,
            Some(blend_factor.as_ptr()),
            0xFFFFFFFF,
        );

        self.device_context
            .OMSetRenderTargets(Some(&self.backend_rtv), None);

        self.device_context
            .VSSetShader(Some(&self.vertex_shader), None);

        self.device_context
            .PSSetShader(Some(&self.pixel_shader), None);

        self.device_context
            .PSSetShaderResources(0, Some(&[Some(shader_res)]));

        self.device_context
            .PSSetSamplers(0, Some(&self.sampler_state));

        self.device_context
            .RSSetViewports(Some(&self.backend_viewport));

        self.device_context.Draw(VERTICES.len() as u32, 0);

        // reset blend state
        self.device_context.OMSetBlendState(None, None, 0xFFFFFFFF);

        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    unsafe fn process_mono_mask(
        &mut self,
        is_mono: bool,
        full_desc: &D3D11_TEXTURE2D_DESC,
        pointer_width: &mut i32,
        pointer_height: &mut i32,
        pointer_left: &mut i32,
        pointer_top: &mut i32,
        pointer_box: &mut D3D11_BOX,
    ) -> CoreResult<Vec<u8>> {
        let desktop_width = full_desc.Width as i32;
        let desktop_height = full_desc.Height as i32;

        let given_left = self.mouse_position_x;
        let given_top = self.mouse_position_y;

        if given_left < 0 {
            *pointer_width = given_left + self.mouse_shape_info.Width as i32;
        } else if given_left + (self.mouse_shape_info.Width as i32) > desktop_width {
            *pointer_width = desktop_width - given_left;
        } else {
            *pointer_width = self.mouse_shape_info.Width as i32;
        }

        if is_mono {
            self.mouse_shape_info.Height /= 2;
        }

        if given_top < 0 {
            *pointer_height = given_top + self.mouse_shape_info.Height as i32;
        } else if given_top + (self.mouse_shape_info.Height as i32) > desktop_height {
            *pointer_height = desktop_height - given_top;
        } else {
            *pointer_height = self.mouse_shape_info.Height as i32;
        }

        if is_mono {
            self.mouse_shape_info.Height *= 2;
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

        let copy_buffer = HRESULT!(self.device.CreateTexture2D(&copy_buffer_desc, None));

        pointer_box.left = *pointer_left as u32;
        pointer_box.top = *pointer_top as u32;
        pointer_box.right = (*pointer_left + *pointer_width) as u32;
        pointer_box.bottom = (*pointer_top + *pointer_height) as u32;

        self.device_context.CopySubresourceRegion(
            &copy_buffer,
            0,
            0,
            0,
            0,
            &self.backend_texture,
            0,
            Some(pointer_box),
        );

        let copy_surface: IDXGISurface = HRESULT!(copy_buffer.cast());

        let mut mapped_surface: DXGI_MAPPED_RECT = std::mem::zeroed();
        HRESULT!(copy_surface.Map(&mut mapped_surface, DXGI_MAP_READ));

        defer! {
            let _ = copy_surface.Unmap();
        }

        let mut init_buffer = Vec::new();
        init_buffer.resize((*pointer_width * *pointer_height * 4) as usize, 0u8);

        let init_buffer_32: &mut [u32] =
            std::slice::from_raw_parts_mut(init_buffer.as_mut_ptr() as _, init_buffer.len() / 4);
        let desktop_32: &[u32] = std::slice::from_raw_parts(
            mapped_surface.pBits as _,
            (desktop_width * desktop_height / 4) as usize,
        );
        let desktop_pitch_in_pixels = mapped_surface.Pitch as usize / std::mem::size_of::<u32>();

        let skip_x = if given_left < 0 { -given_left } else { 0 } as usize;
        let skip_y = if given_top < 0 { -given_top } else { 0 } as usize;

        if is_mono {
            for row in 0..*pointer_height as usize {
                let mut mask: u8 = 0x80;
                mask >>= (skip_x % 8) as u32;

                for col in 0..*pointer_width as usize {
                    let and_mask: u8 = self.mouse_shape_buffer[(col + skip_x) / 8
                        + (row + skip_y) * (self.mouse_shape_info.Pitch as usize)]
                        & mask;

                    let xor_mask: u8 = self.mouse_shape_buffer[(col + skip_x) / 8
                        + ((row + skip_y + ((self.mouse_shape_info.Height / 2) as usize))
                            * (self.mouse_shape_info.Pitch as usize))]
                        & mask;

                    let and_mask_32: u32 = if and_mask > 0 { 0xFFFFFFFF } else { 0xFF000000 };
                    let xor_mask_32: u32 = if xor_mask > 0 { 0x00FFFFFF } else { 0x00000000 };

                    init_buffer_32[row * (*pointer_width as usize) + col] =
                        desktop_32[row * desktop_pitch_in_pixels + col] & and_mask_32 ^ xor_mask_32;

                    if mask == 0x01 {
                        mask = 0x80;
                    } else {
                        mask >>= 1;
                    }
                }
            }
        } else {
            let buffer_32: &mut [u32] = std::slice::from_raw_parts_mut(
                self.mouse_shape_buffer.as_mut_ptr() as _,
                self.mouse_shape_buffer.len() / 4,
            );

            for row in 0..*pointer_height as usize {
                for col in 0..*pointer_width as usize {
                    let v = buffer_32[(col + skip_x)
                        + ((row + skip_y) * (self.mouse_shape_info.Pitch as usize)
                            / std::mem::size_of::<u32>())];

                    let mask_val: u32 = 0xFF000000 & v;

                    init_buffer_32[(row * (*pointer_width) as usize) + col] = if mask_val > 0 {
                        desktop_32[(row * desktop_pitch_in_pixels) + col] ^ v | 0xFF000000
                    } else {
                        v | 0xFF000000
                    };
                }
            }
        }

        Ok(init_buffer)
    }
}

unsafe fn init_output_duplication(
    device: &ID3D11Device,
    monitor_id: Option<String>,
) -> CoreResult<(IDXGIOutputDuplication, String)> {
    let dxgi_device: IDXGIDevice = HRESULT!(device.cast());

    let dxgi_adapter = HRESULT!(dxgi_device.GetParent::<IDXGIAdapter>());

    let adapter_desc = HRESULT!(dxgi_adapter.GetDesc());

    info!(
        name = ?PCWSTR::from_raw(adapter_desc.Description.as_ptr()).to_string()?,
        "DXGI Adapter",
    );

    let mut output_index = 0;

    while let Ok(dxgi_output) = dxgi_adapter.EnumOutputs(output_index) {
        output_index += 1;

        let dxgi_output_desc = HRESULT!(dxgi_output.GetDesc());

        if !dxgi_output_desc.AttachedToDesktop.as_bool() {
            continue;
        }

        let mut dev_index = 0u32;
        loop {
            let origin_device_name = PCWSTR::from_raw(dxgi_output_desc.DeviceName.as_ptr());

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

            if (display_device.StateFlags & DISPLAY_DEVICE_ATTACHED_TO_DESKTOP) != 0 {
                let device_id = PCWSTR::from_raw(display_device.DeviceID.as_ptr()).to_string()?;

                if let Some(ref id) = monitor_id {
                    if *id != device_id {
                        continue;
                    }
                }

                let dxgi_output1: IDXGIOutput1 = HRESULT!(dxgi_output.cast());

                let dxgi_output_duplication = HRESULT!(dxgi_output1.DuplicateOutput(device));

                return Ok((dxgi_output_duplication, device_id));
            }
        }
    }

    Err(core_error!(
        "create IDXGIOutputDuplication failed, all Outputs had tried"
    ))
}

unsafe fn init_shaders(
    device: &ID3D11Device,
) -> CoreResult<(
    ID3D11VertexShader,
    ID3D11Buffer,
    ID3D11PixelShader,
    ID3D11PixelShader,
    ID3D11PixelShader,
)> {
    let vertex_shader = HRESULT!(device.CreateVertexShader(shader::VERTEX_SHADER_BYTES, None));

    let vertex_buffer_desc = D3D11_BUFFER_DESC {
        ByteWidth: VERTEX_STRIDES * VERTICES.len() as u32,
        Usage: D3D11_USAGE_DEFAULT,
        BindFlags: D3D11_BIND_VERTEX_BUFFER,
        CPUAccessFlags: D3D11_CPU_ACCESS_FLAG::default(),
        MiscFlags: D3D11_RESOURCE_MISC_FLAG::default(),
        StructureByteStride: 0,
    };

    let subresource_data = D3D11_SUBRESOURCE_DATA {
        pSysMem: &VERTICES as *const _ as *const c_void,
        SysMemPitch: 0,
        SysMemSlicePitch: 0,
    };

    let vertex_buffer = HRESULT!(device.CreateBuffer(&vertex_buffer_desc, Some(&subresource_data)));

    let pixel_shader = HRESULT!(device.CreatePixelShader(shader::PIXEL_SHADER_BYTES, None));

    let pixel_shader_lumina =
        HRESULT!(device.CreatePixelShader(shader::PIXEL_SHADER_LUMINA_BYTES, None));

    let pixel_shader_chrominance =
        HRESULT!(device.CreatePixelShader(shader::PIXEL_SHADER_CHROMINANCE_BYTES, None));

    Ok((
        vertex_shader,
        vertex_buffer,
        pixel_shader,
        pixel_shader_lumina,
        pixel_shader_chrominance,
    ))
}

unsafe fn init_backend_resources(
    device: &ID3D11Device,
    dxgi_outdupl_desc: &DXGI_OUTDUPL_DESC,
) -> CoreResult<(ID3D11Texture2D, ID3D11RenderTargetView, D3D11_VIEWPORT)> {
    let mut texture_desc: D3D11_TEXTURE2D_DESC = std::mem::zeroed();
    texture_desc.Width = dxgi_outdupl_desc.ModeDesc.Width;
    texture_desc.Height = dxgi_outdupl_desc.ModeDesc.Height;
    texture_desc.MipLevels = 1;
    texture_desc.ArraySize = 1;
    texture_desc.Format = dxgi_outdupl_desc.ModeDesc.Format;
    texture_desc.SampleDesc.Count = 1;
    texture_desc.SampleDesc.Quality = 0;
    texture_desc.Usage = D3D11_USAGE_DEFAULT;
    texture_desc.BindFlags = D3D11_BIND_RENDER_TARGET | D3D11_BIND_SHADER_RESOURCE;

    let texture = HRESULT!(device.CreateTexture2D(&texture_desc, None));

    texture_desc.CPUAccessFlags = D3D11_CPU_ACCESS_READ;
    texture_desc.Usage = D3D11_USAGE_STAGING;
    texture_desc.BindFlags = D3D11_BIND_FLAG::default();

    let rtv = HRESULT!(device.CreateRenderTargetView(&texture, None));

    let viewport = D3D11_VIEWPORT {
        TopLeftX: 0.0,
        TopLeftY: 0.0,
        Width: texture_desc.Width as f32,
        Height: texture_desc.Height as f32,
        MinDepth: 0.0,
        MaxDepth: 1.0,
    };

    Ok((texture, rtv, viewport))
}

unsafe fn init_lumina_resources(
    device: &ID3D11Device,
    dxgi_outdupl_desc: &DXGI_OUTDUPL_DESC,
) -> CoreResult<(
    ID3D11Texture2D,
    ID3D11Texture2D,
    D3D11_VIEWPORT,
    ID3D11RenderTargetView,
)> {
    let mut texture_desc: D3D11_TEXTURE2D_DESC = std::mem::zeroed();
    texture_desc.Width = dxgi_outdupl_desc.ModeDesc.Width;
    texture_desc.Height = dxgi_outdupl_desc.ModeDesc.Height;
    texture_desc.MipLevels = 1;
    texture_desc.ArraySize = 1;
    texture_desc.Format = DXGI_FORMAT_R8_UNORM;
    texture_desc.SampleDesc.Count = 1;
    texture_desc.SampleDesc.Quality = 0;
    texture_desc.Usage = D3D11_USAGE_DEFAULT;
    texture_desc.BindFlags = D3D11_BIND_RENDER_TARGET;

    let render_texture = HRESULT!(device.CreateTexture2D(&texture_desc, None));

    texture_desc.CPUAccessFlags = D3D11_CPU_ACCESS_READ;
    texture_desc.Usage = D3D11_USAGE_STAGING;
    texture_desc.BindFlags = D3D11_BIND_FLAG::default();

    let staging_texture = HRESULT!(device.CreateTexture2D(&texture_desc, None));

    let viewport = D3D11_VIEWPORT {
        TopLeftX: 0.0,
        TopLeftY: 0.0,
        Width: dxgi_outdupl_desc.ModeDesc.Width as f32,
        Height: dxgi_outdupl_desc.ModeDesc.Height as f32,
        MinDepth: 0.0,
        MaxDepth: 1.0,
    };

    let rtv = HRESULT!(device.CreateRenderTargetView(&render_texture, None));

    Ok((render_texture, staging_texture, viewport, rtv))
}

unsafe fn init_chrominance_resources(
    device: &ID3D11Device,
    dxgi_outdupl_desc: &DXGI_OUTDUPL_DESC,
) -> CoreResult<(
    ID3D11Texture2D,
    ID3D11Texture2D,
    D3D11_VIEWPORT,
    ID3D11RenderTargetView,
)> {
    let mut texture_desc: D3D11_TEXTURE2D_DESC = std::mem::zeroed();
    texture_desc.Width = dxgi_outdupl_desc.ModeDesc.Width / 2;
    texture_desc.Height = dxgi_outdupl_desc.ModeDesc.Height / 2;
    texture_desc.MipLevels = 1;
    texture_desc.ArraySize = 1;
    texture_desc.Format = DXGI_FORMAT_R8G8_UNORM;
    texture_desc.SampleDesc.Count = 1;
    texture_desc.SampleDesc.Quality = 0;
    texture_desc.Usage = D3D11_USAGE_DEFAULT;
    texture_desc.BindFlags = D3D11_BIND_RENDER_TARGET;

    let render_texture = HRESULT!(device.CreateTexture2D(&texture_desc, None));

    texture_desc.CPUAccessFlags = D3D11_CPU_ACCESS_READ;
    texture_desc.Usage = D3D11_USAGE_STAGING;
    texture_desc.BindFlags = D3D11_BIND_FLAG::default();

    let staging_texture = HRESULT!(device.CreateTexture2D(&texture_desc, None));

    let viewport = D3D11_VIEWPORT {
        TopLeftX: 0.0,
        TopLeftY: 0.0,
        Width: (dxgi_outdupl_desc.ModeDesc.Width / 2) as f32,
        Height: (dxgi_outdupl_desc.ModeDesc.Height / 2) as f32,
        MinDepth: 0.0,
        MaxDepth: 1.0,
    };

    let rtv = HRESULT!(device.CreateRenderTargetView(&render_texture, None));

    Ok((render_texture, staging_texture, viewport, rtv))
}

unsafe fn init_sampler_state(device: &ID3D11Device) -> CoreResult<ID3D11SamplerState> {
    let mut sampler_desc: D3D11_SAMPLER_DESC = std::mem::zeroed();
    sampler_desc.Filter = D3D11_FILTER_MIN_MAG_MIP_LINEAR;
    sampler_desc.AddressU = D3D11_TEXTURE_ADDRESS_CLAMP;
    sampler_desc.AddressV = D3D11_TEXTURE_ADDRESS_CLAMP;
    sampler_desc.AddressW = D3D11_TEXTURE_ADDRESS_CLAMP;
    sampler_desc.ComparisonFunc = D3D11_COMPARISON_NEVER;
    sampler_desc.MinLOD = 0f32;
    sampler_desc.MaxLOD = D3D11_FLOAT32_MAX;

    let sampler_state = HRESULT!(device.CreateSamplerState(&sampler_desc));

    Ok(sampler_state)
}

unsafe fn init_blend_state(device: &ID3D11Device) -> CoreResult<ID3D11BlendState> {
    let mut blend_desc: D3D11_BLEND_DESC = std::mem::zeroed();
    blend_desc.AlphaToCoverageEnable = true.into();
    blend_desc.IndependentBlendEnable = false.into();
    blend_desc.RenderTarget[0].BlendEnable = true.into();
    blend_desc.RenderTarget[0].SrcBlend = D3D11_BLEND_SRC_ALPHA;
    blend_desc.RenderTarget[0].DestBlend = D3D11_BLEND_INV_SRC_ALPHA;
    blend_desc.RenderTarget[0].BlendOp = D3D11_BLEND_OP_ADD;
    blend_desc.RenderTarget[0].SrcBlendAlpha = D3D11_BLEND_INV_DEST_ALPHA; //D3D11_BLEND_ONE ;
    blend_desc.RenderTarget[0].DestBlendAlpha = D3D11_BLEND_ONE; //D3D11_BLEND_ZERO;
    blend_desc.RenderTarget[0].BlendOpAlpha = D3D11_BLEND_OP_ADD; //D3D11_BLEND_OP_ADD;
    blend_desc.RenderTarget[0].RenderTargetWriteMask = D3D11_COLOR_WRITE_ENABLE_ALL.0 as u8;

    let blend_state = HRESULT!(device.CreateBlendState(&blend_desc));

    Ok(blend_state)
}

unsafe fn init_input_layout(device: &ID3D11Device) -> CoreResult<ID3D11InputLayout> {
    let input_element_desc_array = [
        D3D11_INPUT_ELEMENT_DESC {
            SemanticName: PCSTR(b"POSITION\0".as_ptr()),
            SemanticIndex: 0,
            Format: DXGI_FORMAT_R32G32B32_FLOAT,
            InputSlot: 0,
            AlignedByteOffset: 0,
            InputSlotClass: D3D11_INPUT_PER_VERTEX_DATA,
            InstanceDataStepRate: 0,
        },
        D3D11_INPUT_ELEMENT_DESC {
            SemanticName: PCSTR(b"TEXCOORD\0".as_ptr()),
            SemanticIndex: 0,
            Format: DXGI_FORMAT_R32G32_FLOAT,
            InputSlot: 0,
            AlignedByteOffset: 12,
            InputSlotClass: D3D11_INPUT_PER_VERTEX_DATA,
            InstanceDataStepRate: 0,
        },
    ];

    let input_layout =
        HRESULT!(device.CreateInputLayout(&input_element_desc_array, shader::VERTEX_SHADER_BYTES));

    Ok(input_layout)
}
