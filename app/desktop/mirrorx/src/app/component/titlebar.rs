use eframe::{egui::*, Frame};

use crate::app::widget::peer_connect::PeerConnectWidget;

pub struct TitleBar {
    close_button: TitleBarControlButton,
    min_button: TitleBarControlButton,
    peer_connect_editor: PeerConnectWidget,
}

impl TitleBar {
    pub fn new() -> Self {
        Self {
            close_button: TitleBarControlButton::new(true),
            min_button: TitleBarControlButton::new(false),
            peer_connect_editor: PeerConnectWidget::new(),
        }
    }

    pub fn draw(&mut self, ui: &mut Ui, frame: &mut Frame) {
        let (rect, response) = ui.allocate_at_least(
            Vec2 {
                x: ui.available_width(),
                y: 40.0,
            },
            Sense::click(),
        );

        ui.painter()
            .rect_filled(rect, Rounding::none(), Color32::from_rgb(47, 49, 53));

        // connect input
        let peer_id_rect = Rect::from_min_size(rect.min + vec2(8.0, 6.0), vec2(300.0, 28.0));

        ui.allocate_ui_at_rect(peer_id_rect, |ui| {
            ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                self.peer_connect_editor.draw(ui);
            });
        });

        // control buttons
        let control_buttons_rect = Rect::from_min_max(
            pos2(ui.ctx().screen_rect().right() - 60.0, 0.0),
            pos2(ui.ctx().screen_rect().right(), 40.0),
        );

        ui.allocate_ui_at_rect(control_buttons_rect, |ui| {
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                ui.style_mut().spacing.item_spacing = vec2(0.0, 0.0);
                self.close_button.draw(ui, frame);
                self.min_button.draw(ui, frame);
            });
        });

        if response.is_pointer_button_down_on() {
            frame.drag_window();
        }
    }
}

pub struct TitleBarControlButton {
    background_anim_id: Id,
    foreground_anim_id: Id,
    is_close_button: bool,
}

impl TitleBarControlButton {
    pub fn new(is_close_button: bool) -> Self {
        Self {
            background_anim_id: Id::new(uuid::Uuid::new_v4()),
            foreground_anim_id: Id::new(uuid::Uuid::new_v4()),
            is_close_button,
        }
    }

    pub fn draw(&mut self, ui: &mut Ui, frame: &mut Frame) {
        let (rect, response) = ui.allocate_at_least(vec2(40.0, 40.0), Sense::click());

        let hovered = response.hovered();

        let background_anim_progress = ui.ctx().animate_value_with_time(
            self.background_anim_id,
            if hovered { 1.0 } else { 0.0 },
            0.2,
        );

        let foreground_anim_progress = ui.ctx().animate_value_with_time(
            self.foreground_anim_id,
            if hovered { 1.0 } else { 0.1 },
            0.2,
        );

        // background
        ui.painter().rect_filled(
            rect,
            Rounding::none(),
            if self.is_close_button {
                Color32::from_rgba_unmultiplied(
                    211,
                    51,
                    40,
                    (255.0 * background_anim_progress) as u8,
                )
            } else {
                Color32::from_rgba_unmultiplied(
                    95,
                    95,
                    95,
                    (255.0 * background_anim_progress) as u8,
                )
            },
        );

        // "x" or "-" font

        let galley = ui.painter().layout(
            if self.is_close_button {
                String::from("\u{e5cd}")
            } else {
                String::from("\u{e15b}")
            },
            FontId::proportional(24.0),
            Color32::from_rgba_unmultiplied(
                255,
                255,
                255,
                (255.0 * foreground_anim_progress) as u8,
            ),
            0.0,
        );

        let font_rect = Rect::from_center_size(rect.center(), galley.size());
        ui.painter().galley(font_rect.min, galley);

        if response.clicked() {
            if self.is_close_button {
                frame.close()
            } else {
                frame.set_minimized(true);
            }
        }
    }
}
