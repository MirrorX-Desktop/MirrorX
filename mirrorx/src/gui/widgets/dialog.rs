use eframe::{
    egui::{style::Margin, Frame, Ui},
    epaint::{Color32, Pos2, Rounding, Shadow, Stroke, Vec2},
};

pub struct Dialog {
    title: String,
    size: Vec2,
}

impl Dialog {
    pub fn new(title: &str, size: Vec2) -> Self {
        Self {
            title: title.to_string(),
            size,
        }
    }
    pub fn show<R>(&mut self, ui: &mut Ui, add_contents: impl FnOnce(&mut Ui) -> R) {
        let frame = Frame::default()
            .inner_margin(Margin {
                left: 0.0,
                right: 0.0,
                top: 4.0,
                bottom: 0.0,
            })
            .stroke(Stroke::new(1.0, Color32::GRAY))
            .rounding(Rounding::same(2.0))
            .fill(Color32::WHITE)
            .shadow(Shadow::small_light());

        eframe::egui::Window::new(&self.title)
            .frame(frame)
            .fixed_size(self.size)
            .fixed_pos(Pos2::new(
                (380.0 - self.size.x) / 2.0,
                (630.0 - self.size.y) / 2.0 - 10.0,
            ))
            .collapsible(false)
            .resizable(false)
            .title_bar(false)
            .show(ui.ctx(), add_contents);
    }
}
