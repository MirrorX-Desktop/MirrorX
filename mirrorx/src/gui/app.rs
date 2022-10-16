use super::{
    connect::ConnectPage,
    history::HistoryPage,
    lan::LANPage,
    widgets::{custom_toasts::CustomToasts, device_id_input_field::DeviceIDInputText},
    View,
};
use crate::utility::promise_value::{OneWayUpdatePromiseValue, PromiseValue};
use eframe::{
    egui::{style::Margin, *},
    epaint::{Color32, FontFamily, FontId, Pos2, Rect, Stroke, Vec2},
};
use egui_extras::{Size, StripBuilder};
use mirrorx_core::{
    api::{
        config::{Config, DomainConfig},
        signaling::{PublishMessage, SignalingClient, VisitResponse},
    },
    core_error,
};
use std::{collections::HashMap, path::PathBuf};
use tokio::sync::mpsc::Receiver;

pub struct App {
    selected_page_tab: String,
    toasts: CustomToasts,
    config_and_path: OneWayUpdatePromiseValue<(Config, PathBuf)>,
    signaling_client: PromiseValue<(SignalingClient, Receiver<PublishMessage>)>,
    show_password: bool,
    edit_password: bool,
    edit_password_content: String,
    device_id_input_text: DeviceIDInputText,
    is_desktop_connecting: bool,
    call_signaling_visit: PromiseValue<VisitResponse>,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
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
        let config_and_path_promise = OneWayUpdatePromiseValue::new(|| {
            Box::pin(async move {
                tokio::task::block_in_place(|| {
                    let path = directories_next::BaseDirs::new()
                        .ok_or(core_error!("get config base dir failed"))?;

                    let path = path.data_dir().join("MirrorX").join("mirrorx.db");

                    let config = match mirrorx_core::api::config::read(path.as_ref())? {
                        Some(config) => config,
                        None => {
                            let mut domain_configs = HashMap::new();
                            domain_configs.insert(
                                String::from("MirrorX.cloud"),
                                DomainConfig {
                                    addr: String::from("tcp://127.0.0.1:28000"),
                                    device_id: 0,
                                    device_finger_print:
                                        mirrorx_core::utility::rand::generate_device_finger_print(),
                                    device_password:
                                        mirrorx_core::utility::rand::generate_random_password(),
                                },
                            );

                            let default_config = Config {
                                primary_domain: String::from("MirrorX.cloud"),
                                domain_configs,
                            };

                            mirrorx_core::api::config::save(path.as_ref(), &default_config)?;
                            default_config
                        }
                    };

                    Ok((config, path))
                })
            })
        });

        Self {
            selected_page_tab: String::from("Connect"),
            toasts: CustomToasts::new(),
            config_and_path: config_and_path_promise,
            signaling_client: PromiseValue::new(),
            show_password: false,
            edit_password: false,
            edit_password_content: String::from(""),
            device_id_input_text: DeviceIDInputText::default(),
            is_desktop_connecting: false,
            call_signaling_visit: PromiseValue::new(),
        }
    }

    fn build_panel(&mut self, ui: &mut Ui) {
        ui.spacing_mut().item_spacing = Vec2::ZERO;
        StripBuilder::new(ui)
            .size(Size::relative(0.05)) // Domain Title
            .size(Size::relative(0.1)) // Domain
            .size(Size::relative(0.06)) // Tab
            .size(Size::relative(0.75)) // Page
            .size(Size::relative(0.04)) // footer
            .vertical(|mut strip| {
                strip.cell(|ui| {
                    ui.centered_and_justified(|ui| {
                        let label_rect = ui
                            .label(RichText::new("Domain").font(FontId::proportional(28.0)))
                            .rect;

                        let button_pos = Pos2::new(
                            label_rect.right_center().x - 14.0,
                            label_rect.right_center().y,
                        );

                        ui.allocate_ui_at_rect(
                            Rect::from_center_size(button_pos, Vec2::new(20.0, 20.0)),
                            |ui| {
                                if ui.button("🔄").clicked() {}
                            },
                        );
                    });
                });
                strip.cell(|ui| {
                    ui.centered_and_justified(|ui| {
                        if let Some((config, _)) = self.config_and_path.value() {
                            ui.label(
                                RichText::new(config.primary_domain.as_str())
                                    .font(FontId::proportional(40.0)),
                            );
                        } else {
                            ui.spinner();
                        }
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
                        ui.hyperlink_to("MirrorX", "https://github.com/MirrorX-Desktop/mirrorx");
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
            "Connect" => ConnectPage::new(
                &mut self.config_and_path,
                &mut self.signaling_client,
                &mut self.toasts,
                &mut self.show_password,
                &mut self.edit_password,
                &mut self.edit_password_content,
                &mut self.device_id_input_text,
                &mut self.is_desktop_connecting,
                &mut self.call_signaling_visit,
            )
            .build(ui),
            "LAN" => LANPage::new().build(ui),
            "History" => HistoryPage::new().build(ui),
            _ => panic!("unknown select page tab"),
        }
    }

    fn check_config(&mut self) {
        self.config_and_path.poll();

        if self.config_and_path.value().is_none() {
            self.config_and_path.update();
        }
    }

    fn check_signaling_client(&mut self) {
        self.signaling_client.poll();

        if self.signaling_client.value().is_none() {
            if let Some((config, config_path)) = self.config_and_path.value() {
                let config = config.clone();
                let config_path = config_path.clone();
                self.signaling_client.spawn_update(async move {
                    let (signaling_client, config, publish_message_rx) =
                        SignalingClient::new(config, config_path.clone()).await?;

                    tokio::task::block_in_place(|| {
                        mirrorx_core::api::config::save(&config_path, &config)
                    })?;

                    Ok((signaling_client, publish_message_rx))
                });
            }
        }
    }

    fn check_signaling_visit(&mut self) {
        self.call_signaling_visit.poll();

        if let Some(err) = self.call_signaling_visit.take_error() {
            tracing::error!(?err, "request visit remote failed");
            self.toasts
                .error("Request visit remote failed, please retry later");
            return;
        }

        if let Some(res) = self.call_signaling_visit.take_value() {
            if !res.allow {
                self.toasts.error("Remote reject your visit request");
                return;
            }

            // todo: pop password window
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _: &mut eframe::Frame) {
        let frame = Frame::default()
            .inner_margin(Margin::symmetric(8.0, 0.0))
            .fill(ctx.style().visuals.window_fill());

        CentralPanel::default().frame(frame).show(ctx, |ui| {
            self.check_config();
            if let Some(err) = self.config_and_path.error() {
                ui.centered_and_justified(|ui| {
                    ui.label(
                        RichText::new(format!(
                            "{:?}\n\nPlease delete the database file and re-open app!",
                            err
                        ))
                        .font(FontId::proportional(18.0)),
                    );
                });
                return;
            }

            self.check_signaling_client();
            if let Some(err) = self.signaling_client.error() {
                ui.centered_and_justified(|ui| {
                    ui.label(
                        RichText::new(format!(
                            "{:?}\n\nPlease check network and re-open app!",
                            err
                        ))
                        .font(FontId::proportional(18.0)),
                    );
                });
                return;
            }

            self.build_panel(ui);
            self.toasts.show(ui.ctx());
        });
    }
}
