use super::View;

#[derive(Default)]
pub struct LANPage {}

impl LANPage {}

impl View for LANPage {
    fn build(&mut self, ui: &mut eframe::egui::Ui) {
        ui.centered_and_justified(|ui| {
            ui.label("Building...");
        });
    }
}
