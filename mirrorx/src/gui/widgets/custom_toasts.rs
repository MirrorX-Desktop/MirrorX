use egui::{style::Margin, *};
use egui_toast::{Toast, ToastKind, ToastOptions, Toasts};

pub struct CustomToasts {
    inner_toasts: Toasts,
}

impl CustomToasts {
    pub fn new() -> Self {
        let inner_toasts = Toasts::new()
            .anchor((380.0 - 8.0, 8.0)) // top-right corner with same offset
            .direction(Direction::TopDown)
            .align_to_end(true)
            .custom_contents(ToastKind::Custom(0), custom_toast_contents);

        Self { inner_toasts }
    }

    pub fn error(&mut self, content: &str) {
        self.inner_toasts.add(Toast {
            kind: ToastKind::Custom(0),
            text: content.into(),
            options: ToastOptions::default(),
        });
    }

    pub fn show(&mut self, ctx: &Context) {
        self.inner_toasts.show(ctx)
    }
}

fn custom_toast_contents(ui: &mut Ui, toast: &mut Toast) -> Response {
    egui::Frame::default()
        .fill(Color32::BLACK)
        .inner_margin(Margin::same(8.0))
        .rounding(2.0)
        .show(ui, |ui| {
            let text_galley = toast.text.clone().color(Color32::WHITE).into_galley(
                ui,
                None,
                180.0,
                FontId::proportional(18.0),
            );

            let text_size = text_galley.size();

            let (rect, response) = ui.allocate_exact_size(
                Vec2::new(text_galley.size().x.min(180.0), text_size.y),
                Sense::focusable_noninteractive(),
            );

            ui.allocate_ui_at_rect(rect, |ui| {
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.scope(|ui| {
                        ui.visuals_mut().button_frame = false;
                        ui.visuals_mut().clip_rect_margin = 0.0;
                        if ui
                            .button(
                                RichText::new("❌")
                                    .color(Color32::WHITE)
                                    .font(FontId::proportional(24.0)),
                            )
                            .clicked()
                        {
                            toast.close();
                        }
                    });

                    let (rect, response) = ui
                        .allocate_exact_size(text_galley.size(), Sense::focusable_noninteractive());

                    ui.painter().add(epaint::TextShape {
                        pos: rect.left_top(),
                        galley: text_galley.galley,
                        underline: Stroke::none(),
                        override_text_color: None,
                        angle: 0.0,
                    });

                    ui.label(
                        RichText::new("⛔")
                            .color(Color32::RED)
                            .font(FontId::proportional(24.0)),
                    );

                    response
                });
            });

            response
        })
        .response
}
