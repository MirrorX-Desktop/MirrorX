use crate::frame::{
    asset::StaticImageCache,
    state::{PageType, UIState},
};
use eframe::egui::*;

pub struct NavBar {
    nav_buttons: Vec<NavButton>,
}

impl NavBar {
    pub fn new() -> Self {
        Self {
            nav_buttons: vec![
                NavButton::new(PageType::Device),
                NavButton::new(PageType::Lan),
                NavButton::new(PageType::History),
                NavButton::new(PageType::Settings),
            ],
        }
    }

    pub fn draw(&mut self, ui: &mut eframe::egui::Ui, ui_state: &mut UIState) {
        let rect = Rect::from_min_size(Pos2::ZERO, vec2(216.0, ui.available_height()));

        ui.painter()
            .rect_filled(rect, Rounding::none(), ui_state.theme_color.background_body);

        ui.image(
            StaticImageCache::current().logo.texture_id(ui.ctx()),
            vec2(64.0, 64.0),
        );

        ui.allocate_ui_at_rect(rect, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(11.0);
                ui.style_mut().spacing.item_spacing = vec2(0.0, 8.0);
                for button in self.nav_buttons.iter_mut() {
                    if button.draw(ui, ui_state).clicked() {
                        ui_state.current_page_type = button.button_type.clone();
                    };
                }
            });
        });
    }
}

pub struct NavButton {
    button_type: PageType,
    indicator_anim_id: Id,
}

impl NavButton {
    pub fn new(page_type: PageType) -> Self {
        Self {
            button_type: page_type,
            indicator_anim_id: Id::new(uuid::Uuid::new_v4()),
        }
    }

    pub fn draw(&mut self, ui: &mut eframe::egui::Ui, ui_state: &mut UIState) -> Response {
        let (rect, response) = ui.allocate_at_least(vec2(42.0, 42.0), Sense::click());

        let tooltip_str = match self.button_type {
            PageType::Device => rust_i18n::t!("tooltip.nav.home"),
            PageType::Lan => rust_i18n::t!("tooltip.nav.lan"),
            PageType::History => rust_i18n::t!("tooltip.nav.history"),
            PageType::Settings => rust_i18n::t!("tooltip.nav.settings"),
        };

        let response = response.on_hover_text_at_pointer(RichText::new(tooltip_str));

        if response.hovered() {
            ui.ctx()
                .set_cursor_icon(eframe::egui::CursorIcon::PointingHand);
        }

        let selected = ui_state.current_page_type.eq(&self.button_type);

        let background_color = if selected {
            ui_state.theme_color.primary_plain_active_bg
        } else if response.hovered() {
            ui_state.theme_color.primary_plain_hover_bg
        } else {
            Color32::TRANSPARENT
        };

        let indicator_anim_progress =
            ui.ctx()
                .animate_bool_with_time(self.indicator_anim_id, selected, 0.2);

        // background
        ui.painter()
            .rect_filled(rect, Rounding::same(8.0), background_color);

        // foreground
        let (icon_image, shrink) = match self.button_type {
            PageType::Device => (&StaticImageCache::current().logo_1024, 4.0),
            PageType::Lan => (&StaticImageCache::current().lan_48, 8.0),
            PageType::History => (&StaticImageCache::current().history_toggle_off_48, 8.0),
            PageType::Settings => (&StaticImageCache::current().tune_48, 8.0),
        };

        ui.painter().image(
            icon_image.texture_id(ui.ctx()),
            rect.shrink(shrink),
            Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)),
            ui_state.theme_color.primary_plain_color,
        );

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
            Color32::from_rgba_unmultiplied(
                ui_state.theme_color.primary_400.r(),
                ui_state.theme_color.primary_400.g(),
                ui_state.theme_color.primary_400.b(),
                (255.0 * indicator_anim_progress) as u8,
            ),
        );

        response
    }
}
