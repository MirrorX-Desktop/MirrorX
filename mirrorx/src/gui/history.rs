use super::View;
use mirrorx_core::api::config::ConfigManager;
use std::sync::Arc;

pub struct HistoryPage {
    config_manager: Arc<ConfigManager>,
}

impl HistoryPage {
    pub fn new(config_manager: Arc<ConfigManager>) -> Self {
        Self { config_manager }
    }
}

impl View for HistoryPage {
    fn build(&mut self, ui: &mut eframe::egui::Ui) {
        ui.centered_and_justified(|ui| {
            ui.label("Building...");
        });
    }
}
