use super::{dx::DX, dx_math::VERTICES};
use anyhow::bail;
use log::{info, warn};
use std::{mem::zeroed, ptr::null, slice::from_raw_parts};
use windows::{
    core::Interface,
    Win32::{
        Graphics::{
            Direct3D::D3D11_SRV_DIMENSION_TEXTURE2D,
            Direct3D11::*,
            Dxgi::{Common::*, *},
        },
        System::{
            StationsAndDesktops::{CloseDesktop, OpenInputDesktop, SetThreadDesktop},
            SystemServices::GENERIC_ALL,
            WindowsProgramming::INFINITE,
        },
        UI::HiDpi::{SetProcessDpiAwarenessContext, DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2},
    },
};

pub struct Duplication {
    dx: DX,
    output_desc: DXGI_OUTPUT_DESC,
    output_duplication: IDXGIOutputDuplication,
    backend_texture: ID3D11Texture2D,
    render_texture_lumina: ID3D11Texture2D,
    render_texture_chrominance: ID3D11Texture2D,
    staging_texture_lumina: ID3D11Texture2D,
    staging_texture_chrominance: ID3D11Texture2D,
    view_port_lumina: D3D11_VIEWPORT,
    view_port_chrominance: D3D11_VIEWPORT,
    render_target_view_lumina: ID3D11RenderTargetView,
    render_target_view_chrominance: ID3D11RenderTargetView,
}

impl Duplication {
    pub fn new(output_idx: u32) -> anyhow::Result<Self> {
        unsafe {
            let current_desktop = OpenInputDesktop(0, false, GENERIC_ALL).map_err(|err| {
                anyhow::anyhow!(
                    r#"Duplication: OpenInputDesktop failed {{"error": "{:?}"}}"#,
                    err.code()
                )
            })?;

            let desktop_attached = SetThreadDesktop(current_desktop);
            CloseDesktop(current_desktop);
            if !desktop_attached.as_bool() {
                bail!("Duplication: SetThreadDesktop failed");
            }

            if !SetProcessDpiAwarenessContext(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2).as_bool()
            {
                bail!("Duplication:SetProcessDpiAwarenessContext failed");
            }

            let dx = DX::new()?;
            let (output_desc, output_duplication) = init_output_duplication(&dx, output_idx)?;

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
                BindFlags: D3D11_BIND_SHADER_RESOURCE,
                CPUAccessFlags: D3D11_CPU_ACCESS_FLAG(0),
                MiscFlags: D3D11_RESOURCE_MISC_FLAG(0),
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

            // create lumina plane resource

            texture_desc.Format = DXGI_FORMAT_R8_UNORM;
            texture_desc.BindFlags = D3D11_BIND_RENDER_TARGET;
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

            Ok(Duplication {
                dx,
                output_desc,
                output_duplication,
                backend_texture,
                render_texture_lumina,
                render_texture_chrominance,
                staging_texture_lumina,
                staging_texture_chrominance,
                view_port_lumina,
                view_port_chrominance,
                render_target_view_lumina,
                render_target_view_chrominance,
            })
        }
    }

    pub fn capture_frame(
        &mut self,
        callback: impl FnOnce(i32, i32, *mut u8, u32, *mut u8, u32) -> (),
    ) -> anyhow::Result<()> {
        unsafe {
            self.get_frame()?;
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

            let mapped_resource_chrominance = self.dx.device_context().Map(&self.staging_texture_chrominance, 0, D3D11_MAP_READ, 0).map_err(|err|{
                anyhow::anyhow!(
                    r#"Duplication: ID3D11DeviceContext::Map failed {{"resource_name": "{}", "error": "{:?}"}}"#,
                    "staging_texture_chrominance",
                    err.code()
                )
            })?;

            let width = self.output_desc.DesktopCoordinates.right
                - self.output_desc.DesktopCoordinates.left;

            let height = self.output_desc.DesktopCoordinates.bottom
                - self.output_desc.DesktopCoordinates.top;

            callback(
                width,
                height,
                mapped_resource_lumina.pData as *mut u8,
                mapped_resource_lumina.RowPitch,
                mapped_resource_chrominance.pData as *mut u8,
                mapped_resource_chrominance.RowPitch,
            );

            self.dx
                .device_context()
                .Unmap(&self.staging_texture_lumina, 0);
            self.dx
                .device_context()
                .Unmap(&self.staging_texture_chrominance, 0);

            Ok(())
        }
    }

    unsafe fn get_frame(&mut self) -> anyhow::Result<()> {
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
                warn!("Duplication: IDXGIOutputDuplication::AcquireNextFrame returns DXGI_ERROR_ACCESS_LOST, re-init DXGIOutputDuplication");

                let _ = self.output_duplication.ReleaseFrame();

                std::ptr::drop_in_place(&mut self.output_duplication);

                let (dxgi_output_desc, dxgi_output_duplication) =
                    init_output_duplication(&self.dx, 0)?;

                self.output_duplication = dxgi_output_duplication;
                self.output_desc = dxgi_output_desc;
            }
        }

        let frame_texture :ID3D11Texture2D = match dxgi_resource{
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
            .CopyResource(&self.backend_texture, frame_texture);

        self.output_duplication.ReleaseFrame().map_err(|err| {
            anyhow::anyhow!(
                r#"Duplication: IDXGIOutputDuplication::ReleaseFrame failed {{"error": "{:?}"}}"#,
                err.code()
            )
        })
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
}

unsafe fn init_output_duplication(
    dx: &DX,
    output_idx: u32,
) -> anyhow::Result<(DXGI_OUTPUT_DESC, IDXGIOutputDuplication)> {
    let dxgi_device: IDXGIDevice = dx.device().cast().map_err(|err| {
        anyhow::anyhow!(
            r#"Duplication: ID3D11Device::QueryInterface for IDXGIDevice failed {{"error": "{:?}"}}"#,
            err.code()
        )
    })?;

    let dxgi_adapter = dxgi_device.GetParent::<IDXGIAdapter>().map_err(|err| {
        anyhow::anyhow!(
            r#"Duplication: IDXGIDevice::GetParent for IDXGIAdapter failed {{"error": "{:?}"}}"#,
            err.code()
        )
    })?;

    let adapter_desc = dxgi_adapter.GetDesc()?;
    info!("{:?}", String::from_utf16_lossy(&adapter_desc.Description));

    let dxgi_output = dxgi_adapter.EnumOutputs(output_idx).map_err(|err| {
        anyhow::anyhow!(
            r#"Duplication: IDXGIAdapter::EnumOutputs failed {{"output_idex": "{}", "error": "{:?}"}}"#,
            output_idx,
            err.code()
        )
    })?;

    let dxgi_output_desc = dxgi_output.GetDesc().map_err(|err| {
        anyhow::anyhow!(
            r#"Duplication: IDXGIOutput::GetDesc failed {{"error": "{:?}"}}"#,
            err.code()
        )
    })?;

    let dxgi_output1: IDXGIOutput1 = dxgi_output.cast().map_err(|err| {
        anyhow::anyhow!(
            r#"Duplication: IDXGIOutput::QueryInterface for IDXGIOutput1 failed {{"error": "{:?}"}}"#,
            err.code()
        )
    })?;

    let dxgi_output_duplication = dxgi_output1.DuplicateOutput(dx.device()).map_err(|err| {
        anyhow::anyhow!(
            r#"Duplication: IDXGIOutput1::DuplicateOutput failed {{"error": "{:?}"}}"#,
            err.code()
        )
    })?;

    Ok((dxgi_output_desc, dxgi_output_duplication))
}
