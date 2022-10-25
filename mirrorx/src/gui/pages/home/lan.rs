use egui::Ui;

#[derive(Default)]
pub struct LANPage {}

impl LANPage {
    pub fn new() -> Self {
        Self {}
    }
}

impl LANPage {
    pub fn show(&mut self, ui: &mut Ui) {
        ui.centered_and_justified(|ui| {
            ui.label("Building...");
        });
    }
}
