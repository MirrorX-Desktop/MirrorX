use crate::asset::IMAGE_LOGO;
use eframe::{egui::*, epaint::*};

#[derive(Debug, Hash, PartialEq, Eq, Copy)]
pub enum NavButtonType {
    Home,
    Lan,
    History,
    Settings,
}

impl Clone for NavButtonType {
    fn clone(&self) -> Self {
        match self {
            Self::Home => Self::Home,
            Self::Lan => Self::Lan,
            Self::History => Self::History,
            Self::Settings => Self::Settings,
        }
    }
}

pub struct NavBar {
    select_nav_button_type: NavButtonType,
    nav_buttons: Vec<NavButton>,
}

impl NavBar {
    pub fn new() -> Self {
        Self {
            select_nav_button_type: NavButtonType::Home,
            nav_buttons: vec![
                NavButton::new(NavButtonType::Home),
                NavButton::new(NavButtonType::Lan),
                NavButton::new(NavButtonType::History),
                NavButton::new(NavButtonType::Settings),
            ],
        }
    }
    pub fn draw(&mut self, ui: &mut eframe::egui::Ui) {
        let rect = Rect::from_x_y_ranges(0f32..=64f32, 0f32..=ui.available_height());

        ui.painter()
            .rect_filled(rect, Rounding::none(), Color32::from_rgb(31, 32, 35));

        // disable tooltip shadow
        let mut visuals = ui.style_mut().visuals.clone();
        visuals.popup_shadow = Shadow::NONE;
        ui.ctx().set_visuals(visuals);

        ui.allocate_ui_at_rect(rect, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(11.0);
                ui.style_mut().spacing.item_spacing = vec2(0.0, 8.0);
                for button in self.nav_buttons.iter_mut() {
                    if button.draw(ui, self.select_nav_button_type).clicked() {
                        self.select_nav_button_type = button.button_type;
                    };
                }
            });
        });
    }
}

pub struct NavButton {
    button_type: NavButtonType,
    background_anim_id: Id,
    foreground_anim_id: Id,
    indicator_anim_id: Id,
}

impl NavButton {
    pub fn new(button_type: NavButtonType) -> Self {
        Self {
            button_type,
            background_anim_id: Id::new(uuid::Uuid::new_v4()),
            foreground_anim_id: Id::new(uuid::Uuid::new_v4()),
            indicator_anim_id: Id::new(uuid::Uuid::new_v4()),
        }
    }

    pub fn draw(
        &mut self,
        ui: &mut eframe::egui::Ui,
        selected_button_type: NavButtonType,
    ) -> Response {
        let (rect, response) = ui.allocate_at_least(vec2(42.0, 42.0), Sense::click());

        let response = response.on_hover_ui_at_pointer(|ui| {
            let tooltip_str = match self.button_type {
                NavButtonType::Home => rust_i18n::t!("tooltip.nav.home"),
                NavButtonType::Lan => rust_i18n::t!("tooltip.nav.lan"),
                NavButtonType::History => rust_i18n::t!("tooltip.nav.history"),
                NavButtonType::Settings => rust_i18n::t!("tooltip.nav.settings"),
            };

            ui.colored_label(Color32::WHITE, tooltip_str);
        });

        if response.hovered() {
            ui.ctx()
                .set_cursor_icon(eframe::egui::CursorIcon::PointingHand);
        }

        let selected = selected_button_type == self.button_type;

        let background_anim_progress = ui.ctx().animate_value_with_time(
            self.background_anim_id,
            if selected {
                0.04
            } else if response.hovered() {
                0.01
            } else {
                0.0
            },
            0.2,
        );

        let foreground_anim_progress = ui.ctx().animate_value_with_time(
            self.foreground_anim_id,
            if selected {
                1.0
            } else if response.hovered() {
                0.4
            } else {
                0.1
            },
            0.2,
        );

        let indicator_anim_progress =
            ui.ctx()
                .animate_bool_with_time(self.indicator_anim_id, selected, 0.2);

        // background
        ui.painter().rect_filled(
            rect,
            Rounding::same(8.0),
            Color32::from_rgba_unmultiplied(
                108,
                108,
                108,
                (255.0 * background_anim_progress) as u8,
            ),
        );

        let icon_code = match self.button_type {
            NavButtonType::Home => None,
            NavButtonType::Lan => Some("\u{eb2f}"),
            NavButtonType::History => Some("\u{f17d}"),
            NavButtonType::Settings => Some("\u{e429}"),
        };

        // foreground
        if let Some(icon_code) = icon_code {
            let galley = ui.painter().layout(
                icon_code.to_string(),
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
        } else {
            ui.painter().image(
                IMAGE_LOGO.texture_id(ui.ctx()),
                rect.shrink(3.0),
                Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)),
                Color32::from_rgba_unmultiplied(
                    255,
                    255,
                    255,
                    (255.0 * foreground_anim_progress) as u8,
                ),
            );
        }

        // indicator
        let indicator_min = rect.right_top() + vec2(5.0, 5.0);
        let indicator_max = indicator_min + vec2(6.0, 32.0);

        ui.painter().rect_filled(
            Rect::from_two_pos(indicator_min, indicator_max),
            Rounding {
                nw: 16.0,
                ne: 0.0,
                sw: 16.0,
                se: 0.0,
            },
            Color32::from_rgba_unmultiplied(100, 205, 252, (255.0 * indicator_anim_progress) as u8),
        );

        response
    }
}
