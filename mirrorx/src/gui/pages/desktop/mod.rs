mod state;

use super::View;
use egui::{style::Margin, CentralPanel, Frame};
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
}

impl View for DesktopView {
    fn ui(&mut self, ctx: &egui::Context) {
        let frame = Frame::default()
            .inner_margin(Margin::same(0.0))
            .fill(ctx.style().visuals.window_fill());

        CentralPanel::default().frame(frame).show(ctx, |ui| {
            // if let Some(err) = self.state.handle_event() {
            //     self.custom_toasts.error(err.to_string().as_str());
            // }
            // self.custom_toasts.show(ctx);

            ui.centered_and_justified(|ui| {
                ui.label("Hello");
            });
        });
    }
}
