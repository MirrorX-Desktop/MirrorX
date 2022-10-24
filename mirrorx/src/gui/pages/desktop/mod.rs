mod state;

use std::time::Duration;

use crate::gui::CustomEvent;

use super::View;
use egui::{style::Margin, CentralPanel, Color32, ColorImage, Frame, Sense, Ui, Vec2};
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
        event_loop_proxy: EventLoopProxy<CustomEvent>,
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
            event_loop_proxy,
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
                ui.centered_and_justified(|ui| {
                    if let Some(desktop_texture) = self.state.desktop_texture() {
                        let available_width = ui.available_width();
                        let available_height = ui.available_height();
                        let aspect_ratio = desktop_texture.aspect_ratio();

                        let desktop_size = if (available_width / aspect_ratio) < available_height {
                            (available_width, available_width / aspect_ratio)
                        } else {
                            (available_height * aspect_ratio, available_height)
                        };

                        ui.image(desktop_texture, desktop_size);
                    }
                });
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
