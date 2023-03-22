use super::Page;
use crate::frame::state::UIState;

pub struct SettingsPage {}

impl SettingsPage {
    pub fn new() -> Self {
        Self {}
    }
}

impl Page for SettingsPage {
    fn draw(&mut self, ui: &mut eframe::egui::Ui, ui_state: &mut UIState) {
        ui.label("settings");
    }
}
