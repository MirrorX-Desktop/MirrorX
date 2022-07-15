use super::{dx::DX, dx_math::VERTICES};
use crate::{component::desktop::Frame, error::MirrorXError, utility::wide_char::FromWide};
use anyhow::bail;
use scopeguard::defer;
use std::{ffi::OsString, mem::zeroed, ptr::null};
use tracing::info;
use windows::{
    core::Interface,
    Win32::{
        Graphics::{
            Direct3D::D3D11_SRV_DIMENSION_TEXTURE2D,
            Direct3D11::*,
            Dxgi::{Common::*, *},
            Gdi::{EnumDisplayDevicesW, DISPLAY_DEVICEW, DISPLAY_DEVICE_ATTACHED_TO_DESKTOP},
        },
        System::{
            StationsAndDesktops::{
                CloseDesktop, GetThreadDesktop, OpenInputDesktop, SetThreadDesktop,
            },
            SystemServices::*,
            Threading::GetCurrentThreadId,
            WindowsProgramming::INFINITE,
        },
        UI::WindowsAndMessaging::*,
    },
};

pub struct Duplicator {
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

            Ok(Duplicator {
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
