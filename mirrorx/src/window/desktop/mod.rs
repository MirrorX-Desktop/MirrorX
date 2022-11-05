mod state;

use mirrorx_core::{
    api::endpoint::message::{InputEvent, KeyboardEvent, MouseEvent},
    component::input::key::{KeyboardKey, MouseKey},
    DesktopDecodeFrame,
};
use state::{State, StateUpdater};
use std::{io::Write, sync::Arc, time::Duration};
use tauri_egui::{
    eframe::{
        egui_glow::{self, check_for_gl_error},
        glow::{
            self, Context, HasContext, NativeBuffer, NativeShader, NativeTexture,
            NativeUniformLocation, NativeVertexArray,
        },
    },
    egui::{
        epaint::Shadow,
        mutex::{Mutex, RwLock},
        style::Margin,
        Align, CentralPanel, Color32, ColorImage, FontId, Frame, Layout, PaintCallback, Pos2, Rect,
        RichText, Rounding, Sense, Stroke, TextureHandle, TextureId, Ui, Vec2,
    },
};

pub struct DesktopWindow {
    state: State,
    state_updater: StateUpdater,
    r: Arc<Mutex<RotatingTriangle>>,
    paint_callback: Option<PaintCallback>,
}

impl DesktopWindow {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        local_device_id: i64,
        remote_device_id: i64,
        opening_key: Vec<u8>,
        opening_nonce: Vec<u8>,
        sealing_key: Vec<u8>,
        sealing_nonce: Vec<u8>,
        visit_credentials: String,
        gl_context: Arc<Context>,
    ) -> Self {
        let state = State::new(
            local_device_id,
            remote_device_id,
            opening_key,
            opening_nonce,
            sealing_key,
            sealing_nonce,
            visit_credentials,
        );

        let state_updater = state.new_state_updater();

        Self {
            state,
            state_updater,
            r: Arc::new(Mutex::new(RotatingTriangle::new(gl_context.as_ref()))),
            paint_callback: None,
        }
    }

    fn build_panel(&mut self, ui: &mut Ui) {
        match self.state.visit_state() {
            state::VisitState::Connecting => {
                ui.centered_and_justified(|ui| {
                    let (rect, response) = ui.allocate_exact_size(
                        Vec2::new(160.0, 80.0),
                        Sense::focusable_noninteractive(),
                    );

                    ui.allocate_ui_at_rect(rect, |ui| {
                        ui.spinner();
                        ui.label("connecting");
                    });

                    response
                });
            }
            state::VisitState::Negotiating => {
                ui.centered_and_justified(|ui| {
                    let (rect, response) = ui.allocate_exact_size(
                        Vec2::new(160.0, 80.0),
                        Sense::focusable_noninteractive(),
                    );

                    ui.allocate_ui_at_rect(rect, |ui| {
                        ui.spinner();
                        ui.label("negotiating");
                    });

                    response
                });
            }
            state::VisitState::Serving => {
                self.build_desktop_texture(ui);
                self.build_toolbar(ui);
            }
            state::VisitState::ErrorOccurred => {
                ui.centered_and_justified(|ui| {
                    ui.label(
                        self.state
                            .last_error()
                            .map(|err| err.to_string())
                            .unwrap_or_else(|| String::from("An unknown error occurred")),
                    );
                });
            }
        }
    }

    fn build_desktop_texture(&mut self, ui: &mut Ui) {
        if let Some(frame) = self.state.frame() {
            // let mut pic = frame.luminance_bytes.clone();
            // pic.append(&mut frame.chrominance_bytes.clone());
            // let dir = std::env::temp_dir().join("test_pic");
            // tracing::info!(?dir, "pic wirte");
            // let mut f = std::fs::File::create(dir).unwrap();
            // f.write_all(&pic).unwrap();
            // f.sync_all().unwrap();
            // std::process::exit(0);

            if self.state.use_original_resolution() {
                ui.style_mut().spacing.item_spacing = Vec2::ZERO;
                tauri_egui::egui::ScrollArea::both()
                    .max_width(frame.width as f32)
                    .max_height(frame.height as f32)
                    .auto_shrink([false; 2])
                    .show_viewport(ui, |ui, _| {
                        ui.set_width(frame.width as f32);
                        ui.set_height(frame.height as f32);
                        ui.style_mut().spacing.item_spacing = Vec2::ZERO;

                        let rotating = self.r.clone();

                        let cb = tauri_egui::eframe::egui_glow::CallbackFn::new(
                            move |_info, painter| {
                                rotating.lock().paint(painter.gl(), frame.clone()).unwrap();
                            },
                        );

                        let callback = tauri_egui::egui::PaintCallback {
                            rect: ui.available_rect_before_wrap(),
                            callback: Arc::new(cb),
                        };

                        ui.painter().add(callback);

                        // let events = &response.ctx.input().events;
                        // let left_top = viewport.left_top();
                        // emit_input(&self.state_updater, events, move |pos| {
                        //     pos + left_top.to_vec2()
                        // });
                    });
            } else {
                ui.centered_and_justified(|ui| {
                    let available_width = ui.available_width();
                    let available_height = ui.available_height();
                    let aspect_ratio = (frame.width as f32) / (frame.height as f32);

                    let desktop_size = if (available_width / aspect_ratio) < available_height {
                        (available_width, available_width / aspect_ratio)
                    } else {
                        (available_height * aspect_ratio, available_height)
                    };

                    let scale_ratio = desktop_size.0 / (frame.width as f32);

                    let space_around_image = Vec2::new(
                        (available_width - desktop_size.0) / 2.0,
                        (available_height - desktop_size.1) / 2.0,
                    );

                    let rotating = self.r.clone();

                    let cb =
                        tauri_egui::eframe::egui_glow::CallbackFn::new(move |_info, painter| {
                            rotating.lock().paint(painter.gl(), frame.clone()).unwrap();
                        });

                    let callback = tauri_egui::egui::PaintCallback {
                        rect: Rect {
                            min: space_around_image.to_pos2(),
                            max: space_around_image.to_pos2() + desktop_size.into(),
                        },
                        callback: Arc::new(cb),
                    };

                    ui.painter().add(callback);

                    // let response = ui.image(frame, desktop_size);
                    // let events = &response.ctx.input().events;
                    // emit_input(&self.state_updater, events, move |pos| {
                    //     Pos2::new(
                    //         (pos.x - space_around_image.x).max(0.0) / scale_ratio,
                    //         (pos.y - space_around_image.y).max(0.0) / scale_ratio,
                    //     )
                    // });
                });
            }
        } else {
            ui.centered_and_justified(|ui| {
                let (rect, response) = ui
                    .allocate_exact_size(Vec2::new(160.0, 80.0), Sense::focusable_noninteractive());

                ui.allocate_ui_at_rect(rect, |ui| {
                    ui.spinner();
                    ui.label("preparing");
                });
            });
        }
    }

    fn build_toolbar(&mut self, ui: &mut Ui) {
        // put the toolbar at central top
        let (mut rect, response) = ui.allocate_exact_size(Vec2::new(240.0, 35.0), Sense::click());
        rect.set_center(Pos2::new(ui.max_rect().width() / 2.0, 50.0));

        ui.allocate_ui_at_rect(rect, |ui| {
            Frame::default()
                .inner_margin(Margin::symmetric(6.0, 2.0))
                .rounding(Rounding::same(12.0))
                .fill(ui.style().visuals.window_fill())
                .shadow(Shadow::small_light())
                .stroke(Stroke::new(1.0, Color32::GRAY))
                .show(ui, |ui| {
                    ui.set_min_size(rect.size());

                    ui.style_mut().spacing.item_spacing = Vec2::new(6.0, 2.0);
                    ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                        ui.label(
                            RichText::new(self.state.format_remote_device_id())
                                .font(FontId::proportional(20.0)),
                        );
                        ui.separator();
                        self.build_toolbar_button_scale(ui);
                    });
                })
        });
    }

    fn build_toolbar_button_scale(&mut self, ui: &mut Ui) {
        // when use_original_resolution is true, the button should display 'fit size' icon
        ui.visuals_mut().widgets.active.fg_stroke = Stroke::new(1.0, Color32::WHITE);
        let title = if self.state.use_original_resolution() {
            "原始"
        } else {
            "按比例"
        };
        let button = tauri_egui::egui::Button::new(title);
        if ui.add(button).clicked() {
            self.state_updater
                .update_use_original_resolution(!self.state.use_original_resolution());
        }
    }
}

impl tauri_egui::eframe::App for DesktopWindow {
    fn update(&mut self, ctx: &tauri_egui::egui::Context, frame: &mut tauri_egui::eframe::Frame) {
        let update_instant = std::time::Instant::now();
        // self.build_panel(ui);
        self.state.handle_event(ctx);

        CentralPanel::default()
            .frame(tauri_egui::egui::Frame::none())
            .show(ctx, |ui| {
                self.build_panel(ui);
            });

        // ctx.request_repaint();
        let cost = update_instant.elapsed();

        if let Some(wait) = cost.checked_sub(Duration::from_millis(16)) {
            ctx.request_repaint_after(wait);
        } else {
            ctx.request_repaint();
        }
    }
}

fn emit_input(
    state_updater: &StateUpdater,
    events: &[tauri_egui::egui::Event],
    pos_calc_fn: impl Fn(Pos2) -> Pos2,
) {
    let mut input_series = Vec::new();
    for event in events.iter() {
        match event {
            tauri_egui::egui::Event::PointerMoved(pos) => {
                let mouse_pos = pos_calc_fn(*pos);
                input_series.push(InputEvent::Mouse(MouseEvent::Move(
                    MouseKey::None,
                    mouse_pos.x,
                    mouse_pos.y,
                )));
            }
            tauri_egui::egui::Event::PointerButton {
                pos,
                button,
                pressed,
                modifiers,
            } => {
                let mouse_pos = pos_calc_fn(*pos);

                let mouse_key = match button {
                    tauri_egui::egui::PointerButton::Primary => MouseKey::Left,
                    tauri_egui::egui::PointerButton::Secondary => MouseKey::Right,
                    tauri_egui::egui::PointerButton::Middle => MouseKey::Wheel,
                    tauri_egui::egui::PointerButton::Extra1 => MouseKey::SideBack,
                    tauri_egui::egui::PointerButton::Extra2 => MouseKey::SideForward,
                };

                let mouse_event = if *pressed {
                    MouseEvent::Down(mouse_key, mouse_pos.x, mouse_pos.y)
                } else {
                    MouseEvent::Up(mouse_key, mouse_pos.x, mouse_pos.y)
                };

                input_series.push(InputEvent::Mouse(mouse_event));
            }
            tauri_egui::egui::Event::Scroll(scroll_vector) => {
                input_series.push(InputEvent::Mouse(MouseEvent::ScrollWheel(scroll_vector.y)));
            }
            tauri_egui::egui::Event::Key {
                key,
                pressed,
                modifiers,
            } => {
                // todo: modifiers order
                let keyboard_event = if *pressed {
                    KeyboardEvent::KeyDown(map_key(*key))
                } else {
                    KeyboardEvent::KeyUp(map_key(*key))
                };

                input_series.push(InputEvent::Keyboard(keyboard_event));
            }
            tauri_egui::egui::Event::Text(text) => {
                tracing::info!(?text, "input text");
            }
            _ => {}
        }
    }

    if !input_series.is_empty() {
        tracing::info!(?input_series, "input series");
        state_updater.input(input_series);
    }
}

const fn map_key(key: tauri_egui::egui::Key) -> KeyboardKey {
    match key {
        tauri_egui::egui::Key::ArrowDown => KeyboardKey::ArrowDown,
        tauri_egui::egui::Key::ArrowLeft => KeyboardKey::ArrowLeft,
        tauri_egui::egui::Key::ArrowRight => KeyboardKey::ArrowRight,
        tauri_egui::egui::Key::ArrowUp => KeyboardKey::ArrowUp,
        tauri_egui::egui::Key::Escape => KeyboardKey::Escape,
        tauri_egui::egui::Key::Tab => KeyboardKey::Tab,
        tauri_egui::egui::Key::Backspace => KeyboardKey::Backspace,
        tauri_egui::egui::Key::Enter => KeyboardKey::Enter,
        tauri_egui::egui::Key::Space => KeyboardKey::Space,
        tauri_egui::egui::Key::Insert => KeyboardKey::Insert,
        tauri_egui::egui::Key::Delete => KeyboardKey::Delete,
        tauri_egui::egui::Key::Home => KeyboardKey::Home,
        tauri_egui::egui::Key::End => KeyboardKey::End,
        tauri_egui::egui::Key::PageUp => KeyboardKey::PageUp,
        tauri_egui::egui::Key::PageDown => KeyboardKey::PageDown,
        tauri_egui::egui::Key::Num0 => KeyboardKey::Digit0,
        tauri_egui::egui::Key::Num1 => KeyboardKey::Digit1,
        tauri_egui::egui::Key::Num2 => KeyboardKey::Digit2,
        tauri_egui::egui::Key::Num3 => KeyboardKey::Digit3,
        tauri_egui::egui::Key::Num4 => KeyboardKey::Digit4,
        tauri_egui::egui::Key::Num5 => KeyboardKey::Digit5,
        tauri_egui::egui::Key::Num6 => KeyboardKey::Digit6,
        tauri_egui::egui::Key::Num7 => KeyboardKey::Digit7,
        tauri_egui::egui::Key::Num8 => KeyboardKey::Digit8,
        tauri_egui::egui::Key::Num9 => KeyboardKey::Digit9,
        tauri_egui::egui::Key::A => KeyboardKey::A,
        tauri_egui::egui::Key::B => KeyboardKey::B,
        tauri_egui::egui::Key::C => KeyboardKey::C,
        tauri_egui::egui::Key::D => KeyboardKey::D,
        tauri_egui::egui::Key::E => KeyboardKey::E,
        tauri_egui::egui::Key::F => KeyboardKey::F,
        tauri_egui::egui::Key::G => KeyboardKey::G,
        tauri_egui::egui::Key::H => KeyboardKey::H,
        tauri_egui::egui::Key::I => KeyboardKey::I,
        tauri_egui::egui::Key::J => KeyboardKey::J,
        tauri_egui::egui::Key::K => KeyboardKey::K,
        tauri_egui::egui::Key::L => KeyboardKey::L,
        tauri_egui::egui::Key::M => KeyboardKey::M,
        tauri_egui::egui::Key::N => KeyboardKey::N,
        tauri_egui::egui::Key::O => KeyboardKey::O,
        tauri_egui::egui::Key::P => KeyboardKey::P,
        tauri_egui::egui::Key::Q => KeyboardKey::Q,
        tauri_egui::egui::Key::R => KeyboardKey::R,
        tauri_egui::egui::Key::S => KeyboardKey::S,
        tauri_egui::egui::Key::T => KeyboardKey::T,
        tauri_egui::egui::Key::U => KeyboardKey::U,
        tauri_egui::egui::Key::V => KeyboardKey::V,
        tauri_egui::egui::Key::W => KeyboardKey::W,
        tauri_egui::egui::Key::X => KeyboardKey::X,
        tauri_egui::egui::Key::Y => KeyboardKey::Y,
        tauri_egui::egui::Key::Z => KeyboardKey::Z,
        tauri_egui::egui::Key::F1 => KeyboardKey::F1,
        tauri_egui::egui::Key::F2 => KeyboardKey::F2,
        tauri_egui::egui::Key::F3 => KeyboardKey::F3,
        tauri_egui::egui::Key::F4 => KeyboardKey::F4,
        tauri_egui::egui::Key::F5 => KeyboardKey::F5,
        tauri_egui::egui::Key::F6 => KeyboardKey::F6,
        tauri_egui::egui::Key::F7 => KeyboardKey::F7,
        tauri_egui::egui::Key::F8 => KeyboardKey::F8,
        tauri_egui::egui::Key::F9 => KeyboardKey::F9,
        tauri_egui::egui::Key::F10 => KeyboardKey::F10,
        tauri_egui::egui::Key::F11 => KeyboardKey::F11,
        tauri_egui::egui::Key::F12 => KeyboardKey::F12,
        tauri_egui::egui::Key::F13 => KeyboardKey::PrintScreen,
        tauri_egui::egui::Key::F14 => KeyboardKey::ScrollLock,
        tauri_egui::egui::Key::F15 => KeyboardKey::Pause,
        tauri_egui::egui::Key::F16 => KeyboardKey::Fn, // todo: temp
        tauri_egui::egui::Key::F17 => KeyboardKey::Fn, // todo: temp
        tauri_egui::egui::Key::F18 => KeyboardKey::Fn, // todo: temp
        tauri_egui::egui::Key::F19 => KeyboardKey::Fn, // todo: temp
        tauri_egui::egui::Key::F20 => KeyboardKey::Fn, // todo: temp
    }
}

// const vertex_vertices: [f32; 12] = [
//     1.0, -1.0, 0.0, -1.0, -1.0, 0.0, 1.0, 1.0, 0.0, -1.0, 1.0, 0.0,
// ];

#[rustfmt::skip]
const vertex_vertices: [f32; 20] = [
     1.0,  1.0, 0.0, 1.0, 1.0, 
     1.0, -1.0, 0.0, 1.0, 0.0, 
    -1.0, -1.0, 0.0, 0.0, 0.0, 
    -1.0,  1.0, 0.0, 0.0, 1.0,
];

const vertex_vertices_u8: &[u8] = unsafe {
    std::slice::from_raw_parts(
        vertex_vertices.as_ptr() as *const u8,
        vertex_vertices.len() * std::mem::size_of::<f32>(),
    )
};

// const texture_vertices: [f32; 8] = [1.0, 0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 1.0];

#[rustfmt::skip]
const indices_vertices: [u32; 6] = [
    0, 1, 3, 
    1, 2, 3,
];

const indices_u8: &[u8] = unsafe {
    std::slice::from_raw_parts(
        indices_vertices.as_ptr() as *const u8,
        indices_vertices.len() * std::mem::size_of::<u32>(),
    )
};

struct RotatingTriangle {
    program: glow::Program,
    y_texture: Option<NativeTexture>,
    uv_texture: Option<NativeTexture>,
    vao: NativeVertexArray,
    vbo: NativeBuffer,
    ebo: NativeBuffer,
}

#[allow(unsafe_code)] // we need unsafe code to use glow
impl RotatingTriangle {
    fn new(gl: &glow::Context) -> Self {
        use glow::HasContext as _;

        unsafe {
            tracing::info!("OpenGL version: {:?}", gl.version());

            let program = gl.create_program().expect("Cannot create program");

            let vertex_shader_source = r#"
            #version 330 core
            layout (location = 0) in vec3 aPos;
            layout (location = 1) in vec2 aTexCoord;

            out vec2 texCoord;
            
            void main(void)
            {
                gl_Position = vec4(aPos, 1.0);
                texCoord = vec2(aTexCoord.x, 1 - aTexCoord.y);
            }"#;

            let fragment_shader_source = r#"
            #version 330 core

            uniform sampler2D textureY;
            uniform sampler2D textureUV;

            in vec2 texCoord;
            layout (location = 0) out vec4 fragColor;

            const mat3 YCbCrToRGBmatrix = mat3(
                1.164, 0.000, 1.857,
                1.164,-0.217,-0.543,
                1.164, 2.016, 0.000
            );

            void main(void)
            {
                vec3 yuv;
                vec3 rgb;
                yuv.x = texture(textureY, texCoord.st).r - 0.0625;
                yuv.y = texture(textureUV, texCoord.st).r - 0.5;
                yuv.z = texture(textureUV, texCoord.st).g - 0.5;
                rgb = yuv * YCbCrToRGBmatrix;
                fragColor = vec4(rgb, 1.0);
            }"#;

            // compile, link and attach vertex shader
            let vertex_shader = gl
                .create_shader(glow::VERTEX_SHADER)
                .expect("Cannot create shader");
            check_for_gl_error!(gl);

            gl.shader_source(vertex_shader, vertex_shader_source);
            check_for_gl_error!(gl);

            gl.compile_shader(vertex_shader);
            check_for_gl_error!(gl);

            tracing::info!("{}", gl.get_shader_info_log(vertex_shader));

            if !gl.get_shader_compile_status(vertex_shader) {
                panic!(
                    "Failed to compile vertex shader: {}",
                    gl.get_shader_info_log(vertex_shader)
                );
            }

            gl.attach_shader(program, vertex_shader);
            check_for_gl_error!(gl);

            // compile, link and attach vertex shader
            let fragment_shader = gl
                .create_shader(glow::FRAGMENT_SHADER)
                .expect("Cannot create shader");
            check_for_gl_error!(gl);

            gl.shader_source(fragment_shader, fragment_shader_source);
            check_for_gl_error!(gl);

            gl.compile_shader(fragment_shader);
            check_for_gl_error!(gl);

            tracing::info!("{}", gl.get_shader_info_log(fragment_shader));

            if !gl.get_shader_compile_status(fragment_shader) {
                panic!(
                    "Failed to compile fragment shader: {}",
                    gl.get_shader_info_log(fragment_shader)
                );
            }

            gl.attach_shader(program, fragment_shader);
            check_for_gl_error!(gl);

            gl.bind_attrib_location(program, 0, "aPos");
            gl.bind_attrib_location(program, 1, "aTexCoord");

            gl.link_program(program);
            if !gl.get_program_link_status(program) {
                panic!("{}", gl.get_program_info_log(program));
            }

            gl.detach_shader(program, vertex_shader);
            check_for_gl_error!(gl);

            gl.detach_shader(program, fragment_shader);
            check_for_gl_error!(gl);

            gl.delete_shader(vertex_shader);
            gl.delete_shader(fragment_shader);

            let ebo = gl.create_buffer().unwrap();
            let vao = gl.create_vertex_array().unwrap();
            let vbo = gl.create_buffer().unwrap();

            gl.bind_vertex_array(Some(vao));
            check_for_gl_error!(gl);

            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
            check_for_gl_error!(gl);

            gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, vertex_vertices_u8, glow::STATIC_DRAW);
            check_for_gl_error!(gl);

            gl.vertex_attrib_pointer_f32(
                0,
                3,
                glow::FLOAT,
                false,
                5 * std::mem::size_of::<f32>() as i32,
                0,
            );
            check_for_gl_error!(gl);

            gl.vertex_attrib_pointer_f32(
                1,
                2,
                glow::FLOAT,
                false,
                5 * std::mem::size_of::<f32>() as i32,
                3 * std::mem::size_of::<f32>() as i32,
            );
            check_for_gl_error!(gl);

            gl.enable_vertex_attrib_array(0);
            check_for_gl_error!(gl);

            gl.enable_vertex_attrib_array(1);
            check_for_gl_error!(gl);

            gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(ebo));
            check_for_gl_error!(gl);

            gl.buffer_data_u8_slice(glow::ELEMENT_ARRAY_BUFFER, indices_u8, glow::STATIC_DRAW);
            check_for_gl_error!(gl);

            Self {
                program,
                y_texture: None,
                uv_texture: None,
                vao,
                vbo,
                ebo,
            }
        }
    }

    fn destroy(&self, gl: &glow::Context) {
        use glow::HasContext as _;
        unsafe {
            gl.delete_program(self.program);
            // gl.delete_vertex_array(self.vao);
        }
    }

    fn paint(&mut self, gl: &glow::Context, frame: DesktopDecodeFrame) -> Result<(), String> {
        unsafe {
            if self.y_texture.is_none() {
                self.y_texture = Some(create_texture(
                    gl,
                    true,
                    frame.width,
                    frame.height,
                    frame.luminance_stride,
                ));
            }

            if self.uv_texture.is_none() {
                self.uv_texture = Some(create_texture(
                    gl,
                    false,
                    frame.width / 2,
                    frame.height / 2,
                    frame.chrominance_stride,
                ));
            }

            // gl.clear_color(1.0, 1.0, 1.0, 1.0);
            // gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);

            // gl.disable(glow::SCISSOR_TEST);
            gl.use_program(Some(self.program));
            check_for_gl_error!(gl);

            gl.disable(glow::FRAMEBUFFER_SRGB);
            check_for_gl_error!(gl);

            gl.active_texture(glow::TEXTURE0);
            check_for_gl_error!(gl);
            gl.bind_texture(glow::TEXTURE_2D, self.y_texture);
            check_for_gl_error!(gl);
            gl.tex_sub_image_2d(
                glow::TEXTURE_2D,
                0,
                0,
                0,
                frame.width,
                frame.height,
                glow::RED,
                glow::UNSIGNED_BYTE,
                glow::PixelUnpackData::Slice(&frame.luminance_bytes),
            );
            check_for_gl_error!(gl);

            let y_uniform_location = gl.get_uniform_location(self.program, "textureY");
            check_for_gl_error!(gl);
            gl.uniform_1_i32(y_uniform_location.as_ref(), 0);
            check_for_gl_error!(gl);

            gl.active_texture(glow::TEXTURE1);
            check_for_gl_error!(gl);
            gl.bind_texture(glow::TEXTURE_2D, self.uv_texture);
            check_for_gl_error!(gl);
            gl.tex_sub_image_2d(
                glow::TEXTURE_2D,
                0,
                0,
                0,
                frame.width / 2,
                frame.height / 2,
                glow::RG,
                glow::UNSIGNED_BYTE,
                glow::PixelUnpackData::Slice(&frame.chrominance_bytes),
            );
            check_for_gl_error!(gl);

            let uv_uniform_location = gl.get_uniform_location(self.program, "textureUV");
            check_for_gl_error!(gl);
            gl.uniform_1_i32(uv_uniform_location.as_ref(), 1);
            check_for_gl_error!(gl);

            gl.bind_vertex_array(Some(self.vao));
            check_for_gl_error!(gl);

            gl.draw_elements(glow::TRIANGLES, 6, glow::UNSIGNED_INT, 0);
            check_for_gl_error!(gl);

            Ok(())
        }
    }
}

unsafe fn create_texture(
    gl: &glow::Context,
    is_luminance_texture: bool,
    width: i32,
    height: i32,
    stride: i32,
) -> NativeTexture {
    let texture = gl.create_texture().unwrap();

    gl.bind_texture(glow::TEXTURE_2D, Some(texture));
    check_for_gl_error!(gl);

    // gl.pixel_store_i32(glow::UNPACK_ROW_LENGTH, stride);
    // check_for_gl_error!(gl);

    let internal_format = if is_luminance_texture {
        glow::RED
    } else {
        glow::RG
    };

    gl.tex_image_2d(
        glow::TEXTURE_2D,
        0,
        internal_format as i32,
        width,
        height,
        0,
        internal_format,
        glow::UNSIGNED_BYTE,
        None,
    );
    check_for_gl_error!(gl);

    gl.tex_parameter_i32(
        glow::TEXTURE_2D,
        glow::TEXTURE_MIN_FILTER,
        glow::LINEAR as i32,
    );
    check_for_gl_error!(gl);

    gl.tex_parameter_i32(
        glow::TEXTURE_2D,
        glow::TEXTURE_MAG_FILTER,
        glow::LINEAR as i32,
    );
    check_for_gl_error!(gl);

    gl.tex_parameter_i32(
        glow::TEXTURE_2D,
        glow::TEXTURE_WRAP_S,
        glow::CLAMP_TO_EDGE as i32,
    );
    check_for_gl_error!(gl);

    gl.tex_parameter_i32(
        glow::TEXTURE_2D,
        glow::TEXTURE_WRAP_T,
        glow::CLAMP_TO_EDGE as i32,
    );
    check_for_gl_error!(gl);

    texture
}
