use super::component::nav::NavBar;
use crate::app::component::titlebar::TitleBar;
use eframe::egui::*;

pub struct Viewport {
    nav_bar: NavBar,
    title_bar: TitleBar,
}

impl Viewport {
    pub fn new() -> Self {
        Self {
            nav_bar: NavBar::new(),
            title_bar: TitleBar::new(),
        }
    }

    pub fn draw(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        CentralPanel::default()
            .frame(Frame {
                fill: Color32::from_rgb(35, 36, 39),
                ..Default::default()
            })
            .show(ctx, |ui| {
                ui.with_layout(
                    Layout::left_to_right(Align::Center).with_cross_justify(true),
                    |ui| {
                        ui.style_mut().spacing.item_spacing = vec2(0.0, 0.0);

                        self.nav_bar.draw(ui);
                        ui.add(Separator::default().spacing(0.0));

                        ui.with_layout(
                            Layout::top_down(Align::Center).with_cross_justify(true),
                            |ui| {
                                self.title_bar.draw(ui, frame);
                                ui.add(Separator::default().spacing(0.0));
                            },
                        );
                    },
                );
            });
    }
}
