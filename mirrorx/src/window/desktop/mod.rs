mod desktop_render;
mod state;

use self::desktop_render::DesktopRender;
use egui_extras::RetainedImage;
use mirrorx_core::{
    api::endpoint::message::{InputEvent, KeyboardEvent, MouseEvent},
    component::input::key::{KeyboardKey, MouseKey},
};
use once_cell::sync::Lazy;
use state::State;
use std::{sync::Arc, time::Duration};
use tauri_egui::{
    eframe::glow::{self, Context},
    egui::{
        epaint::Shadow, mutex::Mutex, style::Margin, Align, CentralPanel, Color32, FontId, Frame,
        Layout, Pos2, Rect, RichText, Rounding, Sense, Stroke, Ui, Vec2,
    },
};

static ICON_MAXIMIZE: Lazy<RetainedImage> = Lazy::new(|| {
    RetainedImage::from_color_image("fa_maximize", egui_extras::image::load_svg_bytes(br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512"><!--! Font Awesome Pro 6.2.0 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license (Commercial License) Copyright 2022 Fonticons, Inc. --><path style="fill:rgb(255,255,255)" d="M168 32H24C10.7 32 0 42.7 0 56V200c0 9.7 5.8 18.5 14.8 22.2s19.3 1.7 26.2-5.2l40-40 79 79L81 335 41 295c-6.9-6.9-17.2-8.9-26.2-5.2S0 302.3 0 312V456c0 13.3 10.7 24 24 24H168c9.7 0 18.5-5.8 22.2-14.8s1.7-19.3-5.2-26.2l-40-40 79-79 79 79-40 40c-6.9 6.9-8.9 17.2-5.2 26.2s12.5 14.8 22.2 14.8H424c13.3 0 24-10.7 24-24V312c0-9.7-5.8-18.5-14.8-22.2s-19.3-1.7-26.2 5.2l-40 40-79-79 79-79 40 40c6.9 6.9 17.2 8.9 26.2 5.2s14.8-12.5 14.8-22.2V56c0-13.3-10.7-24-24-24H280c-9.7 0-18.5 5.8-22.2 14.8s-1.7 19.3 5.2 26.2l40 40-79 79-79-79 40-40c6.9-6.9 8.9-17.2 5.2-26.2S177.7 32 168 32z"/></svg>"#).unwrap())
});

static ICON_ARROWS_LEFT_RIGHT_TO_LINE: Lazy<RetainedImage> = Lazy::new(|| {
    RetainedImage::from_color_image("fa_arrows-left-right-to-line", egui_extras::image::load_svg_bytes(br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 640 512"><!--! Font Awesome Pro 6.2.0 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license (Commercial License) Copyright 2022 Fonticons, Inc. --><path style="fill:rgb(255,255,255)" d="M32 64c17.7 0 32 14.3 32 32l0 320c0 17.7-14.3 32-32 32s-32-14.3-32-32V96C0 78.3 14.3 64 32 64zm214.6 73.4c12.5 12.5 12.5 32.8 0 45.3L205.3 224l229.5 0-41.4-41.4c-12.5-12.5-12.5-32.8 0-45.3s32.8-12.5 45.3 0l96 96c12.5 12.5 12.5 32.8 0 45.3l-96 96c-12.5 12.5-32.8 12.5-45.3 0s-12.5-32.8 0-45.3L434.7 288l-229.5 0 41.4 41.4c12.5 12.5 12.5 32.8 0 45.3s-32.8 12.5-45.3 0l-96-96c-12.5-12.5-12.5-32.8 0-45.3l96-96c12.5-12.5 32.8-12.5 45.3 0zM640 96V416c0 17.7-14.3 32-32 32s-32-14.3-32-32V96c0-17.7 14.3-32 32-32s32 14.3 32 32z"/></svg>"#).unwrap())
});

pub struct DesktopWindow {
    state: State,
    desktop_render: Arc<Mutex<DesktopRender>>,
    press_alt: bool,
    press_ctrl: bool,
    press_shift: bool,
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
        addr: String,
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
            addr,
        );

        let desktop_render =
            DesktopRender::new(gl_context.as_ref()).expect("create desktop render failed");

        Self {
            state,
            desktop_render: Arc::new(Mutex::new(desktop_render)),
            press_alt: false,
            press_ctrl: false,
            press_shift: false,
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
        if let Some(frame) = self.state.current_frame() {
            // when client area bigger than original desktop frame, disable scale button
            self.state.set_desktop_frame_scalable(
                ui.available_width() < frame.width as _
                    || ui.available_height() < frame.height as _,
            );

            if self.state.desktop_frame_scaled()
                && (ui.available_width() < frame.width as _
                    || ui.available_height() < frame.height as _)
            {
                let left = ((ui.available_width() - frame.width as f32) / 2.0).max(0.0);
                let top = ((ui.available_height() - frame.height as f32) / 2.0).max(0.0);

                let mut available_rect = ui.available_rect_before_wrap();
                available_rect.min = Pos2::new(left, top);

                ui.allocate_ui_at_rect(available_rect, |ui| {
                    tauri_egui::egui::ScrollArea::both()
                        .auto_shrink([false; 2])
                        .show_viewport(ui, |ui, view_port| {
                            ui.set_width(frame.width as f32);
                            ui.set_height(frame.height as f32);

                            let desktop_render = self.desktop_render.clone();

                            let cb = tauri_egui::eframe::egui_glow::CallbackFn::new(
                                move |_info, painter| {
                                    if let Err(err) = desktop_render.lock().paint(
                                        painter.gl(),
                                        frame.clone(),
                                        painter.intermediate_fbo(),
                                    ) {
                                        tracing::error!(?err, "desktop render failed");
                                    }
                                },
                            );

                            let callback = tauri_egui::egui::PaintCallback {
                                rect: ui.available_rect_before_wrap(),
                                callback: Arc::new(cb),
                            };

                            ui.painter().add(callback);

                            let input = ui.ctx().input();
                            let events = input.events.as_slice();
                            let left_top = view_port.left_top();
                            self.emit_input(events, move |pos| Some(pos + left_top.to_vec2()));
                        });
                });
            } else {
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

                let desktop_render = self.desktop_render.clone();

                let cb = tauri_egui::eframe::egui_glow::CallbackFn::new(move |_info, painter| {
                    if let Err(err) = desktop_render.lock().paint(
                        painter.gl(),
                        frame.clone(),
                        painter.intermediate_fbo(),
                    ) {
                        tracing::error!(?err, "desktop render failed");
                    }
                });

                let callback = tauri_egui::egui::PaintCallback {
                    rect: Rect {
                        min: space_around_image.to_pos2(),
                        max: space_around_image.to_pos2() + desktop_size.into(),
                    },
                    callback: Arc::new(cb),
                };

                ui.painter().add(callback);

                let input = ui.ctx().input();
                let events = input.events.as_slice();
                self.emit_input(events, move |pos| {
                    if (space_around_image.x <= pos.x
                        && pos.x <= space_around_image.x + desktop_size.0)
                        && (space_around_image.y <= pos.y
                            && pos.y <= space_around_image.y + desktop_size.1)
                    {
                        Some(Pos2::new(
                            (pos.x - space_around_image.x).max(0.0) / scale_ratio,
                            (pos.y - space_around_image.y).max(0.0) / scale_ratio,
                        ))
                    } else {
                        None
                    }
                });
            }
        } else {
            ui.centered_and_justified(|ui| {
                let (rect, _) = ui
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
        let (mut rect, _) = ui.allocate_at_least(Vec2::new(220.0, 35.0), Sense::click());
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
                        // remote device id
                        ui.label(
                            RichText::new(self.state.format_remote_device_id())
                                .font(FontId::monospace(22.0)),
                        );

                        ui.separator();

                        self.build_toolbar_button_scale(ui);

                        ui.separator();

                        // FPS

                        ui.label(
                            RichText::new(self.desktop_render.lock().frame_rate().to_string())
                                .font(FontId::monospace(24.0)), // FontFamily::Name("LiquidCrystal".into()))),
                        );
                    })
                })
        });
    }

    fn build_toolbar_button_scale(&mut self, ui: &mut Ui) {
        // when use_original_resolution is true, the button should display 'fit size' icon
        ui.add_enabled_ui(self.state.desktop_frame_scalable(), |ui| {
            // ui.visuals_mut().widgets.active.fg_stroke = Stroke::new(1.0, Color32::WHITE);
            let button = if self.state.desktop_frame_scaled() {
                tauri_egui::egui::ImageButton::new(
                    ICON_ARROWS_LEFT_RIGHT_TO_LINE.texture_id(ui.ctx()),
                    Vec2::new(18.0, 18.0),
                )
            } else {
                tauri_egui::egui::ImageButton::new(
                    ICON_MAXIMIZE.texture_id(ui.ctx()),
                    Vec2::new(18.0, 18.0),
                )
            }
            .tint(ui.visuals().noninteractive().fg_stroke.color);

            if ui.add(button).clicked() {
                self.state
                    .set_desktop_frame_scaled(!self.state.desktop_frame_scaled());
            }
        });
    }
}

impl DesktopWindow {
    fn emit_input(
        &mut self,
        events: &[tauri_egui::egui::Event],
        pos_calc_fn: impl Fn(Pos2) -> Option<Pos2>,
    ) {
        if let Some(client) = self.state.endpoint_client() {
            let mut input_series = Vec::new();
            for event in events.iter() {
                match event {
                    tauri_egui::egui::Event::PointerMoved(pos) => {
                        if let Some(mouse_pos) = pos_calc_fn(*pos) {
                            input_series.push(InputEvent::Mouse(MouseEvent::Move(
                                MouseKey::None,
                                mouse_pos.x,
                                mouse_pos.y,
                            )));
                        }
                    }
                    tauri_egui::egui::Event::PointerButton {
                        pos,
                        button,
                        pressed,
                        modifiers,
                    } => {
                        let Some(mouse_pos) = pos_calc_fn(*pos)else{
                            continue;
                        };

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
                        input_series
                            .push(InputEvent::Mouse(MouseEvent::ScrollWheel(scroll_vector.y)));
                    }
                    tauri_egui::egui::Event::Key {
                        key,
                        pressed,
                        modifiers,
                    } => {
                        tracing::info!(?key, ?pressed, ?modifiers, "modifiers");

                        // todo: mac command map

                        if *pressed {
                            if modifiers.alt {
                                self.press_alt = true;
                                input_series.push(InputEvent::Keyboard(KeyboardEvent::KeyDown(
                                    KeyboardKey::LeftAlt,
                                )))
                            }

                            if modifiers.ctrl {
                                self.press_ctrl = true;
                                input_series.push(InputEvent::Keyboard(KeyboardEvent::KeyDown(
                                    KeyboardKey::LeftControl,
                                )))
                            }

                            if modifiers.shift {
                                self.press_ctrl = true;
                                input_series.push(InputEvent::Keyboard(KeyboardEvent::KeyDown(
                                    KeyboardKey::LeftShift,
                                )))
                            }

                            input_series
                                .push(InputEvent::Keyboard(KeyboardEvent::KeyDown(map_key(*key))));
                        } else {
                            input_series
                                .push(InputEvent::Keyboard(KeyboardEvent::KeyUp(map_key(*key))));

                            if self.press_alt && !modifiers.alt {
                                self.press_alt = false;
                                input_series.push(InputEvent::Keyboard(KeyboardEvent::KeyUp(
                                    KeyboardKey::LeftAlt,
                                )))
                            }

                            if self.press_ctrl && !modifiers.ctrl {
                                self.press_ctrl = false;
                                input_series.push(InputEvent::Keyboard(KeyboardEvent::KeyUp(
                                    KeyboardKey::LeftControl,
                                )))
                            }

                            if self.press_shift && !modifiers.shift {
                                self.press_shift = false;
                                input_series.push(InputEvent::Keyboard(KeyboardEvent::KeyUp(
                                    KeyboardKey::LeftShift,
                                )))
                            }
                        }
                    }
                    tauri_egui::egui::Event::Text(text) => {
                        tracing::info!(?text, "input text");
                    }
                    _ => {}
                }
            }

            if !input_series.is_empty() {
                tracing::info!(?input_series, "input series");
                if let Err(err) = client.send_input_command(input_series) {
                    tracing::error!(?err, "endpoint input failed");
                }
            }
        }
    }
}

impl tauri_egui::eframe::App for DesktopWindow {
    fn update(&mut self, ctx: &tauri_egui::egui::Context, _: &mut tauri_egui::eframe::Frame) {
        let update_instant = std::time::Instant::now();

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

    fn on_exit(&mut self, gl: Option<&glow::Context>) {
        if let Some(gl) = gl {
            self.desktop_render.lock().destroy(gl);
        }
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
