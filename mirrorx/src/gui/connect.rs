use super::{
    widgets::device_id_input_field::{DeviceIDInputField, DeviceIDInputText},
    View,
};
use eframe::{
    egui::{style::Margin, Frame, Layout, Response, RichText, Rounding, TextEdit, Ui},
    emath::Align,
    epaint::{Color32, FontId, Pos2, Rect, Stroke, Vec2},
};
use egui_extras::{Size, StripBuilder};
use egui_notify::Toasts;
use mirrorx_core::{
    api::{
        config::{ConfigManager, DomainConfig},
        signaling::SignalingClient,
    },
    error::CoreResult,
};
use poll_promise::Promise;
use std::{cell::RefCell, rc::Rc, sync::Arc};

pub struct ConnectPage {
    signaling_client_promise: Promise<CoreResult<SignalingClient>>,
    config_manager: Arc<ConfigManager>,
    toasts: Rc<RefCell<Toasts>>,
    show_password: bool,
    edit_password: bool,
    edit_password_content: String,
    input_device_id: DeviceIDInputText,
    domain_promise: Option<Promise<CoreResult<Option<String>>>>,
    domain_config_promise: Option<Promise<CoreResult<Option<DomainConfig>>>>,
    save_domain_config_promise: Option<Promise<CoreResult<()>>>,
}

impl ConnectPage {
    pub fn new(config_manager: Arc<ConfigManager>, toasts: Rc<RefCell<Toasts>>) -> Self {
        Self {
            signaling_client_promise: new_signaling_client_promise(config_manager.clone()),
            config_manager,
            toasts,
            show_password: false,
            edit_password: false,
            edit_password_content: String::from(""),
            input_device_id: DeviceIDInputText::default(),
            domain_promise: None,
            domain_config_promise: None,
            save_domain_config_promise: None,
        }
    }

    fn init_signaling_client(&mut self) {
        let config_manager = self.config_manager.clone();
        self.signaling_client_promise = new_signaling_client_promise(config_manager);
    }

    fn reload_config_domain(&mut self, force: bool) {
        let config_manager = self.config_manager.clone();
        let promise_fn = || {
            Promise::spawn_async(async move {
                if let Some(domain) = config_manager.domain().await? {
                    Ok(config_manager.domain_config(&domain).await?)
                } else {
                    Ok(None)
                }
            })
        };

        if force {
            self.domain_config_promise = Some(promise_fn());
        } else {
            self.domain_config_promise.get_or_insert_with(promise_fn);
        }
    }

    fn build_domain(&mut self, ui: &mut Ui) {
        match self.signaling_client_promise.ready() {
            Some(res) => match res {
                Ok(_) => {
                    let config_manager = self.config_manager.clone();
                    let promise = self.domain_promise.get_or_insert_with(|| {
                        Promise::spawn_async(async move { config_manager.domain().await })
                    });

                    match promise.ready() {
                        Some(res) => match res {
                            Ok(domain) => match domain {
                                Some(domain) => {
                                    ui.label(
                                        RichText::new(domain).font(FontId::proportional(40.0)),
                                    );
                                }
                                None => {
                                    self.toasts
                                        .borrow_mut()
                                        .error("domain config is empty, please restart app!");
                                    ui.spinner();
                                }
                            },
                            Err(err) => {
                                tracing::error!(?err, "read config domain failed");
                                self.toasts.borrow_mut().error("read config domain failed");
                                ui.spinner();
                            }
                        },
                        None => {
                            ui.spinner();
                        }
                    }
                }
                Err(err) => {
                    tracing::error!(?err, "signaling connect failed");
                    self.toasts.borrow_mut().error("signaling connect failed");
                    ui.spinner();
                }
            },
            None => {
                ui.spinner();
            }
        }
    }

    fn build_device_id(&mut self, ui: &mut Ui) {
        match &self.domain_config_promise {
            Some(promise) => match promise.ready() {
                Some(Ok(Some(domain_config))) => {
                    let mut device_id_str = format!("{:0>10}", domain_config.device_id.to_string());
                    device_id_str.insert(2, '-');
                    device_id_str.insert(7, '-');
                    ui.label(RichText::new(device_id_str).font(FontId::proportional(50.0)));
                }
                Some(Ok(None)) => {
                    self.toasts
                        .borrow_mut()
                        .error("domain config is empty, please restart app!");
                    ui.spinner();
                }
                Some(Err(err)) => {
                    tracing::error!(?err, "read domain config failed");
                    self.toasts.borrow_mut().error("read domain config failed");
                    ui.spinner();
                }
                None => {
                    ui.spinner();
                }
            },
            None => {
                self.reload_config_domain(false);
                ui.spinner();
            }
        }
    }

    fn build_device_password(&mut self, ui: &mut Ui) {
        let domain_config = if let Some(promise) = &self.domain_config_promise {
            match promise.ready() {
                Some(Ok(domain_config)) => domain_config,
                Some(Err(err)) => {
                    tracing::error!(?err, "read domain config failed");
                    self.toasts.borrow_mut().error("read domain config failed");
                    &None
                }
                None => &None,
            }
        } else {
            self.reload_config_domain(false);
            &None
        };

        let domain_config = match domain_config {
            Some(domain_config) => domain_config,
            None => {
                ui.spinner();
                return;
            }
        };

        // panel content
        if self.edit_password {
            let text_edit_size = Vec2::new(ui.available_width() * 0.8, 30.0);
            ui.allocate_ui_at_rect(
                Rect::from_min_size(
                    ui.max_rect().min
                                    + (ui.available_size() - text_edit_size) / 2.0 // center
                                    + Vec2::new(0.0, 8.0), // y offset
                    text_edit_size,
                ),
                |ui| {
                    eframe::egui::Frame::default()
                        .stroke(Stroke::new(1.0, Color32::GRAY))
                        .rounding(Rounding::same(2.0))
                        .show(ui, |ui| {
                            ui.add_sized(
                                ui.available_size(),
                                TextEdit::singleline(&mut self.edit_password_content)
                                    .frame(false)
                                    .font(FontId::monospace(26.0)),
                            );
                        })
                },
            );
        } else {
            let content = if self.show_password {
                domain_config.device_password.as_str()
            } else {
                "＊＊＊＊＊＊＊"
            };

            let font_size = if self.show_password { 36.0 } else { 50.0 };

            ui.centered_and_justified(|ui| {
                ui.label(RichText::new(content).font(FontId::proportional(font_size)));
            });
        }

        let tool_bar_size = Vec2::new(80.0, 24.0);

        ui.allocate_ui_at_rect(
            Rect::from_min_size(
                ui.max_rect().min + Vec2::new(ui.available_width() - tool_bar_size.x, 0.0),
                tool_bar_size,
            ),
            |ui| {
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.style_mut().spacing.item_spacing = Vec2::ZERO;

                    if self.edit_password {
                        if make_password_editing_toolbar_cancel_button(ui).clicked() {
                            self.edit_password = false;
                        }

                        if make_password_editing_toolbar_commit_button(ui).clicked() {
                            let mut new_domain_config = domain_config.clone();
                            new_domain_config.device_password = self.edit_password_content.clone();

                            if let Some(Some(Ok(Some(domain)))) =
                                self.domain_promise.as_ref().map(|p| p.ready())
                            {
                                let domain = domain.clone();
                                let config_manager = self.config_manager.clone();

                                self.save_domain_config_promise =
                                    Some(Promise::spawn_async(async move {
                                        config_manager
                                            .save_domain_config(&domain, &new_domain_config)
                                            .await
                                    }));
                            }
                        }

                        if make_password_editing_toolbar_regenerate_button(ui).clicked() {
                            self.edit_password_content =
                                mirrorx_core::utility::rand::generate_random_password();
                        }
                    } else {
                        if make_password_toolbar_right_button(ui).clicked() {
                            self.show_password = !self.show_password;
                        }

                        self.show_password = self.show_password && ui.ui_contains_pointer();

                        if make_password_toolbar_left_button(ui).clicked() {
                            self.edit_password = !self.edit_password;
                            if self.edit_password {
                                self.edit_password_content = domain_config.device_password.clone();
                            }
                        };
                    }
                })
            },
        );

        if let Some(promise) = &self.save_domain_config_promise {
            if let Some(res) = promise.ready() {
                match res {
                    Ok(_) => {
                        self.save_domain_config_promise = None;
                        self.reload_config_domain(true);
                        self.edit_password = false;
                    }
                    Err(err) => {
                        self.toasts
                            .borrow_mut()
                            .error("Update password failed, please try again!");
                        tracing::error!(?err, "update device password failed");
                    }
                }
            }
        }
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
                            self.build_device_password(ui);
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
                                                        make_connect_desktop_button(ui);
                                                    });
                                                });
                                                strip.cell(|ui| {
                                                    ui.centered_and_justified(|ui| {
                                                        make_connect_file_manager_button(ui);
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
fn make_connect_desktop_button(ui: &mut Ui) {
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
fn make_connect_file_manager_button(ui: &mut Ui) {
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

    ui.add_enabled(false, eframe::egui::widgets::Button::new("File Manager"));
}

#[inline]
fn make_password_editing_toolbar_regenerate_button(ui: &mut Ui) -> Response {
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

    ui.button(RichText::new("🔄").font(FontId::proportional(18.0)))
}

#[inline]
fn make_password_editing_toolbar_commit_button(ui: &mut Ui) -> Response {
    ui.visuals_mut().widgets.hovered.expansion = 0.0;
    ui.visuals_mut().widgets.hovered.bg_stroke = Stroke::none();
    ui.visuals_mut().widgets.hovered.rounding = Rounding::none();

    ui.visuals_mut().widgets.inactive.expansion = 0.0;
    ui.visuals_mut().widgets.inactive.bg_stroke = Stroke::none();
    ui.visuals_mut().widgets.inactive.rounding = Rounding::none();

    ui.visuals_mut().widgets.active.expansion = 0.0;
    ui.visuals_mut().widgets.active.bg_stroke = Stroke::none();
    ui.visuals_mut().widgets.active.rounding = Rounding::none();

    ui.button(RichText::new("✔").font(FontId::proportional(18.0)))
}

#[inline]
fn make_password_editing_toolbar_cancel_button(ui: &mut Ui) -> Response {
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

    ui.button(RichText::new("❌").font(FontId::proportional(18.0)))
}

#[inline]
fn make_password_toolbar_left_button(ui: &mut Ui) -> Response {
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

    ui.button(RichText::new("✏").font(FontId::proportional(18.0)))
}

#[inline]
fn make_password_toolbar_right_button(ui: &mut Ui) -> Response {
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

    ui.button(RichText::new("👁").font(FontId::proportional(18.0)))
}

fn new_signaling_client_promise(
    config_manager: Arc<ConfigManager>,
) -> Promise<CoreResult<SignalingClient>> {
    Promise::spawn_async(async move {
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
    })
}
