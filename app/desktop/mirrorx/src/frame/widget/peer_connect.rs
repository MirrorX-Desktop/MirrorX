use crate::frame::asset::StaticImageCache;
use eframe::egui::*;

pub struct PeerConnectWidget {
    resource_type_is_desktop: bool,
    resource_type_dropdown_id: Id,
    domain_dropdown_id: Id,
    edit_content: String,
    resource_type_hovered: bool,
    domain_hovered: bool,
}

impl PeerConnectWidget {
    pub fn new() -> Self {
        Self {
            resource_type_is_desktop: true,
            resource_type_dropdown_id: Id::new(uuid::Uuid::new_v4()),
            domain_dropdown_id: Id::new(uuid::Uuid::new_v4()),
            edit_content: String::default(),
            resource_type_hovered: false,
            domain_hovered: false,
        }
    }

    pub fn draw(&mut self, ui: &mut Ui) {
        let inner = ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
            ui.add_space(6.0);
            ui.style_mut()
                .visuals
                .widgets
                .noninteractive
                .bg_stroke
                .color = Color32::from_rgb(101, 101, 101);

            let resource_type_response = self.draw_resource_type_dropdown(ui);

            ui.separator();

            let peer_id_input_response = ui.add(
                TextEdit::singleline(&mut self.edit_content)
                    .font(FontId::proportional(16.0))
                    .frame(false)
                    .desired_width(120.0)
                    .clip_text(false)
                    .margin(vec2(4.0, 0.0))
                    .vertical_align(Align::Center)
                    .horizontal_align(Align::Max),
            );

            let peer_id_input_response = peer_id_input_response.on_hover_text(
                RichText::new("Input the peer id you want to connect").color(Color32::WHITE),
            );

            let domain_response = self.draw_domain_dropdown(ui);

            ui.add(Separator::default().spacing(0.0));

            let button_response = self.draw_connect_button(ui);

            resource_type_response
                .union(peer_id_input_response)
                .union(domain_response)
                .union(button_response)
        });

        ui.painter().rect(
            inner.response.rect,
            ui.style().visuals.widgets.inactive.rounding,
            Color32::TRANSPARENT,
            Stroke {
                width: 1.0,
                color: Color32::from_rgb(101, 101, 101),
            },
        );
    }

    fn draw_resource_type_dropdown(&mut self, ui: &mut Ui) -> Response {
        // connect type label
        let resource_image = if self.resource_type_is_desktop {
            &StaticImageCache::current().desktop_windows_48
        } else {
            &StaticImageCache::current().folder_48
        };

        let color = if self.resource_type_hovered {
            ui.style().visuals.widgets.active.fg_stroke.color
        } else {
            ui.style().visuals.widgets.noninteractive.fg_stroke.color
        };

        let image_response = ui.add(
            Image::new(resource_image.texture_id(ui.ctx()), vec2(16.0, 16.0))
                .tint(color)
                .sense(Sense::click()),
        );

        // dropdown button
        let dropdown_response = ImageButton::new(
            StaticImageCache::current()
                .expand_more_48
                .texture_id(ui.ctx()),
            vec2(16.0, 16.0),
        )
        .tint(color)
        .frame(false)
        .ui(ui);

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
            AboveOrBelow::Below,
            |ui| {
                ui.set_min_width(100.0);
                if ui
                    .selectable_label(
                        self.resource_type_is_desktop,
                        RichText::new("\u{e30c} Desktop".to_string()),
                    )
                    .clicked()
                {
                    self.resource_type_is_desktop = true;
                }

                if ui
                    .selectable_label(
                        !self.resource_type_is_desktop,
                        RichText::new("\u{e2c7} Files".to_string()),
                    )
                    .clicked()
                {
                    self.resource_type_is_desktop = false;
                }
            },
        );

        if !ui.memory(|mem| mem.is_popup_open(popup_id)) {
            union_response = union_response
                .on_hover_text(RichText::new("Choose control type").color(Color32::WHITE));
        }

        union_response
    }

    fn draw_domain_dropdown(&mut self, ui: &mut Ui) -> Response {
        // hash tag and domain name

        let color = if self.domain_hovered {
            ui.style().visuals.widgets.active.fg_stroke.color
        } else {
            ui.style().visuals.widgets.noninteractive.fg_stroke.color
        };

        let label_response = Label::new(
            RichText::new("#mirrorx.cloud")
                .font(FontId::proportional(16.0))
                .color(color),
        )
        .sense(Sense::click())
        .ui(ui);

        // dropdown button
        let dropdown_response = ImageButton::new(
            StaticImageCache::current()
                .expand_more_48
                .texture_id(ui.ctx()),
            vec2(16.0, 16.0),
        )
        .tint(color)
        .frame(false)
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
            AboveOrBelow::Below,
            |ui| {
                ui.set_min_width(200.0);
                ui.label("Some more info, or things you can select:");
                ui.label("…");
            },
        );

        if !ui.memory(|mem| mem.is_popup_open(popup_id)) {
            union_response = union_response
                .on_hover_text(RichText::new("Choose peer domain").color(Color32::WHITE));
        }

        union_response
    }

    fn draw_connect_button(&mut self, ui: &mut Ui) -> Response {
        let (rect, response) = ui.allocate_at_least(vec2(28.0, 28.0), Sense::click());

        let (bg_color, fg_color) = if response.hovered() {
            (
                ui.style().visuals.widgets.active.bg_fill,
                ui.style().visuals.widgets.active.fg_stroke.color,
            )
        } else {
            (
                Color32::TRANSPARENT,
                ui.style().visuals.widgets.noninteractive.fg_stroke.color,
            )
        };

        ui.painter()
            .rect_filled(rect.shrink2(vec2(1.0, 0.0)), Rounding::none(), bg_color);
        ui.painter().image(
            StaticImageCache::current()
                .arrow_forward_48
                .texture_id(ui.ctx()),
            rect.shrink(4.0),
            Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)),
            fg_color,
        );

        response.on_hover_text(RichText::new("Click to connect peer").color(Color32::WHITE))
    }
}
