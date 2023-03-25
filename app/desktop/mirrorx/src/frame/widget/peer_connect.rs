use crate::frame::{
    asset::StaticImageCache,
    state::{ConnectType, UIState},
};
use eframe::egui::*;

pub struct PeerConnectWidget {
    resource_type_dropdown_id: Id,
    domain_dropdown_id: Id,
    resource_type_hovered: bool,
    domain_hovered: bool,
}

impl Default for PeerConnectWidget {
    fn default() -> Self {
        Self {
            resource_type_dropdown_id: Id::new(uuid::Uuid::new_v4()),
            domain_dropdown_id: Id::new(uuid::Uuid::new_v4()),
            resource_type_hovered: false,
            domain_hovered: false,
        }
    }
}

impl PeerConnectWidget {
    pub fn draw(&mut self, ui: &mut Ui, ui_state: &mut UIState) {
        let button_response = self.draw_connect_button(ui, ui_state);

        ui.add(Separator::default().spacing(1.0));

        let domain_response = self.draw_domain_dropdown(ui, ui_state);

        let peer_id_input_response = ui.add(
            TextEdit::singleline(&mut ui_state.peer_connect_content)
                .font(FontId::proportional(16.0))
                .frame(false)
                .desired_width(220.0)
                .margin(vec2(4.0, 0.0))
                .vertical_align(Align::Center)
                .hint_text("Peer ID"),
        );

        let peer_id_input_response =
            peer_id_input_response.on_hover_text("Input the peer id you want to connect");

        ui.add(Separator::default().spacing(1.0));

        let resource_type_response = self.draw_resource_type_dropdown(ui, ui_state);

        let union_response = resource_type_response
            .union(peer_id_input_response)
            .union(domain_response)
            .union(button_response);

        ui.painter().rect(
            union_response.rect,
            Rounding::same(3.0),
            Color32::TRANSPARENT,
            Stroke {
                width: 1.0,
                color: ui.style().visuals.widgets.noninteractive.bg_stroke.color,
            },
        );
    }

    fn draw_resource_type_dropdown(&mut self, ui: &mut Ui, ui_state: &mut UIState) -> Response {
        // connect type label
        let resource_image = match ui_state.connect_type {
            crate::frame::state::ConnectType::Desktop => {
                &StaticImageCache::current().desktop_windows_48
            }
            crate::frame::state::ConnectType::Files => &StaticImageCache::current().folder_48,
        };

        let color = if self.resource_type_hovered {
            ui_state.theme_color.primary_plain_color
        } else {
            ui_state.theme_color.text_primary
        };

        // dropdown expand button
        let dropdown_response = ImageButton::new(
            StaticImageCache::current()
                .expand_more_48
                .texture_id(ui.ctx()),
            vec2(16.0, 16.0),
        )
        .tint(color)
        .frame(false)
        .ui(ui);

        // resource type image
        let mut image_response = ui.add(
            Image::new(resource_image.texture_id(ui.ctx()), vec2(18.0, 18.0))
                .tint(color)
                .sense(Sense::click()),
        );

        // add space on resource type image left
        image_response.rect.min.x -= 4.0;

        ui.add_space(100.0);

        let mut union_response = image_response.union(dropdown_response);
        self.resource_type_hovered = union_response.hovered();

        let popup_id = ui.make_persistent_id(self.resource_type_dropdown_id);
        if union_response.clicked() {
            ui.memory_mut(|mem| mem.toggle_popup(popup_id));
        }

        // dropdown list
        popup::popup_above_or_below_widget(
            ui,
            popup_id,
            &union_response,
            AboveOrBelow::Above,
            |ui| {
                ui.set_min_width(100.0);
                if ui
                    .selectable_label(
                        ui_state.connect_type.eq(&ConnectType::Desktop),
                        RichText::new("\u{e30c} Desktop".to_string()),
                    )
                    .clicked()
                {
                    ui_state.connect_type = ConnectType::Desktop;
                }

                if ui
                    .selectable_label(
                        ui_state.connect_type.eq(&ConnectType::Files),
                        RichText::new("\u{e2c7} Files".to_string()),
                    )
                    .clicked()
                {
                    ui_state.connect_type = ConnectType::Files;
                }
            },
        );

        if !ui.memory(|mem| mem.is_popup_open(popup_id)) {
            union_response = union_response.on_hover_text("Choose control type");
        }

        union_response
    }

    fn draw_domain_dropdown(&mut self, ui: &mut Ui, ui_state: &mut UIState) -> Response {
        // hash tag and domain name

        let color = if self.domain_hovered {
            ui_state.theme_color.primary_plain_color
        } else {
            ui_state.theme_color.text_primary
        };

        let dropdown_response = ImageButton::new(
            StaticImageCache::current()
                .expand_more_48
                .texture_id(ui.ctx()),
            vec2(16.0, 16.0),
        )
        .tint(color)
        .frame(false)
        .ui(ui);

        let label_response = Label::new(
            RichText::new(format!("#{}", ui_state.peer_domain))
                .font(FontId::proportional(16.0))
                .color(color),
        )
        .sense(Sense::click())
        .ui(ui);

        let mut union_response = label_response.union(dropdown_response);
        self.domain_hovered = union_response.hovered();

        let popup_id = ui.make_persistent_id(self.domain_dropdown_id);
        if union_response.clicked() {
            ui.memory_mut(|mem| mem.toggle_popup(popup_id));
        }

        // dropdown list
        popup::popup_above_or_below_widget(
            ui,
            popup_id,
            &union_response,
            AboveOrBelow::Above,
            |ui| {
                ui.set_min_width(200.0);
                ui.label("Some more info, or things you can select:");
                ui.label("…");
            },
        );

        if !ui.memory(|mem| mem.is_popup_open(popup_id)) {
            union_response = union_response.on_hover_text("Choose peer domain");
        }

        union_response
    }

    fn draw_connect_button(&mut self, ui: &mut Ui, ui_state: &mut UIState) -> Response {
        let (rect, response) =
            ui.allocate_at_least(Vec2::splat(ui.available_height()), Sense::click());

        let (bg_color, fg_color) = if response.hovered() {
            (
                ui_state.theme_color.primary_plain_active_bg,
                ui_state.theme_color.primary_plain_color,
            )
        } else {
            (Color32::TRANSPARENT, ui_state.theme_color.text_primary)
        };

        ui.painter().rect_filled(
            rect,
            Rounding {
                nw: 0.0,
                ne: 3.0,
                sw: 0.0,
                se: 3.0,
            },
            bg_color,
        );

        ui.painter().image(
            StaticImageCache::current()
                .arrow_forward_48
                .texture_id(ui.ctx()),
            rect.shrink(4.0),
            Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)),
            fg_color,
        );

        response
            .on_hover_text("Click to connect peer")
            .on_hover_cursor(CursorIcon::PointingHand)
    }
}
