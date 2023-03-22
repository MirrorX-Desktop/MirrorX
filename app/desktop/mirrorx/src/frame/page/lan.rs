use super::Page;
use crate::frame::state::UIState;

pub struct LanPage {}

impl LanPage {
    pub fn new() -> Self {
        Self {}
    }
}

impl Page for LanPage {
    fn draw(&mut self, ui: &mut eframe::egui::Ui, ui_state: &mut UIState) {
        ui.label("lan");
    }
}
