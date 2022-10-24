mod state;

use std::time::Duration;

use crate::gui::CustomEvent;

use super::View;
use egui::{
    epaint::Shadow, style::Margin, Align, CentralPanel, Color32, ColorImage, FontId, Frame, Layout,
    Pos2, RichText, Rounding, Sense, Stroke, Ui, Vec2,
};
use egui_extras::RetainedImage;
use state::{State, StateUpdater};
use winit::{event_loop::EventLoopProxy, window::WindowId};

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
                        ui.label("Connecting EndPoint Server...");
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
                        ui.label("Connecting EndPoint Server...");
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
                });
            }
        } else {
            ui.centered_and_justified(|ui| {
                let (rect, response) = ui
                    .allocate_exact_size(Vec2::new(160.0, 80.0), Sense::focusable_noninteractive());

                ui.allocate_ui_at_rect(rect, |ui| {
                    ui.spinner();
                    ui.label("Are you ready? Preparing into ...");
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
