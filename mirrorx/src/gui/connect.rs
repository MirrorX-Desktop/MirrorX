use super::{
    widgets::device_id_input_field::{DeviceIDInputField, DeviceIDInputText},
    View,
};
use eframe::{
    egui::{style::Margin, Frame, RichText, Rounding, Ui},
    epaint::{Color32, FontId, Pos2, Rect, Stroke, Vec2},
};
use egui_extras::{Size, StripBuilder};
use mirrorx_core::{
    api::{
        config::{ConfigManager, DomainConfig},
        signaling::SignalingClient,
    },
    error::CoreResult,
};
use poll_promise::Promise;
use std::sync::Arc;

pub struct ConnectPage {
    config_manager: Arc<ConfigManager>,
    signaling_client_promise:
        Option<Promise<CoreResult<mirrorx_core::api::signaling::SignalingClient>>>,
    show_password: bool,
    input_device_id: DeviceIDInputText,
    domain_promise: Option<Promise<CoreResult<Option<String>>>>,
    domain_config_promise: Option<Promise<CoreResult<Option<DomainConfig>>>>,
}

impl ConnectPage {
    pub fn new(config_manager: Arc<ConfigManager>) -> Self {
        let mut page = Self {
            config_manager,
            signaling_client_promise: None,
            show_password: false,
            input_device_id: DeviceIDInputText::default(),
            domain_promise: None,
            domain_config_promise: None,
        };

        page.init_signaling_client();

        page
    }

    fn init_signaling_client(&mut self) {
        let config_manager = self.config_manager.clone();
        self.signaling_client_promise = Some(Promise::spawn_async(async move {
            let mut domain = config_manager.domain().await?;
            let domain = domain.get_or_insert_with(|| String::from("MirrorX.cloud"));

            let mut domain_config = config_manager.domain_config(domain).await?;
            let mut domain_config = domain_config.get_or_insert_with(|| DomainConfig {
                uri: String::from("tcp://127.0.0.1:28000"),
                device_id: 0,
                device_finger_print: mirrorx_core::utility::rand::generate_device_finger_print(),
                device_password: mirrorx_core::utility::rand::generate_random_password(),
            });

            let client =
                mirrorx_core::api::signaling::SignalingClient::dial(&domain_config.uri).await?;

            let register_response = client
                .register(mirrorx_core::api::signaling::RegisterRequest {
                    device_id: if domain_config.device_id != 0 {
                        Some(domain_config.device_id)
                    } else {
                        None
                    },
                    device_finger_print: domain_config.device_finger_print.clone(),
                })
                .await?;

            domain_config.device_id = register_response.device_id;

            config_manager
                .save_domain(&register_response.domain)
                .await?;

            config_manager
                .save_domain_config(&register_response.domain, domain_config)
                .await?;

            Ok(client)
        }));
    }

    fn build_domain(&mut self, ui: &mut Ui) {
        if let Some(Some(Ok(_))) = self.signaling_client_promise.as_ref().map(|p| p.ready()) {
            let config_manager = self.config_manager.clone();
            let promise = self.domain_promise.get_or_insert_with(|| {
                Promise::spawn_async(async move { config_manager.domain().await })
            });

            match promise.ready() {
                Some(Ok(Some(domain))) => {
                    ui.label(RichText::new(domain).font(FontId::proportional(40.0)));
                }
                Some(Ok(None)) => {
                    ui.label(RichText::new("None").font(FontId::proportional(40.0)));
                }
                Some(Err(err)) => {
                    tracing::error!(?err, "read config domain failed");
                    ui.label(RichText::new("Error").font(FontId::proportional(40.0)));
                }
                None => {
                    ui.spinner();
                }
            }

            return;
        }

        ui.spinner();
    }

    fn build_device_id(&mut self, ui: &mut Ui) {
        if let Some(Some(Ok(_))) = self.signaling_client_promise.as_ref().map(|p| p.ready()) {
            let config_manager = self.config_manager.clone();
            let promise = self.domain_config_promise.get_or_insert_with(|| {
                Promise::spawn_async(async move {
                    if let Some(domain) = config_manager.domain().await? {
                        Ok(config_manager.domain_config(&domain).await?)
                    } else {
                        Ok(None)
                    }
                })
            });

            match promise.ready() {
                Some(Ok(Some(domain_config))) => {
                    let mut device_id_str = format!("{:0>10}", domain_config.device_id.to_string());
                    device_id_str.insert(2, '-');
                    device_id_str.insert(7, '-');
                    ui.label(RichText::new(device_id_str).font(FontId::proportional(50.0)));
                }
                Some(Ok(None)) => {
                    ui.label(RichText::new("None").font(FontId::proportional(40.0)));
                }
                Some(Err(err)) => {
                    tracing::error!(?err, "read config domain failed");
                    ui.label(RichText::new("Error").font(FontId::proportional(40.0)));
                }
                None => {
                    ui.spinner();
                }
            }

            return;
        }

        ui.spinner();
    }

    fn build_device_password(&mut self, ui: &mut Ui) {
        if let Some(Some(Ok(_))) = self.signaling_client_promise.as_ref().map(|p| p.ready()) {
            let config_manager = self.config_manager.clone();
            let promise = self.domain_config_promise.get_or_insert_with(|| {
                Promise::spawn_async(async move {
                    if let Some(domain) = config_manager.domain().await? {
                        Ok(config_manager.domain_config(&domain).await?)
                    } else {
                        Ok(None)
                    }
                })
            });

            match promise.ready() {
                Some(Ok(Some(domain_config))) => {
                    let content = if self.show_password {
                        domain_config.device_password.as_str()
                    } else {
                        "＊＊＊＊＊＊＊"
                    };

                    let font_size = if self.show_password { 36.0 } else { 50.0 };

                    let password_label =
                        ui.label(RichText::new(content).font(FontId::proportional(font_size)));

                    // put the show password toggle button on the label right-top corner
                    let password_right_top_pos = password_label.rect.right_top();
                    let show_password_toggle_pos = Pos2::new(
                        password_right_top_pos.x - 14.0,
                        password_right_top_pos.y + 12.0,
                    );

                    ui.allocate_ui_at_rect(
                        Rect::from_center_size(show_password_toggle_pos, Vec2::new(20.0, 20.0)),
                        |ui| {
                            if !ui
                                .toggle_value(
                                    &mut self.show_password,
                                    RichText::new("👁").font(FontId::proportional(18.0)),
                                )
                                .hovered()
                            {
                                self.show_password = false;
                            };
                        },
                    );
                }
                Some(Ok(None)) => {
                    ui.label(RichText::new("None").font(FontId::proportional(40.0)));
                }
                Some(Err(err)) => {
                    tracing::error!(?err, "read config domain failed");
                    ui.label(RichText::new("Error").font(FontId::proportional(40.0)));
                }
                None => {
                    ui.spinner();
                }
            }

            return;
        }

        ui.spinner();
    }
}

impl View for ConnectPage {
    fn build(&mut self, ui: &mut eframe::egui::Ui) {
        Frame::default()
            .inner_margin(Margin::same(8.0))
            .show(ui, |ui| {
                StripBuilder::new(ui)
                    .size(Size::relative(0.09)) // Domain Title
                    .size(Size::relative(0.16)) // Domain
                    .size(Size::relative(0.16)) // Device ID Panel
                    .size(Size::relative(0.16)) // Password Panel
                    .size(Size::relative(0.12)) // Connect Remote Title
                    .size(Size::relative(0.13)) // Connect Device ID Panel
                    .size(Size::relative(0.18)) // Connect Button
                    .vertical(|mut strip| {
                        // Domain Title
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

                        // Domain
                        strip.cell(|ui| {
                            ui.centered_and_justified(|ui| {
                                self.build_domain(ui);
                            });
                        });

                        // Device ID Panel
                        strip.cell(|ui| {
                            ui.centered_and_justified(|ui| {
                                self.build_device_id(ui);
                            });
                        });

                        // Password Panel
                        strip.cell(|ui| {
                            ui.centered_and_justified(|ui| {
                                self.build_device_password(ui);
                            });
                        });

                        // Connect Remote Title
                        strip.cell(|ui| {
                            ui.painter().line_segment(
                                [
                                    ui.max_rect().left_top() + Vec2::new(2.0, 0.0),
                                    ui.max_rect().right_top() + Vec2::new(-2.0, 0.0),
                                ],
                                Stroke::new(1.0, Color32::GRAY),
                            );

                            ui.centered_and_justified(|ui| {
                                ui.label(
                                    RichText::new("Connect Remote")
                                        .font(FontId::proportional(28.0)),
                                );
                            });
                        });

                        // Connect Device ID Panel
                        strip.cell(|ui| {
                            ui.centered_and_justified(|ui| {
                                ui.add(DeviceIDInputField::text(&mut self.input_device_id));
                            });
                        });

                        // Connect Button
                        strip.strip(|strip| {
                            strip
                                .size(Size::relative(0.28))
                                .size(Size::relative(0.54))
                                .size(Size::relative(0.28))
                                .vertical(|mut strip| {
                                    strip.empty();
                                    strip.strip(|strip| {
                                        strip
                                            .size(Size::relative(0.15))
                                            .size(Size::relative(0.35))
                                            .size(Size::relative(0.35))
                                            .size(Size::relative(0.15))
                                            .horizontal(|mut strip| {
                                                strip.empty();
                                                strip.cell(|ui| {
                                                    ui.centered_and_justified(|ui| {
                                                        make_left_connect_button(ui);
                                                    });
                                                });
                                                strip.cell(|ui| {
                                                    ui.centered_and_justified(|ui| {
                                                        make_right_connect_button(ui);
                                                    });
                                                });
                                                strip.empty();
                                            });
                                    });
                                    strip.empty();
                                });
                        });
                    });
            });
    }
}

#[inline]
fn make_left_connect_button(ui: &mut Ui) {
    ui.visuals_mut().widgets.hovered.expansion = 0.0;
    ui.visuals_mut().widgets.hovered.bg_stroke = Stroke::none();
    ui.visuals_mut().widgets.hovered.rounding = Rounding {
        nw: 2.0,
        ne: 0.0,
        sw: 2.0,
        se: 0.0,
    };

    ui.visuals_mut().widgets.inactive.expansion = 0.0;
    ui.visuals_mut().widgets.inactive.bg_stroke = Stroke::none();
    ui.visuals_mut().widgets.inactive.rounding = Rounding {
        nw: 2.0,
        ne: 0.0,
        sw: 2.0,
        se: 0.0,
    };

    ui.visuals_mut().widgets.active.expansion = 0.0;
    ui.visuals_mut().widgets.active.bg_stroke = Stroke::none();
    ui.visuals_mut().widgets.active.rounding = Rounding {
        nw: 2.0,
        ne: 0.0,
        sw: 2.0,
        se: 0.0,
    };

    if ui.button("Desktop").clicked() {}
}

#[inline]
fn make_right_connect_button(ui: &mut Ui) {
    ui.visuals_mut().widgets.hovered.expansion = 0.0;
    ui.visuals_mut().widgets.hovered.bg_stroke = Stroke::none();
    ui.visuals_mut().widgets.hovered.rounding = Rounding {
        nw: 0.0,
        ne: 2.0,
        sw: 0.0,
        se: 2.0,
    };

    ui.visuals_mut().widgets.inactive.expansion = 0.0;
    ui.visuals_mut().widgets.inactive.bg_stroke = Stroke::none();
    ui.visuals_mut().widgets.inactive.rounding = Rounding {
        nw: 0.0,
        ne: 2.0,
        sw: 0.0,
        se: 2.0,
    };

    ui.visuals_mut().widgets.active.expansion = 0.0;
    ui.visuals_mut().widgets.active.bg_stroke = Stroke::none();
    ui.visuals_mut().widgets.active.rounding = Rounding {
        nw: 0.0,
        ne: 2.0,
        sw: 0.0,
        se: 2.0,
    };

    if ui.button("File Manager").clicked() {}
}
