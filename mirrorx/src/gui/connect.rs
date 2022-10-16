use std::path::PathBuf;

use crate::utility::promise_value::{OneWayUpdatePromiseValue, PromiseValue};

use super::{
    widgets::{
        custom_toasts::CustomToasts,
        device_id_input_field::{DeviceIDInputField, DeviceIDInputText},
    },
    View,
};
use eframe::{egui::*, emath::Align, epaint::*};
use egui_extras::{Size, StripBuilder};
use egui_toast::{Toast, ToastKind, ToastOptions, Toasts};
use mirrorx_core::{
    api::{
        config::{Config, DomainConfig},
        signaling::{PublishMessage, SignalingClient, VisitResponse},
    },
    error::CoreResult,
};
use tokio::sync::mpsc::Receiver;

pub struct ConnectPage<'a> {
    config_and_path: &'a mut OneWayUpdatePromiseValue<(Config, PathBuf)>,
    signaling_client: &'a mut PromiseValue<(SignalingClient, Receiver<PublishMessage>)>,
    toasts: &'a mut CustomToasts,
    show_password: &'a mut bool,
    edit_password: &'a mut bool,
    edit_password_content: &'a mut String,
    device_id_input_text: &'a mut DeviceIDInputText,
    is_desktop_connecting: &'a mut bool,
    call_signaling_visit: &'a mut PromiseValue<VisitResponse>,
}

impl<'a> ConnectPage<'a> {
    pub fn new(
        config_and_path: &'a mut OneWayUpdatePromiseValue<(Config, PathBuf)>,
        signaling_client: &'a mut PromiseValue<(SignalingClient, Receiver<PublishMessage>)>,
        toasts: &'a mut CustomToasts,
        show_password: &'a mut bool,
        edit_password: &'a mut bool,
        edit_password_content: &'a mut String,
        device_id_input_text: &'a mut DeviceIDInputText,
        is_desktop_connecting: &'a mut bool,
        call_signaling_visit: &'a mut PromiseValue<VisitResponse>,
    ) -> Self {
        Self {
            config_and_path,
            signaling_client,
            toasts,
            show_password,
            edit_password,
            edit_password_content,
            device_id_input_text,
            is_desktop_connecting,
            call_signaling_visit,
        }
    }

    #[inline]
    fn build_device_id(&mut self, ui: &mut Ui) {
        if let Some((config, _)) = self.config_and_path.value() {
            if let Some(domain_config) = config.domain_configs.get(&config.primary_domain) {
                let mut device_id_str = format!("{:0>10}", domain_config.device_id);
                device_id_str.insert(2, '-');
                device_id_str.insert(7, '-');
                ui.label(RichText::new(device_id_str).font(FontId::proportional(50.0)));
            }
        } else {
            ui.spinner();
        }
    }

    #[inline]
    fn build_device_password(&mut self, ui: &mut Ui) {
        let domain_config = if let Some((config, _)) = self.config_and_path.value() {
            if let Some(domain_config) = config.domain_configs.get(&config.primary_domain) {
                domain_config
            } else {
                ui.spinner();
                return;
            }
        } else {
            ui.spinner();
            return;
        };

        if *self.edit_password {
            build_device_password_edit(ui, self.edit_password_content);
        } else {
            build_device_password_label(ui, *self.show_password, &domain_config.device_password);
        }
    }

    #[inline]
    fn build_device_password_toolbar(&mut self, ui: &mut Ui) {
        let tool_bar_size = Vec2::new(80.0, 24.0);
        ui.allocate_ui_at_rect(
            Rect::from_min_size(
                ui.max_rect().min + Vec2::new(ui.available_width() - tool_bar_size.x, 0.0),
                tool_bar_size,
            ),
            |ui| {
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.style_mut().spacing.item_spacing = Vec2::ZERO;

                    if *self.edit_password {
                        if make_password_editing_toolbar_cancel_button(ui).clicked() {
                            *self.edit_password = false;
                        }

                        if make_password_editing_toolbar_commit_button(ui).clicked() {
                            if let Some((old_config, config_path)) = self.config_and_path.value() {
                                let mut new_config = old_config.clone();
                                if let Some(domain_config) = new_config
                                    .domain_configs
                                    .get_mut(&old_config.primary_domain)
                                {
                                    domain_config.device_password =
                                        self.edit_password_content.clone();

                                    match mirrorx_core::api::config::save(config_path, &new_config)
                                    {
                                        Ok(_) => {
                                            self.config_and_path.update();
                                            *self.edit_password = false;
                                        }
                                        Err(err) => {
                                            tracing::error!(?err, "update device password failed");
                                            self.toasts.error("Update device password failed, please try again later!");
                                        }
                                    }
                                }
                            }
                        }

                        if make_password_editing_toolbar_regenerate_button(ui).clicked() {
                            *self.edit_password_content =
                                mirrorx_core::utility::rand::generate_random_password();
                        }
                    } else {
                        if make_password_toolbar_right_button(ui).clicked() {
                            *self.show_password = !(*self.show_password);
                        }

                        *self.show_password = *self.show_password && ui.ui_contains_pointer();

                        if make_password_toolbar_left_button(ui).clicked() {
                            *self.edit_password = !(*self.edit_password);
                            if *self.edit_password {
                                let current_password = match self.config_and_path.value() {
                                    Some((config, _)) => {
                                        match config.domain_configs.get(&config.primary_domain) {
                                            Some(domain_config) => {
                                                domain_config.device_password.clone()
                                            }
                                            None => String::from(""),
                                        }
                                    }
                                    None => String::from(""),
                                };

                                *self.edit_password_content = current_password;
                            }
                        };
                    }
                })
            },
        );
    }

    #[inline]
    fn build_connect_desktop_button(&mut self, ui: &mut Ui) {
        ui.visuals_mut().widgets.hovered.expansion = 0.0;
        ui.visuals_mut().widgets.hovered.bg_stroke = Stroke::none();
        ui.visuals_mut().widgets.hovered.bg_fill = Color32::from_rgb(0x19, 0x8C, 0xFF);
        ui.visuals_mut().widgets.hovered.fg_stroke = Stroke::new(1.0, Color32::WHITE);
        ui.visuals_mut().widgets.hovered.rounding = Rounding {
            nw: 2.0,
            ne: 0.0,
            sw: 2.0,
            se: 0.0,
        };

        ui.visuals_mut().widgets.inactive.expansion = 0.0;
        ui.visuals_mut().widgets.inactive.bg_stroke = Stroke::none();
        ui.visuals_mut().widgets.inactive.bg_fill = Color32::from_rgb(0x01, 0x6F, 0xFF);
        ui.visuals_mut().widgets.inactive.fg_stroke = Stroke::new(1.0, Color32::WHITE);
        ui.visuals_mut().widgets.inactive.rounding = Rounding {
            nw: 2.0,
            ne: 0.0,
            sw: 2.0,
            se: 0.0,
        };

        ui.visuals_mut().widgets.active.expansion = 0.0;
        ui.visuals_mut().widgets.active.bg_stroke = Stroke::none();
        ui.visuals_mut().widgets.active.bg_fill = Color32::from_rgb(0x00, 0x54, 0xE6);
        ui.visuals_mut().widgets.active.fg_stroke = Stroke::new(1.0, Color32::WHITE);
        ui.visuals_mut().widgets.active.rounding = Rounding {
            nw: 2.0,
            ne: 0.0,
            sw: 2.0,
            se: 0.0,
        };

        let (rect, response) = ui.allocate_exact_size(ui.available_size(), Sense::click());

        ui.allocate_ui_at_rect(rect, |ui| {
            if *self.is_desktop_connecting {
                ui.painter().rect_filled(
                    rect,
                    ui.visuals().widgets.active.rounding,
                    ui.visuals().widgets.active.bg_fill,
                );
                ui.add_enabled(false, Spinner::default());
            } else {
                let visuals = ui.style().interact(&response);
                ui.painter()
                    .rect_filled(rect, visuals.rounding, visuals.bg_fill);

                let text = WidgetText::from("桌面")
                    .color(visuals.fg_stroke.color)
                    .into_galley(ui, None, ui.available_width(), FontId::proportional(28.0));

                ui.painter().add(TextShape {
                    pos: rect.left_top() + ((ui.available_size() - text.size()) / 2.0),
                    galley: text.galley,
                    underline: Stroke::none(),
                    override_text_color: None,
                    angle: 0.0,
                });
            }
        });

        if response.clicked() && !(*self.is_desktop_connecting) {
            self.connect_desktop();
        }
    }

    fn connect_desktop(&mut self) {
        let input_device_id = self.device_id_input_text.as_str();
        if input_device_id.len() != 10 || !input_device_id.chars().all(|c| c.is_ascii_digit()) {
            self.toasts.error("Invalid connect device id");
            return;
        }

        let input_device_id: i64 = match input_device_id.parse() {
            Ok(v) => v,
            Err(_) => {
                self.toasts.error("Invalid connect device id format");
                return;
            }
        };

        let config = match self.config_and_path.value() {
            Some((config, _)) => config,
            None => {
                self.toasts.error("Current config is empty");
                return;
            }
        };

        let domain_config = match config.domain_configs.get(&config.primary_domain) {
            Some(domain_config) => domain_config,
            None => {
                self.toasts.error("Current domain config is empty");
                return;
            }
        };

        let domain = config.primary_domain.clone();
        let local_device_id = domain_config.device_id;

        if let Some((signaling_client, _)) = self.signaling_client.value() {
            *self.is_desktop_connecting = true;
            let signaling_client = signaling_client.clone();
            self.call_signaling_visit.spawn_update(async move {
                signaling_client
                    .visit(mirrorx_core::api::signaling::VisitRequest {
                        domain,
                        local_device_id,
                        remote_device_id: input_device_id,
                        resource_type: mirrorx_core::api::signaling::ResourceType::Desktop,
                    })
                    .await
            });
        } else {
            // todo: give a resolution
            self.toasts.error("Signaling connection is broken");
        }
    }
}

impl View for ConnectPage<'_> {
    fn build(&mut self, ui: &mut eframe::egui::Ui) {
        StripBuilder::new(ui)
            .size(Size::relative(0.21)) // Device ID Panel
            .size(Size::relative(0.21)) // Password Panel
            .size(Size::relative(0.17)) // Connect Remote Title
            .size(Size::exact(64.0)) // Connect Device ID Panel
            .size(Size::remainder()) // Connect Button
            .vertical(|mut strip| {
                // Device ID Panel
                strip.cell(|ui| {
                    ui.centered_and_justified(|ui| {
                        self.build_device_id(ui);
                    });
                });

                // Password Panel
                strip.cell(|ui| {
                    self.build_device_password(ui);
                    self.build_device_password_toolbar(ui);
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
                        ui.label(RichText::new("Connect Remote").font(FontId::proportional(28.0)));
                    });
                });

                // Connect Device ID Panel
                strip.cell(|ui| {
                    ui.centered_and_justified(|ui| {
                        ui.add_sized(
                            Vec2::new(0.0, 60.0),
                            DeviceIDInputField::text(self.device_id_input_text),
                        );
                    });
                });

                // Connect Button
                strip.strip(|strip| {
                    strip
                        .size(Size::relative(0.3))
                        .size(Size::relative(0.4))
                        .size(Size::relative(0.3))
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
                                                self.build_connect_desktop_button(ui);
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
    }
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

#[inline]
fn build_device_password_edit(ui: &mut Ui, edit_content: &mut dyn TextBuffer) {
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
                        TextEdit::singleline(edit_content)
                            .frame(false)
                            .font(FontId::monospace(26.0)),
                    );
                })
        },
    );
}

#[inline]
fn build_device_password_label(ui: &mut Ui, show_password: bool, password: &str) {
    let content = if show_password {
        password
    } else {
        "＊＊＊＊＊＊＊"
    };

    let font_size = if show_password { 36.0 } else { 50.0 };

    ui.centered_and_justified(|ui| {
        ui.label(RichText::new(content).font(FontId::proportional(font_size)));
    });
}
