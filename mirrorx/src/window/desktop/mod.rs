mod desktop_render;
mod state;

use self::desktop_render::DesktopRender;
use mirrorx_core::{
    api::endpoint::message::{InputEvent, KeyboardEvent, MouseEvent},
    component::input::key::{KeyboardKey, MouseKey},
};
use state::{State, StateUpdater};
use std::{sync::Arc, time::Duration};
use tauri_egui::{
    eframe::glow::{self, Context},
    egui::{
        epaint::Shadow, mutex::Mutex, style::Margin, Align, CentralPanel, Color32, FontId, Frame,
        Layout, Pos2, Rect, RichText, Rounding, Sense, Stroke, Ui, Vec2,
    },
};

pub struct DesktopWindow {
    state: State,
    state_updater: StateUpdater,
    desktop_render: Arc<Mutex<DesktopRender>>,
    toolbar_scale_available: bool,
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

        let desktop_render =
            DesktopRender::new(gl_context.as_ref()).expect("create desktop render failed");

        Self {
            state,
            state_updater,
            desktop_render: Arc::new(Mutex::new(desktop_render)),
            toolbar_scale_available: true,
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
            // when client area bigger than original desktop frame, disable scale button
            self.toolbar_scale_available = ui.available_width() < frame.width as _
                || ui.available_height() < frame.height as _;

            if self.state.use_original_resolution()
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
                                    if let Err(err) =
                                        desktop_render.lock().paint(painter.gl(), frame.clone())
                                    {
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
                            emit_input(&self.state_updater, events, move |pos| {
                                pos + left_top.to_vec2()
                            });
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
                    if let Err(err) = desktop_render.lock().paint(painter.gl(), frame.clone()) {
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
                emit_input(&self.state_updater, events, move |pos| {
                    Pos2::new(
                        (pos.x - space_around_image.x).max(0.0) / scale_ratio,
                        (pos.y - space_around_image.y).max(0.0) / scale_ratio,
                    )
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
        ui.add_enabled_ui(self.toolbar_scale_available, |ui| {
            ui.visuals_mut().widgets.active.fg_stroke = Stroke::new(1.0, Color32::WHITE);
            let title = if self.state.use_original_resolution() {
                "适应窗口"
            } else {
                "原始比例"
            };

            let button = tauri_egui::egui::Button::new(title);
            if ui.add(button).clicked() {
                self.state_updater
                    .update_use_original_resolution(!self.state.use_original_resolution());
            }
        });
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

    fn on_exit(&mut self, gl: Option<&glow::Context>) {
        if let Some(gl) = gl {
            self.desktop_render.lock().destroy(gl);
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
