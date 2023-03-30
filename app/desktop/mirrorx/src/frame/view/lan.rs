use crate::frame::{state::SharedState, widget::StatefulWidget};

pub struct LanView {}

impl LanView {
    pub fn new() -> Self {
        Self {}
    }
}

impl StatefulWidget for LanView {
    fn update_state(&mut self, shared_state: &SharedState) {
        //
    }

    fn update_view(&self, ui: &mut eframe::egui::Ui) {
        ui.label("LAN");
    }
}
