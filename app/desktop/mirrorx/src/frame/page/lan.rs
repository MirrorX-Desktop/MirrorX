use super::Page;

pub struct LanPage {}

impl LanPage {
    pub fn new() -> Self {
        Self {}
    }
}
impl Page for LanPage {
    fn draw(&mut self, ui: &mut eframe::egui::Ui) {
        ui.label("lan");
    }
}
