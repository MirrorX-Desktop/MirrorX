use eframe::{
    egui::{style::Margin, CentralPanel, Context, FontData, FontDefinitions, Frame, RichText, Ui},
    emath::Align,
    epaint::{Color32, FontFamily, FontId, Stroke, Vec2},
};
use egui_extras::{Size, StripBuilder};

use super::View;

#[derive(Default)]
pub struct App {
    selected_page_tab: String,
    connect_page: super::connect::ConnectPage,
    history_page: super::history::HistoryPage,
    lan_page: super::lan::LANPage,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        let mut fonts = FontDefinitions::default();
        fonts.font_data.insert(
            "NotoSans".to_owned(),
            FontData::from_static(include_bytes!("../../assets/fonts/NotoSans-Regular.ttf")),
        );
        fonts.font_data.insert(
            "NotoSansJP".to_owned(),
            FontData::from_static(include_bytes!("../../assets/fonts/NotoSansJP-Regular.otf")),
        );
        fonts.font_data.insert(
            "NotoSansKR".to_owned(),
            FontData::from_static(include_bytes!("../../assets/fonts/NotoSansKR-Regular.otf")),
        );
        fonts.font_data.insert(
            "NotoSansSC".to_owned(),
            FontData::from_static(include_bytes!("../../assets/fonts/NotoSansSC-Regular.otf")),
        );
        fonts.font_data.insert(
            "NotoSansTC".to_owned(),
            FontData::from_static(include_bytes!("../../assets/fonts/NotoSansTC-Regular.otf")),
        );
        fonts.font_data.insert(
            "NotoSansMono".to_owned(),
            FontData::from_static(include_bytes!(
                "../../assets/fonts/NotoSansMono-Regular.ttf"
            )),
        );

        let mut proportional_fonts = vec![
            "NotoSans".to_owned(),
            "NotoSansSC".to_owned(),
            "NotoSansTC".to_owned(),
            "NotoSansJP".to_owned(),
            "NotoSansKR".to_owned(),
        ];

        let old_fonts = fonts.families.entry(FontFamily::Proportional).or_default();

        proportional_fonts.append(old_fonts);

        fonts
            .families
            .insert(FontFamily::Proportional, proportional_fonts.clone());

        let mut mono_fonts = vec!["NotoSansMono".to_owned()];

        let old_fonts = fonts.families.entry(FontFamily::Monospace).or_default();

        mono_fonts.append(old_fonts);

        fonts
            .families
            .insert(FontFamily::Monospace, mono_fonts.clone());

        cc.egui_ctx.set_debug_on_hover(true);
        cc.egui_ctx.set_fonts(fonts);

        Self::default()
    }

    fn build_toggle_tab(&mut self, ui: &mut Ui, display_text: &str, toggle_tab_value: &str) {
        ui.centered_and_justified(|ui| {
            if ui
                .toggle_value(
                    &mut (self.selected_page_tab == toggle_tab_value),
                    display_text,
                )
                .clicked()
            {
                toggle_tab_value.clone_into(&mut self.selected_page_tab);
            }
        });
    }

    fn build_tab_view(&mut self, ui: &mut Ui) {
        if self.selected_page_tab == "Connect" {
            self.connect_page.build(ui);
        }

        if self.selected_page_tab == "LAN" {
            self.lan_page.build(ui);
        }

        if self.selected_page_tab == "History" {
            self.history_page.build(ui);
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.spacing_mut().item_spacing = Vec2::ZERO;
            StripBuilder::new(ui)
                .size(Size::relative(0.1))
                .size(Size::relative(0.06))
                .size(Size::relative(0.8))
                .size(Size::remainder())
                .vertical(|mut strip| {
                    strip.cell(|ui| {
                        ui.centered_and_justified(|ui| {
                            ui.label("Title");
                        });
                    });
                    strip.strip(|builder| {
                        builder
                            .sizes(Size::relative(0.333333), 3)
                            .horizontal(|mut strip| {
                                strip.cell(|ui| {
                                    self.build_toggle_tab(ui, "Connect", "Connect");
                                });

                                strip.cell(|ui| {
                                    self.build_toggle_tab(ui, "LAN", "LAN");
                                });

                                strip.cell(|ui| self.build_toggle_tab(ui, "History", "History"));
                            });
                    });
                    strip.cell(|ui| self.build_tab_view(ui));
                    strip.cell(|ui| {
                        ui.painter().line_segment(
                            [
                                ui.max_rect().left_top() + Vec2::new(2.0, 0.0),
                                ui.max_rect().right_top() + Vec2::new(-2.0, 0.0),
                            ],
                            Stroke::new(1.0, Color32::GRAY),
                        );

                        ui.centered_and_justified(|ui| {
                            ui.hyperlink_to(
                                "MirrorX",
                                "https://github.com/MirrorX-Desktop/mirrorx",
                            );
                        });
                    });
                });
        });
    }
}
