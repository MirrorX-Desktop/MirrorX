use super::Page;

pub struct SettingsPage {}

impl SettingsPage {
    pub fn new() -> Self {
        Self {}
    }
}

impl Page for SettingsPage {
    fn draw(&mut self, ui: &mut eframe::egui::Ui) {
        ui.label("settings");
    }
}
