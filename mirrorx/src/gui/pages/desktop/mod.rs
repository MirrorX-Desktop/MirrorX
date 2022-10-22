mod state;

use super::View;
use egui::{style::Margin, CentralPanel, Frame, Sense, Ui, Vec2};
use state::State;

pub struct DesktopView {
    state: State,
    remote_device_id: i64,
}

impl DesktopView {
    pub fn new(
        local_device_id: i64,
        remote_device_id: i64,
        opening_key: Vec<u8>,
        opening_nonce: Vec<u8>,
        sealing_key: Vec<u8>,
        sealing_nonce: Vec<u8>,
        visit_credentials: String,
    ) -> Self {
        Self {
            state: State::new(
                local_device_id,
                remote_device_id,
                opening_key,
                opening_nonce,
                sealing_key,
                sealing_nonce,
                visit_credentials,
            ),
            remote_device_id,
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
                ui.centered_and_justified(|ui| ui.label("could serve"));
            }
            state::VisitState::ErrorOccurred => {
                ui.centered_and_justified(|ui| {
                    ui.label(
                        self.state
                            .take_last_error()
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
            self.state.handle_event();
            self.build_panel(ui);
        });
    }
}
