use super::View;

#[derive(Default)]
pub struct HistoryPage {}

impl HistoryPage {}

impl View for HistoryPage {
    fn build(&mut self, ui: &mut eframe::egui::Ui) {
        ui.label("HistoryPage");
    }
}
