use egui::CentralPanel;

use super::View;

pub struct DesktopView {}

impl DesktopView {
    pub fn new() -> Self {
        Self {}
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
