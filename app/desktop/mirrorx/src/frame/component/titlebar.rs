use std::ops::SubAssign;

use crate::frame::{
    asset::StaticImageCache,
    state::{PageType, UIState},
    widget::peer_connect::PeerConnectWidget,
};
use eframe::{egui::*, epaint::TextShape, Frame};
use interpolation::Ease;

pub enum ControlButtonType {
    Min,
    Close,
}

pub struct TitleBar {
    close_button: TitleBarControlButton,
    min_button: TitleBarControlButton,
    peer_connect_editor: PeerConnectWidget,
    device_nav_button: TitleBarNavButton,
    lan_nav_button: TitleBarNavButton,
    history_nav_button: TitleBarNavButton,
    settings_nav_button: TitleBarNavButton,
    indicator_anim_id: Id,
    current_indicator_pos: Pos2,
    target_indicator_pos: Pos2,
}

impl TitleBar {
    pub fn new() -> Self {
        Self {
            close_button: TitleBarControlButton::new(ControlButtonType::Close),
            min_button: TitleBarControlButton::new(ControlButtonType::Min),
            peer_connect_editor: PeerConnectWidget::new(),
            device_nav_button: TitleBarNavButton::new(PageType::Device),
            lan_nav_button: TitleBarNavButton::new(PageType::Lan),
            history_nav_button: TitleBarNavButton::new(PageType::History),
            settings_nav_button: TitleBarNavButton::new(PageType::Settings),
            indicator_anim_id: Id::new(uuid::Uuid::new_v4()),
            current_indicator_pos: Pos2::ZERO,
            target_indicator_pos: Pos2::ZERO,
        }
    }

    pub fn draw_menu(&mut self, ui: &mut Ui, frame: &mut Frame, ui_state: &mut UIState) {
        let (rect, response) = ui.allocate_exact_size(ui.available_size(), Sense::click());
        ui.allocate_ui_at_rect(rect, |ui| {
            ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                ui.style_mut().spacing.item_spacing = Vec2::ZERO;
                ui.add_space(12.0);
                self.device_nav_button.draw(ui, ui_state);
                ui.add_space(32.0);
                self.lan_nav_button.draw(ui, ui_state);
                ui.add_space(32.0);
                self.history_nav_button.draw(ui, ui_state);
                ui.add_space(32.0);
                self.settings_nav_button.draw(ui, ui_state);
                self.draw_menu_indicator(ui, ui_state);
            })
        });

        if response.is_pointer_button_down_on() {
            frame.drag_window();
        }
    }

    pub fn draw_menu_indicator(&mut self, ui: &mut Ui, ui_state: &mut UIState) {
        self.target_indicator_pos = match ui_state.current_page_type {
            PageType::Device => self.device_nav_button.center_pos,
            PageType::Lan => self.lan_nav_button.center_pos,
            PageType::History => self.history_nav_button.center_pos,
            PageType::Settings => self.settings_nav_button.center_pos,
        };

        let anim_progress = ui.ctx().animate_bool_with_time(
            self.indicator_anim_id,
            !self
                .target_indicator_pos
                .x
                .eq(&self.current_indicator_pos.x),
            0.3,
        );

        if self.target_indicator_pos.x != self.current_indicator_pos.x {
            self.current_indicator_pos.x = interpolation::lerp(
                &self.current_indicator_pos.x,
                &self.target_indicator_pos.x,
                &anim_progress.bounce_in_out(),
            );
        }

        let indicator_rect =
            Rect::from_center_size(pos2(self.current_indicator_pos.x, 40.0), vec2(30.0, 4.0));

        ui.painter().rect(
            indicator_rect,
            Rounding::same(6.0),
            ui_state.theme_color.primary_300,
            Stroke::NONE,
        );
    }

    pub fn draw(&mut self, ui: &mut Ui, frame: &mut Frame, ui_state: &mut UIState) {
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            ui.style_mut().spacing.item_spacing = Vec2::ZERO;
            ui.add_space(12.0);
            self.close_button.draw(ui, frame, ui_state);
            ui.add_space(4.0);
            self.min_button.draw(ui, frame, ui_state);
            ui.add_space(12.0);
            ui.add(Separator::default().shrink(12.0).spacing(0.0));
            ui.add_space(12.0);
            ui.label("SSSSSSSSSSSSSSSSSSS");
            self.draw_menu(ui, frame, ui_state);
        });
    }
}

pub struct TitleBarControlButton {
    foreground_anim_id: Id,
    button_type: ControlButtonType,
}

impl TitleBarControlButton {
    pub fn new(button_type: ControlButtonType) -> Self {
        Self {
            foreground_anim_id: Id::new(uuid::Uuid::new_v4()),
            button_type,
        }
    }

    pub fn draw(&mut self, ui: &mut Ui, frame: &mut Frame, ui_state: &mut UIState) {
        let (rect, response) = ui.allocate_exact_size(Vec2::splat(24.0), Sense::click());

        let hovered = response.hovered();

        let foreground_anim_progress =
            ui.ctx()
                .animate_bool_with_time(self.foreground_anim_id, hovered, 0.3);

        let image = match self.button_type {
            ControlButtonType::Min => &StaticImageCache::current().remove_48,
            ControlButtonType::Close => &StaticImageCache::current().close_48,
        };

        let (normal_color, hover_color) = match self.button_type {
            ControlButtonType::Min => (
                ui_state.theme_color.neutral_400,
                ui_state.theme_color.neutral_600,
            ),
            ControlButtonType::Close => (
                ui_state.theme_color.neutral_400,
                Color32::from_rgb(211, 51, 40),
            ),
        };

        let current_color = interpolation::lerp(
            &normal_color.to_array(),
            &hover_color.to_array(),
            &foreground_anim_progress,
        );

        ui.painter().image(
            image.texture_id(ui.ctx()),
            rect.shrink(2.0),
            Rect::from_min_max(Pos2::ZERO, pos2(1.0, 1.0)),
            Color32::from_rgba_premultiplied(
                current_color[0],
                current_color[1],
                current_color[2],
                current_color[3],
            ),
        );

        if response.clicked() {
            match self.button_type {
                ControlButtonType::Min => frame.set_minimized(true),
                ControlButtonType::Close => frame.close(),
            }
        }
    }
}

pub struct TitleBarNavButton {
    page_type: PageType,
    foreground_anim_id: Id,
    current_foreground_color: Color32,
    target_foreground_color: Color32,
    center_pos: Pos2,
}

impl TitleBarNavButton {
    pub fn new(page_type: PageType) -> Self {
        Self {
            page_type,
            foreground_anim_id: Id::new(uuid::Uuid::new_v4()),
            current_foreground_color: Color32::TRANSPARENT,
            target_foreground_color: Color32::TRANSPARENT,
            center_pos: Pos2::ZERO,
        }
    }

    pub fn draw(&mut self, ui: &mut Ui, ui_state: &mut UIState) {
        let title = match self.page_type {
            PageType::Device => "Device",
            PageType::Lan => "LAN",
            PageType::History => "History",
            PageType::Settings => "Settings",
        };

        let galley = ui.painter().layout(
            title.to_string(),
            FontId::proportional(18.0),
            self.current_foreground_color,
            f32::INFINITY,
        );

        let (rect, response) = ui.allocate_at_least(galley.size(), Sense::click());

        self.center_pos = rect.center();

        let foreground_anim_progress = ui.ctx().animate_bool_with_time(
            self.foreground_anim_id,
            !self
                .current_foreground_color
                .eq(&self.target_foreground_color),
            0.3,
        );

        let selected = ui_state.current_page_type.eq(&self.page_type);
        let color = if selected {
            ui_state.theme_color.neutral_900
        } else if response.hovered() {
            ui_state.theme_color.neutral_400
        } else {
            ui_state.theme_color.neutral_300
        };

        self.target_foreground_color = color;

        if self.target_foreground_color != self.current_foreground_color {
            let current_color_array = interpolation::lerp(
                &self.current_foreground_color.to_array(),
                &self.target_foreground_color.to_array(),
                &foreground_anim_progress.bounce_in(),
            );

            self.current_foreground_color = Color32::from_rgba_premultiplied(
                current_color_array[0],
                current_color_array[1],
                current_color_array[2],
                current_color_array[3],
            );
        }

        ui.painter().add(TextShape {
            pos: rect.min,
            galley,
            underline: Stroke::NONE,
            override_text_color: Some(self.current_foreground_color),
            angle: 0.0,
        });

        if response.on_hover_cursor(CursorIcon::PointingHand).clicked() {
            ui_state.current_page_type = self.page_type.clone();
        }
    }
}
