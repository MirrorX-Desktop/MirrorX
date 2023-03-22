use super::Page;
use crate::frame::state::UIState;

pub struct HistoryPage {}

impl HistoryPage {
    pub fn new() -> Self {
        Self {}
    }
}

impl Page for HistoryPage {
    fn draw(&mut self, ui: &mut eframe::egui::Ui, ui_state: &mut UIState) {
        ui.label("history");
    }
}
