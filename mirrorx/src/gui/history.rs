use super::View;
use std::sync::Arc;

pub struct HistoryPage {}

impl HistoryPage {
    pub fn new() -> Self {
        Self {}
    }
}

impl View for HistoryPage {
    fn build(&mut self, ui: &mut eframe::egui::Ui) {
        ui.centered_and_justified(|ui| {
            ui.label("Building...");
        });
    }
}
