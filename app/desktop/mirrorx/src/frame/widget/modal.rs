use eframe::{egui::*, epaint::*};

pub struct Modal {
    id: Id,
    ctx: Context,
}

impl Modal {
    pub fn new(id: &str, ctx: &Context) -> Self {
        let id = Id::new(id);
        Self {
            id,
            ctx: ctx.clone(),
        }
    }

    pub fn open(&self) {
        self.ctx.memory_mut(|mem| mem.open_popup(self.id));
    }

    pub fn close(&self) {
        let is_open = self.ctx.memory(|mem| mem.is_popup_open(self.id));
        if is_open {
            self.ctx.memory_mut(|mem| mem.toggle_popup(self.id));
        }
    }

    pub fn show(
        &self,
        title: &str,
        draw_content_fn: impl FnOnce(&mut Ui),
        button_handle_fn: &[(&str, &dyn Fn())],
    ) {
        let is_open = self.ctx.memory(|mem| mem.is_popup_open(self.id));
        if !is_open {
            return;
        }

        Area::new(self.id)
            .interactable(true)
            .fixed_pos(Pos2::ZERO)
            .order(Order::Background)
            .show(&self.ctx, |ui| {
                let overlay_color = Color32::from_rgba_unmultiplied(18, 18, 18, 180);
                let rect = ui.ctx().screen_rect();
                ui.allocate_response(rect.size(), Sense::hover());
                let inner = ui.allocate_ui_at_rect(rect, |ui| {
                    ui.painter()
                        .add(Shape::rect_filled(rect, 0.0, overlay_color));

                    Window::new(title)
                        .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
                        .collapsible(false)
                        .movable(false)
                        .resizable(false)
                        .show(ui.ctx(), |ui| {
                            draw_content_fn(ui);

                            if button_handle_fn.is_empty() {
                                return;
                            }

                            ui.separator();

                            let inner_width = ui.available_width();

                            let (rect, _) =
                                ui.allocate_exact_size(vec2(inner_width, 34.0), Sense::click());

                            let space = 6.0;

                            let button_width = if button_handle_fn.len() == 1 {
                                inner_width
                            } else {
                                (inner_width - ((button_handle_fn.len() - 1) as f32) * space)
                                    / (button_handle_fn.len() as f32)
                            };

                            ui.allocate_ui_at_rect(rect, |ui| {
                                let mut min = ui.next_widget_position() + vec2(0.0, 1.0);

                                for (i, (name, handler)) in button_handle_fn.iter().enumerate() {
                                    let rect = Rect::from_min_size(min, vec2(button_width, 32.0));

                                    ui.allocate_ui_at_rect(rect, |ui| {
                                        ui.centered_and_justified(|ui| {
                                            if ui.add(Button::new(*name)).clicked() {
                                                handler();
                                            }
                                        });
                                    });

                                    if i != button_handle_fn.len() - 1 {
                                        min += vec2(button_width + space, 0.0);
                                    }
                                }
                            });
                        });
                });
                inner.response
            });
    }
}
