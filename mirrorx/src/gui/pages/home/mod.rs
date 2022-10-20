mod connect;
mod history;
mod lan;

use super::View;
use crate::gui::{
    state::{AppState, AppStateUpdater},
    widgets::{custom_toasts::CustomToasts, dialog::Dialog},
};
use connect::ConnectPage;
use egui::{
    epaint::{Color32, FontId, Pos2, Rect, Stroke, Vec2},
    style::Margin,
    CentralPanel, Context, Frame, Label, RichText, Rounding, TextEdit, Ui, Widget,
};
use egui_extras::{Size, StripBuilder};
use history::HistoryPage;
use lan::LANPage;
use mirrorx_core::{
    api::{
        config::{Config, DomainConfig},
        signaling::ResourceType,
    },
    core_error,
};
use std::collections::HashMap;

pub struct HomeView {
    app_state: AppState,
    app_state_updater: AppStateUpdater,
    init_once: std::sync::Once,
    custom_toasts: CustomToasts,
}

impl HomeView {
    pub fn new() -> Self {
        let state = AppState::new("Connect");
        let state_updater = state.new_state_updater();

        Self {
            app_state: state,
            app_state_updater: state_updater,
            init_once: std::sync::Once::new(),
            custom_toasts: CustomToasts::new(),
        }
    }

    fn init_once(&mut self) {
        if self.init_once.is_completed() {
            return;
        }

        let state_updater = self.app_state.new_state_updater();
        self.init_once.call_once(move || {
            tokio::task::spawn_blocking(move || {
                let base_dir_path = match directories_next::BaseDirs::new() {
                    Some(base_dir_path) => base_dir_path,
                    None => {
                        state_updater.update_last_error(core_error!("get config base dir failed"));
                        return;
                    }
                };

                let dir_path = base_dir_path.data_dir().join("MirrorX");
                if let Err(err) = std::fs::create_dir_all(dir_path.clone()) {
                    state_updater
                        .update_last_error(core_error!("create config dir failed ({:?})", err));
                    return;
                }

                let config_file_path = dir_path.join("mirrorx.db");

                match mirrorx_core::api::config::read(config_file_path.as_ref()) {
                    Ok(config) => {
                        let config = match config {
                            Some(config) => config,
                            None => {
                                let mut domain_configs = HashMap::new();
                                domain_configs.insert(
                                String::from("MirrorX.cloud"),
                                DomainConfig {
                                    addr: String::from("tcp://192.168.0.101:28000"),
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

                                if let Err(err) = mirrorx_core::api::config::save(
                                    config_file_path.as_ref(),
                                    &default_config,
                                ) {
                                    state_updater.update_last_error(core_error!(
                                        "save config failed ({:?})",
                                        err
                                    ));
                                }

                                default_config
                            }
                        };

                        state_updater.update_config_path(config_file_path.as_path());
                        state_updater.update_config(&config);
                    }
                    Err(err) => {
                        state_updater
                            .update_last_error(core_error!("read config failed ({:?})", err));
                    }
                };
            });
        });
    }

    fn build_panel(&mut self, ui: &mut Ui) {
        ui.spacing_mut().item_spacing = Vec2::ZERO;
        StripBuilder::new(ui)
            .size(Size::relative(0.06)) // Domain Title
            .size(Size::relative(0.09)) // Domain
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
                        if let Some(config) = self.app_state.config() {
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
                &mut (self.app_state.current_page_name() == toggle_tab_value),
                display_text,
            );

            if toggle.clicked() {
                self.app_state_updater
                    .update_current_page_name(toggle_tab_value);
            }
        });
    }

    fn build_tab_view(&mut self, ui: &mut Ui) {
        match self.app_state.current_page_name() {
            "Connect" => ConnectPage::new(&self.app_state).show(ui),
            "LAN" => LANPage::new().show(ui),
            "History" => HistoryPage::new().show(ui),
            _ => panic!("unknown select page tab"),
        }
    }

    fn build_dialog_visit_request(&mut self, ui: &mut Ui) {
        if let Some((active_device_id, passive_device_id, resource_type)) =
            self.app_state.dialog_visit_request_visible()
        {
            Dialog::new("MirrorX Visit Request", Vec2::new(280.0, 140.0)).show(ui, |ui| {
                ui.style_mut().spacing.item_spacing = Vec2::ZERO;
                StripBuilder::new(ui)
                    .size(Size::relative(0.75))
                    .size(Size::remainder())
                    .vertical(|mut strip| {
                        strip.cell(|ui| {
                            ui.centered_and_justified(|ui| {
                                let mut device_id_str = format!("{:0>10}", active_device_id);
                                device_id_str.insert(2, '-');
                                device_id_str.insert(7, '-');

                                Label::new(
                                    RichText::new(format!(
                                        "{}\nwant to visit your {}",
                                        device_id_str,
                                        if let ResourceType::Desktop = resource_type {
                                            "desktop"
                                        } else {
                                            "files"
                                        }
                                    ))
                                    .font(FontId::proportional(20.0)),
                                )
                                .ui(ui);
                            });
                        });

                        strip.strip(|builder| {
                            builder
                                .sizes(Size::relative(0.5), 2)
                                .horizontal(|mut strip| {
                                    strip.cell(|ui| {
                                        ui.centered_and_justified(|ui| {
                                            ui.style_mut().spacing.window_margin = Margin {
                                                left: 1.0,
                                                right: 0.0,
                                                top: 0.0,
                                                bottom: 1.0,
                                            };

                                            ui.visuals_mut().widgets.hovered.expansion = 0.0;
                                            ui.visuals_mut().widgets.hovered.bg_stroke =
                                                Stroke::none();
                                            ui.visuals_mut().widgets.hovered.bg_fill =
                                                Color32::from_rgb(0x19, 0x8C, 0xFF);
                                            ui.visuals_mut().widgets.hovered.fg_stroke =
                                                Stroke::new(1.0, Color32::WHITE);
                                            ui.visuals_mut().widgets.hovered.rounding = Rounding {
                                                nw: 0.0,
                                                ne: 0.0,
                                                sw: 2.0,
                                                se: 0.0,
                                            };

                                            ui.visuals_mut().widgets.inactive.expansion = 0.0;
                                            ui.visuals_mut().widgets.inactive.bg_stroke =
                                                Stroke::none();
                                            ui.visuals_mut().widgets.inactive.bg_fill =
                                                Color32::from_rgb(0x01, 0x6F, 0xFF);
                                            ui.visuals_mut().widgets.inactive.fg_stroke =
                                                Stroke::new(1.0, Color32::WHITE);
                                            ui.visuals_mut().widgets.inactive.rounding = Rounding {
                                                nw: 0.0,
                                                ne: 0.0,
                                                sw: 2.0,
                                                se: 0.0,
                                            };

                                            ui.visuals_mut().widgets.active.expansion = 0.0;
                                            ui.visuals_mut().widgets.active.bg_stroke =
                                                Stroke::none();
                                            ui.visuals_mut().widgets.active.bg_fill =
                                                Color32::from_rgb(0x00, 0x54, 0xE6);
                                            ui.visuals_mut().widgets.active.fg_stroke =
                                                Stroke::new(1.0, Color32::WHITE);
                                            ui.visuals_mut().widgets.active.rounding = Rounding {
                                                nw: 0.0,
                                                ne: 0.0,
                                                sw: 2.0,
                                                se: 0.0,
                                            };

                                            if ui.button("Allow").clicked() {
                                                self.app_state_updater.emit_signaling_visit_reply(
                                                    true,
                                                    *active_device_id,
                                                    *passive_device_id,
                                                );
                                                self.app_state_updater
                                                    .update_dialog_visit_request_visible(None);
                                            }
                                        });
                                    });
                                    strip.cell(|ui| {
                                        ui.centered_and_justified(|ui| {
                                            ui.style_mut().spacing.window_margin = Margin {
                                                left: 0.0,
                                                right: 1.0,
                                                top: 0.0,
                                                bottom: 1.0,
                                            };

                                            ui.visuals_mut().widgets.hovered.expansion = 0.0;
                                            ui.visuals_mut().widgets.hovered.bg_stroke =
                                                Stroke::none();
                                            ui.visuals_mut().widgets.hovered.rounding = Rounding {
                                                nw: 0.0,
                                                ne: 0.0,
                                                sw: 0.0,
                                                se: 2.0,
                                            };

                                            ui.visuals_mut().widgets.inactive.expansion = 0.0;
                                            ui.visuals_mut().widgets.inactive.bg_stroke =
                                                Stroke::none();
                                            ui.visuals_mut().widgets.inactive.rounding = Rounding {
                                                nw: 0.0,
                                                ne: 0.0,
                                                sw: 0.0,
                                                se: 2.0,
                                            };

                                            ui.visuals_mut().widgets.active.expansion = 0.0;
                                            ui.visuals_mut().widgets.active.bg_stroke =
                                                Stroke::none();
                                            ui.visuals_mut().widgets.active.rounding = Rounding {
                                                nw: 0.0,
                                                ne: 0.0,
                                                sw: 0.0,
                                                se: 2.0,
                                            };

                                            if ui.button("Reject").clicked() {
                                                self.app_state_updater.emit_signaling_visit_reply(
                                                    false,
                                                    *active_device_id,
                                                    *passive_device_id,
                                                );
                                                self.app_state_updater
                                                    .update_dialog_visit_request_visible(None);
                                            }
                                        });
                                    });
                                });
                        });
                    });
            });
        }
    }

    fn build_dialog_visit_password_input(&mut self, ui: &mut Ui) {
        if let Some((active_device_id, passive_device_id)) =
            self.app_state.dialog_input_visit_password_visible()
        {
            Dialog::new("MirrorX Visit Password Input", Vec2::new(280.0, 140.0)).show(ui, |ui| {
                ui.style_mut().spacing.item_spacing = Vec2::ZERO;
                StripBuilder::new(ui)
                    .size(Size::relative(0.75))
                    .size(Size::remainder())
                    .vertical(|mut strip| {
                        strip.strip(|builder| {
                            builder.sizes(Size::relative(0.5), 2).vertical(|mut strip| {
                                strip.cell(|ui| {
                                    ui.centered_and_justified(|ui| {
                                        ui.label(
                                            RichText::new("Please input remote device password")
                                                .font(FontId::proportional(18.0)),
                                        );
                                    });
                                });
                                strip.cell(|ui| {
                                    ui.centered_and_justified(|ui| {
                                        ui.visuals_mut().widgets.inactive =
                                            ui.visuals_mut().widgets.active;

                                        let mut snapshot_password = self
                                            .app_state
                                            .dialog_input_visit_password()
                                            .to_string();

                                        Frame::default().outer_margin(Margin::same(12.0)).show(
                                            ui,
                                            |ui| {
                                                if TextEdit::singleline(&mut snapshot_password)
                                                    .font(FontId::proportional(22.0))
                                                    .password(true)
                                                    .show(ui)
                                                    .response
                                                    .changed()
                                                {
                                                    self.app_state_updater
                                                        .update_dialog_input_visit_password(
                                                            &snapshot_password,
                                                        );
                                                }
                                            },
                                        );
                                    });
                                });
                            });
                        });

                        strip.strip(|builder| {
                            builder
                                .sizes(Size::relative(0.5), 2)
                                .horizontal(|mut strip| {
                                    strip.cell(|ui| {
                                        ui.centered_and_justified(|ui| {
                                            // ui.visuals_mut().button_frame = false;

                                            ui.visuals_mut().widgets.hovered.expansion = 0.0;
                                            ui.visuals_mut().widgets.hovered.bg_stroke =
                                                Stroke::none();
                                            ui.visuals_mut().widgets.hovered.bg_fill =
                                                Color32::from_rgb(0x19, 0x8C, 0xFF);
                                            ui.visuals_mut().widgets.hovered.fg_stroke =
                                                Stroke::new(1.0, Color32::WHITE);
                                            ui.visuals_mut().widgets.hovered.rounding = Rounding {
                                                nw: 0.0,
                                                ne: 0.0,
                                                sw: 2.0,
                                                se: 0.0,
                                            };

                                            ui.visuals_mut().widgets.inactive.expansion = 0.0;
                                            ui.visuals_mut().widgets.inactive.bg_stroke =
                                                Stroke::none();
                                            ui.visuals_mut().widgets.inactive.bg_fill =
                                                Color32::from_rgb(0x01, 0x6F, 0xFF);
                                            ui.visuals_mut().widgets.inactive.fg_stroke =
                                                Stroke::new(1.0, Color32::WHITE);
                                            ui.visuals_mut().widgets.inactive.rounding = Rounding {
                                                nw: 0.0,
                                                ne: 0.0,
                                                sw: 2.0,
                                                se: 0.0,
                                            };

                                            ui.visuals_mut().widgets.active.expansion = 0.0;
                                            ui.visuals_mut().widgets.active.bg_stroke =
                                                Stroke::none();
                                            ui.visuals_mut().widgets.active.bg_fill =
                                                Color32::from_rgb(0x00, 0x54, 0xE6);
                                            ui.visuals_mut().widgets.active.fg_stroke =
                                                Stroke::new(1.0, Color32::WHITE);
                                            ui.visuals_mut().widgets.active.rounding = Rounding {
                                                nw: 0.0,
                                                ne: 0.0,
                                                sw: 2.0,
                                                se: 0.0,
                                            };

                                            if ui.button("Ok").clicked() {
                                                self.app_state_updater.emit_signaling_key_exchange(
                                                    active_device_id,
                                                    passive_device_id,
                                                );
                                            }
                                        });
                                    });
                                    strip.cell(|ui| {
                                        ui.centered_and_justified(|ui| {
                                            ui.visuals_mut().widgets.hovered.expansion = 0.0;
                                            ui.visuals_mut().widgets.hovered.bg_stroke =
                                                Stroke::none();
                                            ui.visuals_mut().widgets.hovered.rounding = Rounding {
                                                nw: 0.0,
                                                ne: 0.0,
                                                sw: 0.0,
                                                se: 2.0,
                                            };

                                            ui.visuals_mut().widgets.inactive.expansion = 0.0;
                                            ui.visuals_mut().widgets.inactive.bg_stroke =
                                                Stroke::none();
                                            ui.visuals_mut().widgets.inactive.rounding = Rounding {
                                                nw: 0.0,
                                                ne: 0.0,
                                                sw: 0.0,
                                                se: 2.0,
                                            };

                                            ui.visuals_mut().widgets.active.expansion = 0.0;
                                            ui.visuals_mut().widgets.active.bg_stroke =
                                                Stroke::none();
                                            ui.visuals_mut().widgets.active.rounding = Rounding {
                                                nw: 0.0,
                                                ne: 0.0,
                                                sw: 0.0,
                                                se: 2.0,
                                            };

                                            if ui.button("Cancel").clicked() {
                                                self.app_state_updater
                                                    .update_dialog_input_visit_password_visible(
                                                        None,
                                                    );
                                                self.app_state_updater
                                                    .update_dialog_input_visit_password("");
                                                self.app_state_updater
                                                    .update_connect_page_desktop_connecting(false);
                                            }
                                        });
                                    });
                                });
                        });
                    });
            });
        }
    }
}

impl View for HomeView {
    fn ui(&mut self, ctx: &Context) {
        ctx.request_repaint_after(std::time::Duration::from_secs(1));

        self.init_once();

        let frame = Frame::default()
            .inner_margin(Margin::symmetric(8.0, 0.0))
            .fill(ctx.style().visuals.window_fill());

        CentralPanel::default().frame(frame).show(ctx, |ui| {
            self.build_panel(ui);
            self.build_dialog_visit_request(ui);
            self.build_dialog_visit_password_input(ui);
            if let Some(err) = self.app_state.handle_event() {
                self.custom_toasts.error(err.to_string().as_str());
            }
            self.custom_toasts.show(ctx);
        });
    }
}
