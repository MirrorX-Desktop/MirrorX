use super::View;

#[derive(Default)]
pub struct HistoryPage {}

impl HistoryPage {}

impl View for HistoryPage {
    fn build(&mut self, ui: &mut eframe::egui::Ui) {
        ui.centered_and_justified(|ui| {
            ui.label("Building...");
        });
    }
}
