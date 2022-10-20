use egui::CentralPanel;

use super::View;

pub struct DesktopView {
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
        Self { remote_device_id }
    }
}

impl View for DesktopView {
    fn ui(&mut self, ctx: &egui::Context) {
        CentralPanel::default().show(ctx, |ui| {
            ui.centered_and_justified(|ui| {
                // ui.label("Hello Desktop");
                ui.spinner();
            });
        });
    }
}
