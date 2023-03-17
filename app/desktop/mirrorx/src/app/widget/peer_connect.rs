use eframe::egui::*;

const ICON_DESKTOP: &str = "\u{e30c}";
const ICON_FILE_FOLDER: &str = "\u{e2c7}";

pub struct PeerConnectWidget {
    resource_type_is_desktop: bool,
    resource_type_dropdown_id: Id,
    domain_dropdown_id: Id,
    edit_content: String,
}

impl PeerConnectWidget {
    pub fn new() -> Self {
        Self {
            resource_type_is_desktop: true,
            resource_type_dropdown_id: Id::new(uuid::Uuid::new_v4()),
            domain_dropdown_id: Id::new(uuid::Uuid::new_v4()),
            edit_content: String::default(),
        }
    }

    pub fn draw(&mut self, ui: &mut Ui) {
        let inner = ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
            ui.add_space(6.0);

            let resource_type_response = self.draw_resource_type_dropdown(ui);

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

            resource_type_response
                .union(peer_id_input_response)
                .union(domain_response)
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

        ui.add_space(8.0);
        let connect_to_peer_response =
            Button::new(RichText::new("\u{e5c8}").font(FontId::proportional(16.0)))
                .min_size(vec2(28.0, 28.0))
                .stroke(Stroke {
                    width: 1.0,
                    color: Color32::from_rgb(101, 101, 101),
                })
                .ui(ui)
                .on_hover_text(RichText::new("Click to connect peer").color(Color32::WHITE));

        if connect_to_peer_response.clicked() {}
    }

    fn draw_resource_type_dropdown(&mut self, ui: &mut Ui) -> Response {
        // connect type label
        let label_response = Label::new(
            RichText::new(if self.resource_type_is_desktop {
                ICON_DESKTOP
            } else {
                ICON_FILE_FOLDER
            })
            .font(FontId::proportional(16.0)),
        )
        .sense(Sense::click())
        .ui(ui);

        // dropdown button
        let dropdown_response =
            Button::new(RichText::new("\u{e5cf}").font(FontId::proportional(16.0)))
                .frame(false)
                .ui(ui);

        ui.style_mut()
            .visuals
            .widgets
            .noninteractive
            .bg_stroke
            .color = Color32::from_rgb(101, 101, 101);
        ui.separator();

        let popup_id = ui.make_persistent_id(self.resource_type_dropdown_id);
        if label_response.clicked() || dropdown_response.clicked() {
            ui.memory_mut(|mem| mem.toggle_popup(popup_id));
        }

        let mut union_response = label_response.union(dropdown_response);

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
                        false,
                        RichText::new(format!("{ICON_DESKTOP} Desktop"))
                            .font(FontId::proportional(16.0)),
                    )
                    .clicked()
                {
                    self.resource_type_is_desktop = true;
                }

                if ui
                    .selectable_label(
                        false,
                        RichText::new(format!("{ICON_FILE_FOLDER} Files"))
                            .font(FontId::proportional(16.0)),
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
        let label_response =
            Label::new(RichText::new("#mirrorx.cloud").font(FontId::monospace(16.0)))
                .sense(Sense::click())
                .ui(ui);

        // dropdown button
        let dropdown_response =
            Button::new(RichText::new("\u{e5cf}").font(FontId::proportional(16.0)))
                .frame(false)
                .ui(ui);

        let popup_id = ui.make_persistent_id(self.domain_dropdown_id);
        if label_response.clicked() || dropdown_response.clicked() {
            ui.memory_mut(|mem| mem.toggle_popup(popup_id));
        }

        let mut union_response = label_response.union(dropdown_response);

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
}
