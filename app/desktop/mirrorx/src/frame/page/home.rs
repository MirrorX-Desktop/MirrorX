use super::Page;
use eframe::{egui::*, emath::*, epaint::*};

pub struct HomePage {}

impl HomePage {
    pub fn new() -> Self {
        Self {}
    }

    fn draw_status_bar(&mut self, ui: &mut Ui) {
        let (rect, _) = ui.allocate_at_least(
            vec2(60.0, 30.0),
            Sense {
                click: false,
                drag: false,
                focusable: false,
            },
        );

        ui.allocate_ui_at_rect(
            Rect::from_min_size(rect.min, vec2(rect.width() - 8.0, rect.height())),
            |ui| {
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.label(RichText::new("\u{e1d8}").font(FontId::proportional(18.0)));
                });
            },
        );
    }

    fn draw_panel(&mut self, ui: &mut Ui) {
        let (rect, _) = ui.allocate_exact_size(
            vec2(760.0, 440.0),
            Sense {
                click: false,
                drag: false,
                focusable: false,
            },
        );

        ui.allocate_ui_at_rect(rect, |ui| {
            // ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
            // ui.allocate_ui_at_rect(
            //     Rect::from_min_size(
            //         ui.available_rect_before_wrap().min,
            //         vec2(ui.available_width() / 2.0, ui.available_height()),
            //     ),
            //     |ui| {
            //         ui.painter().rect_filled(
            //             ui.available_rect_before_wrap(),
            //             Rounding::none(),
            //             Color32::RED,
            //         );
            //     },
            // );

            // ui.allocate_ui_at_rect(
            //     Rect::from_min_size(
            //         ui.available_rect_before_wrap().min + vec2(ui.available_width() / 2.0, 0.0),
            //         vec2(ui.available_width() / 2.0, ui.available_height()),
            //     ),
            //     |ui| {
            //         ui.painter().rect_filled(
            //             ui.available_rect_before_wrap(),
            //             Rounding::none(),
            //             Color32::GREEN,
            //         );
            //     },
            // );
            // });
            ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                let (left_rect, _) = ui.allocate_exact_size(
                    vec2(ui.available_width() / 2.0, ui.available_height()),
                    Sense::click(),
                );

                let (separator_rect, _) =
                    ui.allocate_exact_size(vec2(1.0, ui.available_height()), Sense::click());

                let (right_rect, _) = ui.allocate_exact_size(
                    vec2(ui.available_width(), ui.available_height()),
                    Sense::click(),
                );

                ui.allocate_ui_at_rect(left_rect, |ui| {
                    self.draw_panel_left(ui);
                });

                ui.allocate_ui_at_rect(separator_rect, |ui| {
                    ui.separator();
                });

                ui.allocate_ui_at_rect(right_rect, |ui| {
                    self.draw_panel_right(ui);
                });
            });
        });
    }

    fn draw_panel_left(&mut self, ui: &mut Ui) {
        ui.centered_and_justified(|ui| {
            ui.with_layout(Layout::top_down(Align::Min), |ui| {
                ui.label(RichText::new("Peer ID").size(32.0));

                ui.add_space(18.0);
                ui.label(RichText::new("mmmm").size(18.0));

                ui.add_space(18.0);
                ui.label(RichText::new("Password").size(32.0));

                ui.add_space(18.0);
                ui.checkbox(&mut true, RichText::new("Use Time Based OTP").size(18.0));

                ui.add_space(18.0);
                ui.vertical_centered(|ui| {
                    ui.label(RichText::new("F G I 2 X L").size(24.0));
                });

                ui.add_space(18.0);
                ui.checkbox(&mut true, RichText::new("Use One-Time Password").size(18.0));

                ui.add_space(18.0);
                ui.vertical_centered(|ui| {
                    ui.label(RichText::new("F G I 2 X L").size(24.0));
                });

                ui.add_space(18.0);
                ui.checkbox(
                    &mut true,
                    RichText::new("Use Permanent Password").size(18.0),
                );

                ui.add_space(18.0);
                ui.vertical_centered(|ui| {
                    ui.text_edit_singleline(&mut "aaaa");
                });
            })
        });
    }

    fn draw_panel_right(&mut self, ui: &mut Ui) {
        ui.label("B");
    }
}

impl Page for HomePage {
    fn draw(&mut self, ui: &mut eframe::egui::Ui) {
        ui.with_layout(
            Layout::bottom_up(Align::Center).with_cross_justify(true),
            |ui| {
                ui.style_mut().spacing.item_spacing = Vec2::ZERO;

                // status bar
                ui.allocate_ui(vec2(0.0, 0.0), |ui| self.draw_status_bar(ui));

                ui.add(Separator::default().spacing(0.0));

                // panel
                let (panel_rect, _) = ui.allocate_exact_size(
                    ui.available_size(),
                    Sense {
                        click: false,
                        drag: false,
                        focusable: false,
                    },
                );

                ui.allocate_ui_at_rect(panel_rect, |ui| {
                    ui.centered_and_justified(|ui| {
                        self.draw_panel(ui);
                    });
                });
            },
        );
    }
}
