use egui::Ui;

pub struct HistoryPage {}

impl HistoryPage {
    pub fn new() -> Self {
        Self {}
    }
}

impl HistoryPage {
    pub fn show(&mut self, ui: &mut Ui) {
        ui.centered_and_justified(|ui| {
            ui.label("Building...");
        });
    }
}
