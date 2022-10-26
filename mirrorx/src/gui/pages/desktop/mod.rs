mod state;

use super::View;
use egui::{
    epaint::Shadow, style::Margin, Align, CentralPanel, Color32, FontId, Frame, Layout, Pos2,
    RichText, Rounding, Sense, Stroke, Ui, Vec2,
};
use mirrorx_core::{
    api::endpoint::message::{InputEvent, KeyboardEvent, MouseEvent},
    component::input::key::{KeyboardKey, MouseKey},
};
use state::{State, StateUpdater};

pub struct DesktopView {
    state: State,
    state_updater: StateUpdater,
}

impl DesktopView {
    pub fn new(
        // window_id: WindowId,
        local_device_id: i64,
        remote_device_id: i64,
        opening_key: Vec<u8>,
        opening_nonce: Vec<u8>,
        sealing_key: Vec<u8>,
        sealing_nonce: Vec<u8>,
        visit_credentials: String,
    ) -> Self {
        let state = State::new(
            // window_id,
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
                        ui.label(t!("desktop.label.connecting"));
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
                        ui.label(t!("desktop.label.negotiating"));
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
        if let Some(desktop_texture) = self.state.desktop_texture() {
            if self.state.use_original_resolution() {
                ui.style_mut().spacing.item_spacing = Vec2::ZERO;
                egui::ScrollArea::both()
                    .max_width(desktop_texture.size_vec2().x)
                    .max_height(desktop_texture.size_vec2().y)
                    .auto_shrink([false; 2])
                    .show_viewport(ui, |ui, viewport| {
                        ui.set_width(desktop_texture.size_vec2().x);
                        ui.set_height(desktop_texture.size_vec2().y);

                        ui.image(desktop_texture, desktop_texture.size_vec2());

                        emit_input(&self.state_updater, ui, viewport.left_top());
                    });
            } else {
                ui.centered_and_justified(|ui| {
                    let available_width = ui.available_width();
                    let available_height = ui.available_height();
                    let aspect_ratio = desktop_texture.aspect_ratio();

                    let desktop_size = if (available_width / aspect_ratio) < available_height {
                        (available_width, available_width / aspect_ratio)
                    } else {
                        (available_height * aspect_ratio, available_height)
                    };

                    ui.image(desktop_texture, desktop_size);

                    emit_input(&self.state_updater, ui, Pos2::new(0.0, 0.0));
                });
            }
        } else {
            ui.centered_and_justified(|ui| {
                let (rect, response) = ui
                    .allocate_exact_size(Vec2::new(160.0, 80.0), Sense::focusable_noninteractive());

                ui.allocate_ui_at_rect(rect, |ui| {
                    ui.spinner();
                    ui.label(t!("desktop.label.preparing"));
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
        let texture_id = if self.state.use_original_resolution() {
            crate::gui::assets::font_awesome::FA_DOWN_LEFT_AND_UP_RIGHT_TO_CENTER_SOLID
                .texture_id(ui.ctx())
        } else {
            crate::gui::assets::font_awesome::FA_UP_RIGHT_AND_DOWN_LEFT_FROM_CENTER_SOLID
                .texture_id(ui.ctx())
        };
        let button = egui::ImageButton::new(texture_id, (18.0, 18.0));
        if ui.add(button).clicked() {
            self.state_updater
                .update_use_original_resolution(!self.state.use_original_resolution());
        }
    }
}

impl View for DesktopView {
    fn ui(&mut self, ctx: &egui::Context) {
        let frame = Frame::default()
            .inner_margin(Margin::same(0.0))
            .fill(ctx.style().visuals.window_fill());

        CentralPanel::default().frame(frame).show(ctx, |ui| {
            self.build_panel(ui);
            self.state.handle_event(ctx);
        });

        ctx.request_repaint();
    }
}

fn emit_input(state_updater: &StateUpdater, ui: &mut Ui, mouse_offset: Pos2) {
    let mut input_series = Vec::new();
    for event in ui.input().events.iter() {
        match event {
            egui::Event::PointerMoved(pos) => {
                let mouse_pos = *pos + mouse_offset.to_vec2();
                input_series.push(InputEvent::Mouse(MouseEvent::Move(
                    MouseKey::None,
                    mouse_pos.x,
                    mouse_pos.y,
                )));
            }
            egui::Event::PointerButton {
                pos,
                button,
                pressed,
                modifiers,
            } => {
                let mouse_pos = *pos + mouse_offset.to_vec2();

                let mouse_key = match button {
                    egui::PointerButton::Primary => MouseKey::Left,
                    egui::PointerButton::Secondary => MouseKey::Right,
                    egui::PointerButton::Middle => MouseKey::Wheel,
                    egui::PointerButton::Extra1 => MouseKey::SideBack,
                    egui::PointerButton::Extra2 => MouseKey::SideForward,
                };

                let mouse_event = if *pressed {
                    MouseEvent::Down(mouse_key, mouse_pos.x, mouse_pos.y)
                } else {
                    MouseEvent::Up(mouse_key, mouse_pos.x, mouse_pos.y)
                };

                input_series.push(InputEvent::Mouse(mouse_event));
            }
            egui::Event::Scroll(scroll_vector) => {
                input_series.push(InputEvent::Mouse(MouseEvent::ScrollWheel(scroll_vector.y)));
            }
            egui::Event::Key {
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
            egui::Event::Text(text) => {
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

const fn map_key(key: egui::Key) -> KeyboardKey {
    match key {
        egui::Key::ArrowDown => KeyboardKey::ArrowDown,
        egui::Key::ArrowLeft => KeyboardKey::ArrowLeft,
        egui::Key::ArrowRight => KeyboardKey::ArrowRight,
        egui::Key::ArrowUp => KeyboardKey::ArrowUp,
        egui::Key::Escape => KeyboardKey::Escape,
        egui::Key::Tab => KeyboardKey::Tab,
        egui::Key::Backspace => KeyboardKey::Backspace,
        egui::Key::Enter => KeyboardKey::Enter,
        egui::Key::Space => KeyboardKey::Space,
        egui::Key::Insert => KeyboardKey::Insert,
        egui::Key::Delete => KeyboardKey::Delete,
        egui::Key::Home => KeyboardKey::Home,
        egui::Key::End => KeyboardKey::End,
        egui::Key::PageUp => KeyboardKey::PageUp,
        egui::Key::PageDown => KeyboardKey::PageDown,
        egui::Key::Num0 => KeyboardKey::Digit0,
        egui::Key::Num1 => KeyboardKey::Digit1,
        egui::Key::Num2 => KeyboardKey::Digit2,
        egui::Key::Num3 => KeyboardKey::Digit3,
        egui::Key::Num4 => KeyboardKey::Digit4,
        egui::Key::Num5 => KeyboardKey::Digit5,
        egui::Key::Num6 => KeyboardKey::Digit6,
        egui::Key::Num7 => KeyboardKey::Digit7,
        egui::Key::Num8 => KeyboardKey::Digit8,
        egui::Key::Num9 => KeyboardKey::Digit9,
        egui::Key::A => KeyboardKey::A,
        egui::Key::B => KeyboardKey::B,
        egui::Key::C => KeyboardKey::C,
        egui::Key::D => KeyboardKey::D,
        egui::Key::E => KeyboardKey::E,
        egui::Key::F => KeyboardKey::F,
        egui::Key::G => KeyboardKey::G,
        egui::Key::H => KeyboardKey::H,
        egui::Key::I => KeyboardKey::I,
        egui::Key::J => KeyboardKey::J,
        egui::Key::K => KeyboardKey::K,
        egui::Key::L => KeyboardKey::L,
        egui::Key::M => KeyboardKey::M,
        egui::Key::N => KeyboardKey::N,
        egui::Key::O => KeyboardKey::O,
        egui::Key::P => KeyboardKey::P,
        egui::Key::Q => KeyboardKey::Q,
        egui::Key::R => KeyboardKey::R,
        egui::Key::S => KeyboardKey::S,
        egui::Key::T => KeyboardKey::T,
        egui::Key::U => KeyboardKey::U,
        egui::Key::V => KeyboardKey::V,
        egui::Key::W => KeyboardKey::W,
        egui::Key::X => KeyboardKey::X,
        egui::Key::Y => KeyboardKey::Y,
        egui::Key::Z => KeyboardKey::Z,
        egui::Key::F1 => KeyboardKey::F1,
        egui::Key::F2 => KeyboardKey::F2,
        egui::Key::F3 => KeyboardKey::F3,
        egui::Key::F4 => KeyboardKey::F4,
        egui::Key::F5 => KeyboardKey::F5,
        egui::Key::F6 => KeyboardKey::F6,
        egui::Key::F7 => KeyboardKey::F7,
        egui::Key::F8 => KeyboardKey::F8,
        egui::Key::F9 => KeyboardKey::F9,
        egui::Key::F10 => KeyboardKey::F10,
        egui::Key::F11 => KeyboardKey::F11,
        egui::Key::F12 => KeyboardKey::F12,
        egui::Key::F13 => KeyboardKey::PrintScreen,
        egui::Key::F14 => KeyboardKey::ScrollLock,
        egui::Key::F15 => KeyboardKey::Pause,
        egui::Key::F16 => KeyboardKey::Fn, // todo: temp
        egui::Key::F17 => KeyboardKey::Fn, // todo: temp
        egui::Key::F18 => KeyboardKey::Fn, // todo: temp
        egui::Key::F19 => KeyboardKey::Fn, // todo: temp
        egui::Key::F20 => KeyboardKey::Fn, // todo: temp
    }
}
