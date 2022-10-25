use super::state::State;
use super::state::StateUpdater;
use super::widgets::device_id_input_field::DeviceIDInputField;
use egui::{emath::Align, epaint::*, *};
use egui_extras::{Size, StripBuilder};
use mirrorx_core::{api::signaling::ResourceType, core_error};

pub struct ConnectPage<'a> {
    state: &'a State,
    state_updater: StateUpdater,
}

impl<'a> ConnectPage<'a> {
    pub fn new(state: &'a State) -> Self {
        let app_state_updater = state.new_state_updater();
        Self {
            state,
            state_updater: app_state_updater,
        }
    }

    #[inline]
    fn build_device_id(&mut self, ui: &mut Ui) {
        if let Some(config) = self.state.config() {
            if let Some(domain_config) = config.domain_configs.get(&config.primary_domain) {
                if domain_config.device_id != 0 {
                    let mut device_id_str = format!("{:0>10}", domain_config.device_id);
                    device_id_str.insert(2, '-');
                    device_id_str.insert(7, '-');
                    ui.label(RichText::new(device_id_str).font(FontId::proportional(50.0)));
                } else {
                    ui.spinner();
                }
            }
        } else {
            ui.spinner();
        }
    }

    #[inline]
    fn build_device_password(&mut self, ui: &mut Ui) {
        if let Some(config) = self.state.config() {
            if config.domain_configs.get(&config.primary_domain).is_none() {
                ui.spinner();
                return;
            }
        } else {
            ui.spinner();
            return;
        };

        if self.state.connect_page_password_editing() {
            self.build_device_password_edit(ui);
        } else {
            self.build_device_password_label(ui);
        }
    }

    #[inline]
    fn build_device_password_edit(&mut self, ui: &mut Ui) {
        let text_edit_size = Vec2::new(ui.available_width() * 0.8, 30.0);
        ui.allocate_ui_at_rect(
            Rect::from_min_size(
                ui.max_rect().min
                                + (ui.available_size() - text_edit_size) / 2.0 // center
                                + Vec2::new(0.0, 8.0), // y offset
                text_edit_size,
            ),
            |ui| {
                Frame::default()
                    .stroke(Stroke::new(1.0, Color32::GRAY))
                    .rounding(Rounding::same(2.0))
                    .show(ui, |ui| {
                        let mut text_buffer = self.state.connect_page_password().to_string();
                        if text_buffer.is_empty() {
                            if let Some(config) = self.state.config() {
                                if let Some(domain_config) =
                                    config.domain_configs.get(&config.primary_domain)
                                {
                                    text_buffer = domain_config.device_password.to_owned();
                                } else {
                                    text_buffer = String::new();
                                }
                            } else {
                                text_buffer = String::new();
                            }
                        }

                        if ui
                            .add_sized(
                                ui.available_size(),
                                TextEdit::singleline(&mut text_buffer)
                                    .frame(false)
                                    .text_color(ui.visuals().text_color())
                                    .font(FontId::monospace(26.0)),
                            )
                            .changed()
                        {
                            self.state_updater
                                .update_connect_page_password(text_buffer.as_str());
                        }
                    })
            },
        );
    }

    #[inline]
    fn build_device_password_label(&mut self, ui: &mut Ui) {
        ui.centered_and_justified(|ui| {
            let mut content = "";
            if self.state.connect_page_password_visible() {
                if let Some(config) = self.state.config() {
                    if let Some(domain_config) = config.domain_configs.get(&config.primary_domain) {
                        content = domain_config.device_password.as_str();
                    }
                }
            } else {
                content = "＊＊＊＊＊＊＊";
            };

            if content.is_empty() {
                ui.spinner();
            } else {
                let font_size = if self.state.connect_page_password_visible() {
                    36.0
                } else {
                    50.0
                };

                ui.label(RichText::new(content).font(FontId::proportional(font_size)));
            }
        });
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

                    if self.state.connect_page_password_editing() {
                        if make_password_editing_toolbar_cancel_button(ui).clicked() {
                            self.state_updater
                                .update_connect_page_password_editing(false);
                        }

                        if make_password_editing_toolbar_commit_button(ui).clicked() {
                            if let Some(old_config) = self.state.config() {
                                let mut new_config = old_config.clone();
                                if let Some(domain_config) = new_config
                                    .domain_configs
                                    .get_mut(&old_config.primary_domain)
                                {
                                    let password = self.state.connect_page_password().to_string();
                                    domain_config.device_password = password;

                                    self.state_updater.update_config(&new_config);
                                    self.state_updater.update_connect_page_password("");
                                    self.state_updater
                                        .update_connect_page_password_editing(false);
                                }
                            }
                        }

                        if make_password_editing_toolbar_regenerate_button(ui).clicked() {
                            self.state_updater.update_connect_page_password(
                                &mirrorx_core::utility::rand::generate_random_password(),
                            );
                        }
                    } else {
                        let mut snapshot_visible = self.state.connect_page_password_visible();
                        if make_password_toolbar_visible_button(ui).clicked() {
                            snapshot_visible = true;
                        }

                        self.state_updater.update_connect_page_password_visible(
                            snapshot_visible && ui.ui_contains_pointer(),
                        );

                        if make_password_toolbar_edit_button(ui).clicked() {
                            self.state_updater
                                .update_connect_page_password_editing(true);
                        };
                    }
                })
            },
        );
    }

    #[inline]
    fn build_connect_desktop_button(&mut self, ui: &mut Ui) {
        ui.visuals_mut().widgets.hovered.rounding = Rounding {
            nw: 2.0,
            ne: 0.0,
            sw: 2.0,
            se: 0.0,
        };

        ui.visuals_mut().widgets.inactive.rounding = Rounding {
            nw: 2.0,
            ne: 0.0,
            sw: 2.0,
            se: 0.0,
        };

        ui.visuals_mut().widgets.active.rounding = Rounding {
            nw: 2.0,
            ne: 0.0,
            sw: 2.0,
            se: 0.0,
        };

        let (rect, response) = ui.allocate_exact_size(ui.available_size(), Sense::click());

        ui.allocate_ui_at_rect(rect, |ui| {
            if self.state.connect_page_desktop_connecting() {
                ui.painter().rect_filled(
                    rect,
                    ui.visuals().widgets.active.rounding,
                    ui.visuals().widgets.active.bg_fill,
                );
                ui.add_enabled(false, Spinner::default())
            } else {
                // let visuals = ui.style().interact(&response);
                // ui.painter()
                //     .rect_filled(rect, visuals.rounding, visuals.bg_fill);

                // let text = WidgetText::from("桌面").into_galley(
                //     ui,
                //     None,
                //     ui.available_width(),
                //     FontId::proportional(28.0),
                // );

                // ui.painter().add(TextShape {
                //     pos: rect.left_top() + ((ui.available_size() - text.size()) / 2.0),
                //     galley: text.galley,
                //     underline: Stroke::none(),
                //     override_text_color: None,
                //     angle: 0.0,
                // });

                ui.button("桌面")
            }
        });

        if response.clicked() && !self.state.connect_page_desktop_connecting() {
            self.connect_desktop();
        }
    }

    fn connect_desktop(&mut self) {
        let input_device_id = self.state.connect_page_visit_device_id().to_string();
        tracing::info!("visit device id: {:?}", input_device_id);

        if input_device_id.len() != 10 || !input_device_id.chars().all(|c| c.is_ascii_digit()) {
            self.state_updater
                .update_last_error(core_error!("Invalid visit device ID"));
            return;
        }

        let input_device_id: i64 = match input_device_id.parse() {
            Ok(v) => v,
            Err(_) => {
                self.state_updater
                    .update_last_error(core_error!("Invalid visit device ID format"));
                return;
            }
        };

        let config = match self.state.config() {
            Some(config) => config,
            None => {
                self.state_updater.update_last_error(core_error!(
                    "Config hasn't initialized, please try it later"
                ));
                return;
            }
        };

        let domain_config = match config.domain_configs.get(&config.primary_domain) {
            Some(domain_config) => domain_config,
            None => {
                self.state_updater.update_last_error(core_error!(
                    "Config hasn't initialized, please try it later"
                ));
                return;
            }
        };

        if domain_config.device_id == input_device_id {
            // todo: popup dialog
            return;
        }

        let local_device_id = domain_config.device_id;

        self.state_updater.emit_signaling_visit(
            local_device_id,
            input_device_id,
            ResourceType::Desktop,
        );
    }

    fn make_connect_file_manager_button(&mut self, ui: &mut Ui) {
        ui.visuals_mut().widgets.hovered.rounding = Rounding {
            nw: 0.0,
            ne: 2.0,
            sw: 0.0,
            se: 2.0,
        };

        ui.visuals_mut().widgets.inactive.rounding = Rounding {
            nw: 0.0,
            ne: 2.0,
            sw: 0.0,
            se: 2.0,
        };

        ui.visuals_mut().widgets.active.rounding = Rounding {
            nw: 0.0,
            ne: 2.0,
            sw: 0.0,
            se: 2.0,
        };

        if Button::new("File Manager").ui(ui).clicked() {}
    }
}

impl ConnectPage<'_> {
    pub fn show(&mut self, ui: &mut egui::Ui) {
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
                        let mut input_field =
                            DeviceIDInputField::new(self.state, &mut self.state_updater);
                        ui.add_sized(Vec2::new(0.0, 60.0), &mut input_field);
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
                                                self.make_connect_file_manager_button(ui);
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
fn make_password_toolbar_edit_button(ui: &mut Ui) -> Response {
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
fn make_password_toolbar_visible_button(ui: &mut Ui) -> Response {
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
