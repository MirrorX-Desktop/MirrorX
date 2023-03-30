use crate::frame::{state::SharedState, widget::StatefulWidget};

pub struct HistoryView {}

impl HistoryView {
    pub fn new() -> Self {
        Self {}
    }
}

impl StatefulWidget for HistoryView {
    fn update_state(&mut self, shared_state: &SharedState) {
        //
    }

    fn update_view(&self, ui: &mut eframe::egui::Ui) {
        ui.label("History");
    }
}
