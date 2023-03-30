use crate::frame::widget::StatefulWidget;

pub struct SettingsView {}

impl SettingsView {
    pub fn new() -> Self {
        Self {}
    }
}

impl StatefulWidget for SettingsView {
    fn update_state(&mut self, shared_state: &crate::frame::state::SharedState) {
        //
    }

    fn update_view(&self, ui: &mut eframe::egui::Ui) {
        ui.label("Settings");
    }
}
