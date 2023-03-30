use super::View;
use crate::frame::{
    asset::StaticImageCache,
    state::SharedState,
    widget::StatefulWidget,
    widgets::{modal::Modal, my_device_card::MyDeviceCard, peer_connect::PeerConnectWidget},
};
use eframe::egui::*;
use egui_extras::{Size, StripBuilder};

#[derive(Default)]
pub struct HomePage {
    peer_connect: PeerConnectWidget,
}

impl HomePage {
    fn draw_status_bar(&mut self, ui: &mut Ui, ui_state: &SharedState) {
        ui.painter().rect(
            ui.available_rect_before_wrap(),
            Rounding::none(),
            ui_state.theme_color().background_level2,
            Stroke::NONE,
        );

        let available_rect = ui.available_rect_before_wrap().shrink(12.0);
        ui.allocate_ui_at_rect(available_rect, |ui| {
            ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                StaticImageCache::current()
                    .github
                    .show_size(ui, vec2(20.0, 20.0));
                ui.add_space(12.0);
                // light/dark mode
                draw_theme_switch_button(ui, ui_state);
                ui.add_space(12.0);
                ui.label(RichText::new("\u{e1ba}").size(20.0)); // latency

                // draw right connect panel
                let (right_rect, _) = ui.allocate_exact_size(ui.available_size(), Sense::click());

                ui.allocate_ui_at_rect(right_rect, |ui| {
                    ui.with_layout(
                        Layout::right_to_left(Align::Center).with_main_align(Align::Max),
                        |ui| {
                            self.peer_connect.draw(ui, ui_state);
                        },
                    );
                });
            });
        });
    }

    fn draw_panel(&mut self, ui: &mut Ui, ui_state: &SharedState) {
        let (rect, _) = ui.allocate_exact_size(vec2(780.0, 480.0), Sense::hover());

        ui.allocate_ui_at_rect(rect, |ui| {
            StripBuilder::new(ui)
                .size(Size::relative(0.4))
                .size(Size::relative(0.03))
                .size(Size::remainder())
                .horizontal(|mut strip| {
                    strip.cell(|ui| {
                        self.draw_panel_left(ui, ui_state);
                    });
                    strip.cell(|ui| {
                        ui.add(Separator::default().vertical());
                    });
                    strip.cell(|ui| {
                        self.draw_panel_right(ui, ui_state);
                    });
                });
        });
    }

    fn draw_panel_left(&mut self, ui: &mut Ui, ui_state: &SharedState) {
        ui.centered_and_justified(|ui| {
            ui.with_layout(Layout::top_down(Align::Min), |ui| {
                ui.label(RichText::new("Peer ID").size(32.0));

                ui.add_space(18.0);
                ui.label(
                    RichText::new(format!("{}#{}", ui_state.peer_id, ui_state.peer_domain))
                        .size(18.0),
                );

                ui.add_space(18.0);
                ui.label(RichText::new("Password").size(32.0));

                ui.add_space(18.0);
                ui.checkbox(
                    &mut ui_state.use_totp_password,
                    RichText::new("Use Time Based OTP").size(18.0),
                );

                ui.add_space(18.0);
                ui.vertical_centered(|ui| {
                    ui.label(RichText::new(&ui_state.totp_password).size(24.0));
                });

                ui.add_space(18.0);
                ui.checkbox(
                    &mut ui_state.use_otp_password,
                    RichText::new("Use One-Time Password").size(18.0),
                );

                ui.add_space(18.0);
                ui.vertical_centered(|ui| {
                    ui.label(RichText::new(&ui_state.otp_password).size(24.0));
                });

                // ui.add_space(18.0);
                // ui.checkbox(
                //     &mut ui_state.use_permanent_password,
                //     RichText::new("Use Permanent Password").size(18.0),
                // );

                // ui.add_space(18.0);
                // ui.vertical_centered(|ui| {
                //     ui.label(RichText::new(&ui_state.permanent_password).size(24.0));
                // });
            })
        });
    }

    fn draw_panel_right(&mut self, ui: &mut Ui, ui_state: &SharedState) {
        ui.vertical(|ui| {
            ui.label(RichText::new("My Devices").size(32.0));

            // let mut devices_card_rect = ui.available_rect_before_wrap();
            // devices_card_rect.set_height(if ui_state.my_devices.is_empty() {
            //     80.0
            // } else {
            //     200.0
            // });

            ui.allocate_ui_at_rect(devices_card_rect, |ui| {
                if ui_state.is_login {
                    ScrollArea::horizontal()
                        .auto_shrink([false, false])
                        .always_show_scroll(false)
                        .show(ui, |ui| {
                            ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                                ui.style_mut().spacing.item_spacing = vec2(4.0, 0.0);
                                // for device in ui_state.my_devices.iter_mut() {
                                //     MyDeviceCard::draw(ui, device);
                                // }
                            });
                        });
                } else {
                    let modal = Modal::new("login_modal", ui.ctx());
                    modal.show(
                        "Login",
                        |ui| {
                            show_modal_login(ui, ui_state, &modal);
                        },
                        &[],
                    );

                    ui.centered_and_justified(|ui| {
                        if ui
                            .link(
                                RichText::new("Login to see account associated online devices")
                                    .size(16.0),
                            )
                            .clicked()
                        {
                            modal.open();
                        }
                    });
                }
            });

            ui.add_space(8.0);
            ui.separator();

            ui.label(RichText::new("Recent Connect").size(32.0));
        });
    }
}

fn draw_theme_switch_button(ui: &mut Ui, ui_state: &SharedState) {
    let title = match ui_state.theme_color().style {
        crate::frame::color::ThemeColorStyle::Light => "\u{e518}",
        crate::frame::color::ThemeColorStyle::Dark => "\u{e51c}",
    };

    if ui
        .add(Button::new(RichText::new(title).size(20.0)).frame(false))
        .on_hover_cursor(CursorIcon::PointingHand)
        .on_hover_text("Switch Theme Style")
        .clicked()
    {
        ui_state.switch_theme_style();
    }
}

impl StatefulWidget for HomePage {
    fn update_state(&mut self, shared_state: &SharedState) {
        //
    }

    fn update_view(&self, ui: &mut Ui) {
        let central_content_height = ui.available_height() - 58.0;
        StripBuilder::new(ui)
            .size(Size::exact(central_content_height))
            .size(Size::remainder())
            .vertical(|mut strip| {
                strip.cell(|ui| {
                    ui.centered_and_justified(|ui| {
                        self.draw_panel(ui, ui_state);
                    });
                });
                strip.cell(|ui| self.draw_status_bar(ui, ui_state));
            });
    }
}

fn show_modal_login(ui: &mut Ui, ui_state: &SharedState, modal: &Modal) -> InnerResponse<()> {
    ui.vertical_centered_justified(|ui| {
        ui.allocate_ui_with_layout(
            vec2(160.0, 26.0),
            Layout::left_to_right(Align::Center),
            |ui| {
                ui.add_sized(
                    Vec2::splat(20.0),
                    Label::new(RichText::new("\u{e853}").size(20.0)),
                );

                ui.add_sized(
                    ui.available_size(),
                    TextEdit::singleline(&mut ui_state.login_email)
                        .hint_text("Email")
                        .font(FontId::proportional(20.0))
                        .vertical_align(Align::Center),
                );
            },
        );

        ui.add_space(4.0);

        ui.allocate_ui_with_layout(
            vec2(160.0, 26.0),
            Layout::left_to_right(Align::Center),
            |ui| {
                ui.add_sized(
                    Vec2::splat(20.0),
                    Label::new(RichText::new("\u{f042}").size(20.0)),
                );

                ui.add_sized(
                    ui.available_size(),
                    TextEdit::singleline(&mut ui_state.login_password)
                        .hint_text("Password")
                        .font(FontId::proportional(20.0))
                        .vertical_align(Align::Center)
                        .password(true),
                );
            },
        );

        ui.add_space(4.0);

        ui.allocate_ui(vec2(ui.available_width(), 38.0), |ui| {
            StripBuilder::new(ui)
                .size(Size::relative(0.5))
                .size(Size::remainder())
                .horizontal(|mut strip| {
                    strip.cell(|ui| {
                        ui.centered_and_justified(|ui| {
                            ui.style_mut().visuals.widgets.active.expansion = 0.0;
                            if ui
                                .add(
                                    Button::new(
                                        RichText::new("Login").size(18.0).color(Color32::WHITE),
                                    )
                                    .stroke(Stroke::new(1.0, Color32::WHITE))
                                    .fill(Color32::from_rgb(57, 117, 226)),
                                )
                                .clicked()
                            {
                                // ui_state.notifications.push_notification("dS".to_string());
                            };
                        });
                    });
                    strip.cell(|ui| {
                        ui.centered_and_justified(|ui| {
                            if ui.button(RichText::new("Cancel").size(18.0)).clicked() {
                                modal.close();
                            }
                        });
                    });
                });
        });

        ui.add(Separator::default().spacing(4.0));

        ui.add_sized(
            vec2(ui.available_width(), 38.0),
            Button::new(RichText::new("Login with Github").size(18.0)),
        );

        ui.add_space(4.0);

        ui.add_sized(
            vec2(ui.available_width(), 38.0),
            Button::new(RichText::new("Login with Google").size(18.0)),
        );
    })
}
