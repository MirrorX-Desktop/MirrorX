use super::Page;

pub struct HistoryPage {}

impl HistoryPage {
    pub fn new() -> Self {
        Self {}
    }
}

impl Page for HistoryPage {
    fn draw(&mut self, ui: &mut eframe::egui::Ui) {
        ui.label("history");
    }
}
