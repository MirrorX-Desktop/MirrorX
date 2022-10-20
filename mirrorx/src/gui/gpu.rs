use mirrorx_core::{core_error, error::CoreResult};
use raw_window_handle::HasRawWindowHandle;
use wgpu::{Device, Queue};

pub struct Gpu {
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface: wgpu::Surface,
    window_size: winit::dpi::PhysicalSize<u32>,
}

impl Gpu {
    pub fn new<W: HasRawWindowHandle + raw_window_handle::HasRawDisplayHandle>(
        window: &W,
        window_size: winit::dpi::PhysicalSize<u32>,
    ) -> CoreResult<Self> {
        let instance = wgpu::Instance::new(wgpu::Backends::PRIMARY);
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
            power_preference: wgpu::PowerPreference::HighPerformance,
        });

        let adapter = tokio::task::block_in_place(move || futures::executor::block_on(adapter))
            .ok_or(core_error!("adapter not found"))?;

        let (device, queue) = tokio::task::block_in_place(move || {
            futures::executor::block_on(
                adapter.request_device(&wgpu::DeviceDescriptor::default(), None),
            )
        })
        .map_err(|err| core_error!("device not found ({})", err))?;

        let gpu = Self {
            device,
            queue,
            surface,
            window_size,
        };
        gpu.reconfigure_surface();

        Ok(gpu)
    }

    fn reconfigure_surface(&self) {
        self.surface.configure(
            &self.device,
            &wgpu::SurfaceConfiguration {
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                format: wgpu::TextureFormat::Bgra8UnormSrgb,
                width: self.window_size.width,
                height: self.window_size.height,
                present_mode: wgpu::PresentMode::Fifo,
                alpha_mode: wgpu::CompositeAlphaMode::Auto,
            },
        )
    }

    pub fn device(&self) -> &Device {
        &self.device
    }

    pub fn queue(&self) -> &Queue {
        &self.queue
    }

    pub fn resize(&mut self, window_size: winit::dpi::PhysicalSize<u32>) {
        self.window_size = window_size;
        self.reconfigure_surface();
    }

    pub fn prepare(&mut self) -> CoreResult<(wgpu::CommandEncoder, wgpu::SurfaceTexture)> {
        let frame = self
            .surface
            .get_current_texture()
            .or_else(|err| match err {
                wgpu::SurfaceError::Outdated => {
                    // Recreate the swap chain to mitigate race condition on drawing surface resize.
                    self.reconfigure_surface();
                    self.surface.get_current_texture()
                }
                err => Err(err),
            })
            .map_err(|err| core_error!("GPU failed to acquire a surface frame ({:?})", err))?;

        let encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("gpu_command_encoder"),
            });

        Ok((encoder, frame))
    }
}
