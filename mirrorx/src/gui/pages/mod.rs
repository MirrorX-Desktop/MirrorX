pub mod desktop;
pub mod home;

use super::{gpu::Gpu, CustomEvent};
use egui::{FontData, FontDefinitions, FontFamily};
use egui_wgpu_backend::{RenderPass, ScreenDescriptor};
use mirrorx_core::{core_error, error::CoreResult};
use std::time::Instant;
use winit::{
    dpi::{LogicalSize, PhysicalPosition, PhysicalSize},
    event_loop::{EventLoopProxy, EventLoopWindowTarget},
    window::{Window, WindowBuilder, WindowId},
};

#[macro_export]
macro_rules! send_event {
    ($tx:expr, $event:expr) => {
        if let Err(err) = $tx.send($event) {
            tracing::error!("send event {:?} failed", err.0.as_ref());
        }
    };
}

pub trait View {
    fn ui(&mut self, ctx: &egui::Context);
}

#[derive(Debug, Default)]
pub struct PageOptions {
    pub size: LogicalSize<u32>,
    pub min_size: Option<LogicalSize<u32>>,
    pub max_size: Option<LogicalSize<u32>>,
    pub initial_pos: Option<PhysicalPosition<u32>>,
    pub resizable: bool,
    pub maximized: bool,
}

pub struct Page {
    // title: String,
    window: Window,
    egui_ctx: egui::Context,
    egui_state: egui_winit::State,
    screen_descriptor: ScreenDescriptor,
    render_pass: RenderPass,
    view: Box<dyn View>,
    gpu: Gpu,
    next_repaint_instant: Option<Instant>,
}

impl Page {
    pub fn new(
        title: &str,
        options: PageOptions,
        window_target: &EventLoopWindowTarget<CustomEvent>,
        event_loop_proxy: EventLoopProxy<CustomEvent>,
        view: Box<dyn View>,
    ) -> CoreResult<Self> {
        let window = create_window(title, &options, window_target)?;

        let gpu = Gpu::new(&window, window.inner_size())?;

        let mut egui_state = egui_winit::State::new(window_target);
        egui_state.set_pixels_per_point(window.scale_factor() as f32);

        let window_id = window.id();
        let (repaint_tx, mut repaint_rx) = tokio::sync::mpsc::channel(1);

        tokio::task::spawn_blocking(move || loop {
            match repaint_rx.blocking_recv() {
                Some(event) => {
                    if let Err(err) = event_loop_proxy.send_event(event) {
                        tracing::error!(?err, "event loop proxy send user event failed");
                    }
                }
                None => return,
            }
        });

        let egui_ctx = egui::Context::default();
        set_fonts(&egui_ctx);
        egui_ctx.set_request_repaint_callback(move || {
            let _ = repaint_tx.try_send(CustomEvent::Repaint(window_id));
        });

        tracing::info!(
            "width height {} {} {}",
            window.scale_factor(),
            window.inner_size().width,
            window.inner_size().height
        );
        let screen_descriptor = ScreenDescriptor {
            physical_width: window.inner_size().width,
            physical_height: window.inner_size().height,
            scale_factor: window.scale_factor() as f32,
        };

        let render_pass = RenderPass::new(gpu.device(), wgpu::TextureFormat::Bgra8UnormSrgb, 1);

        Ok(Self {
            // title: title.to_string(),
            window,
            egui_ctx,
            egui_state,
            screen_descriptor,
            render_pass,
            view,
            gpu,
            next_repaint_instant: Some(Instant::now()),
        })
    }

    // pub fn title(&self) -> &str {
    //     self.title.as_str()
    // }

    pub fn window_id(&self) -> WindowId {
        self.window.id()
    }

    pub fn handle_event(&mut self, event: &winit::event::WindowEvent) -> bool {
        self.egui_state.on_event(&self.egui_ctx, event)
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        self.gpu.resize(size);
        self.screen_descriptor.physical_width = size.width;
        self.screen_descriptor.physical_height = size.height;
    }

    pub fn scale_factor(&mut self, scale_factor: f64) {
        self.screen_descriptor.scale_factor = scale_factor as f32;
        self.egui_state.set_pixels_per_point(scale_factor as f32);
    }

    pub fn request_redraw(&self) {
        self.window.request_redraw();
    }

    pub fn next_repaint_instant(&self) -> Option<Instant> {
        self.next_repaint_instant
    }

    pub fn render(&mut self) -> CoreResult<()> {
        let raw_input = self.egui_state.take_egui_input(&self.window);
        let full_output = self.egui_ctx.run(raw_input, |egui_ctx| {
            self.view.ui(egui_ctx);
        });

        self.egui_state.handle_platform_output(
            &self.window,
            &self.egui_ctx,
            full_output.platform_output,
        );

        let paint_jobs = self.egui_ctx.tessellate(full_output.shapes);

        let (mut encoder, frame) = self.gpu.prepare()?;

        let texture_view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        self.render_pass
            .add_textures(
                self.gpu.device(),
                self.gpu.queue(),
                &full_output.textures_delta,
            )
            .map_err(|err| core_error!("render backend error ({})", err))?;

        self.render_pass.update_buffers(
            self.gpu.device(),
            self.gpu.queue(),
            &paint_jobs,
            &self.screen_descriptor,
        );

        self.render_pass
            .execute(
                &mut encoder,
                &texture_view,
                &paint_jobs,
                &self.screen_descriptor,
                Some(wgpu::Color::BLACK),
            )
            .map_err(|err| core_error!("render pass execute failed ({})", err))?;

        self.gpu.queue().submit(Some(encoder.finish()));
        frame.present();

        self.next_repaint_instant = Instant::now().checked_add(full_output.repaint_after);

        Ok(())
    }
}

fn create_window(
    title: &str,
    options: &PageOptions,
    window_target: &EventLoopWindowTarget<CustomEvent>,
) -> Result<winit::window::Window, mirrorx_core::error::CoreError> {
    let mut window_builder = {
        #[cfg(target_os = "windows")]
        {
            WindowBuilder::new()
        }

        #[cfg(target_os = "macos")]
        {
            use winit::platform::macos::WindowBuilderExtMacOS;
            WindowBuilder::new()
                .with_fullsize_content_view(true)
                .with_titlebar_transparent(true)
                .with_title_hidden(true)
        }
    }
    .with_title(title)
    .with_resizable(options.resizable)
    .with_maximized(options.maximized)
    .with_inner_size(options.size);

    if let Some(min_size) = options.min_size {
        window_builder = window_builder.with_min_inner_size(min_size);
    }

    if let Some(max_size) = options.max_size {
        window_builder = window_builder.with_max_inner_size(max_size);
    }

    if let Some(position) = options.initial_pos {
        window_builder = window_builder.with_position(position);
    }

    let window = window_builder
        .build(window_target)
        .map_err(|err| core_error!("winit build window error ({})", err))?;

    Ok(window)
}

fn set_fonts(ctx: &egui::Context) {
    let mut fonts = FontDefinitions::default();
    fonts.font_data.insert(
        "NotoSans".to_owned(),
        FontData::from_static(include_bytes!("../../../assets/fonts/NotoSans-Regular.ttf")),
    );
    fonts.font_data.insert(
        "NotoSansJP".to_owned(),
        FontData::from_static(include_bytes!(
            "../../../assets/fonts/NotoSansJP-Regular.otf"
        )),
    );
    fonts.font_data.insert(
        "NotoSansKR".to_owned(),
        FontData::from_static(include_bytes!(
            "../../../assets/fonts/NotoSansKR-Regular.otf"
        )),
    );
    fonts.font_data.insert(
        "NotoSansSC".to_owned(),
        FontData::from_static(include_bytes!(
            "../../../assets/fonts/NotoSansSC-Regular.otf"
        )),
    );
    fonts.font_data.insert(
        "NotoSansTC".to_owned(),
        FontData::from_static(include_bytes!(
            "../../../assets/fonts/NotoSansTC-Regular.otf"
        )),
    );
    fonts.font_data.insert(
        "NotoSansMono".to_owned(),
        FontData::from_static(include_bytes!(
            "../../../assets/fonts/NotoSansMono-Regular.ttf"
        )),
    );

    let mut proportional_fonts = vec![
        "NotoSans".to_owned(),
        "NotoSansSC".to_owned(),
        "NotoSansTC".to_owned(),
        "NotoSansJP".to_owned(),
        "NotoSansKR".to_owned(),
    ];

    let old_fonts = fonts.families.entry(FontFamily::Proportional).or_default();

    proportional_fonts.append(old_fonts);

    fonts
        .families
        .insert(FontFamily::Proportional, proportional_fonts.clone());

    let mut mono_fonts = vec!["NotoSansMono".to_owned()];

    let old_fonts = fonts.families.entry(FontFamily::Monospace).or_default();

    mono_fonts.append(old_fonts);

    fonts
        .families
        .insert(FontFamily::Monospace, mono_fonts.clone());

    // cc.egui_ctx.set_debug_on_hover(true);
    // cc.egui_ctx.request_repaint_after(Duration::from_secs(1));
    ctx.set_fonts(fonts);
}
