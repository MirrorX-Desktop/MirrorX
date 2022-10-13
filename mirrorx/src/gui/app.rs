use super::{connect::ConnectPage, history::HistoryPage, View};
use eframe::{
    egui::{
        style::Margin, CentralPanel, Context, FontData, FontDefinitions, Frame, RichText, Rounding,
        Ui,
    },
    epaint::{Color32, FontFamily, FontId, Stroke, Vec2},
};
use egui_extras::{Size, StripBuilder};
use egui_notify::Toasts;
use mirrorx_core::api::config::ConfigManager;
use std::{cell::RefCell, rc::Rc, sync::Arc};

pub struct App {
    selected_page_tab: String,
    connect_page: super::connect::ConnectPage,
    history_page: super::history::HistoryPage,
    lan_page: super::lan::LANPage,
    // config_manager: Arc<mirrorx_core::api::config::ConfigManager>,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>, config_manager: ConfigManager) -> Self {
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

        // initialize some global components
        let toasts = Rc::new(RefCell::new(Toasts::default()));
        let config_manager = Arc::new(config_manager);

        Self {
            selected_page_tab: String::from("Connect"),
            connect_page: ConnectPage::new(config_manager.clone(), toasts),
            history_page: HistoryPage::new(config_manager),
            lan_page: Default::default(),
            // config_manager,
        }
    }

    fn build_panel(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        CentralPanel::default()
            .frame(
                Frame::default()
                    .inner_margin(Margin::symmetric(8.0, 0.0))
                    .fill(ctx.style().visuals.window_fill()),
            )
            .show(ctx, |ui| {
                ui.spacing_mut().item_spacing = Vec2::ZERO;
                StripBuilder::new(ui)
                    .size(Size::relative(0.1))
                    .size(Size::relative(0.06))
                    .size(Size::relative(0.8))
                    .size(Size::remainder())
                    .vertical(|mut strip| {
                        strip.cell(|ui| {
                            ui.centered_and_justified(|ui| {
                                ui.label(RichText::new("MirrorX").font(FontId::proportional(40.0)));
                            });
                        });
                        strip.strip(|builder| {
                            builder
                                .sizes(Size::relative(1.0 / 3.0), 3)
                                .horizontal(|mut strip| {
                                    strip.cell(|ui| {
                                        self.build_toggle_tab(ui, "Connect", "Connect");
                                    });

                                    strip.cell(|ui| {
                                        self.build_toggle_tab(ui, "LAN", "LAN");
                                    });

                                    strip
                                        .cell(|ui| self.build_toggle_tab(ui, "History", "History"));
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

    fn build_toggle_tab(&mut self, ui: &mut Ui, display_text: &str, toggle_tab_value: &str) {
        ui.centered_and_justified(|ui| {
            ui.visuals_mut().widgets.hovered.expansion = 0.0;
            ui.visuals_mut().widgets.hovered.bg_stroke = Stroke::none();
            ui.visuals_mut().widgets.hovered.rounding = Rounding::same(2.0);

            ui.visuals_mut().widgets.inactive.expansion = 0.0;
            ui.visuals_mut().widgets.inactive.bg_stroke = Stroke::none();
            ui.visuals_mut().widgets.inactive.rounding = Rounding::same(2.0);

            ui.visuals_mut().widgets.active.expansion = 0.0;
            ui.visuals_mut().widgets.active.bg_stroke = Stroke::none();
            ui.visuals_mut().widgets.active.rounding = Rounding::same(2.0);

            let toggle = ui.toggle_value(
                &mut (self.selected_page_tab == toggle_tab_value),
                display_text,
            );

            if toggle.clicked() {
                self.selected_page_tab = toggle_tab_value.to_string();
            }
        });
    }

    fn build_tab_view(&mut self, ui: &mut Ui) {
        match self.selected_page_tab.as_str() {
            "Connect" => self.connect_page.build(ui),
            "LAN" => self.lan_page.build(ui),
            "History" => self.history_page.build(ui),
            _ => panic!("unknown select page tab"),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, frame: &mut eframe::Frame) {
        self.build_panel(ctx, frame);
    }
}
