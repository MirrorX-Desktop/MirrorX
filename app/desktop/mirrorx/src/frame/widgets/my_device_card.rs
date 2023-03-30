use crate::frame::{
    asset::StaticImageCache,
    state::{MyDevice, MyDeviceType},
};
use eframe::{egui::*, epaint::TextShape};
use egui_extras::{Size, StripBuilder};

pub struct MyDeviceCard {}

impl MyDeviceCard {
    pub fn draw(ui: &mut Ui, device: &MyDevice) {
        let (rect, response) = ui.allocate_exact_size(vec2(120.0, 160.0), Sense::hover());

        ui.allocate_ui_at_rect(rect, |ui| {
            StripBuilder::new(ui)
                .size(Size::exact(70.0))
                .size(Size::exact(50.0))
                .size(Size::remainder())
                .vertical(|mut strip| {
                    strip.cell(|ui| {
                        ui.with_layout(Layout::bottom_up(Align::Center), |ui| {
                            ui.label(
                                RichText::new(if device.is_this_computer {
                                    "(This Computer)"
                                } else {
                                    "(Your Device)"
                                })
                                .size(11.0),
                            );

                            ui.centered_and_justified(|ui| {
                                let image = match device.device_type {
                                    MyDeviceType::Computer => &StaticImageCache::current().dvr_48,
                                    MyDeviceType::Phone => {
                                        &StaticImageCache::current().smart_phone_48
                                    }
                                };

                                ui.image(image.texture_id(ui.ctx()), vec2(36.0, 36.0));
                            });
                        });
                    });

                    strip.cell(|ui| {
                        ui.centered_and_justified(|ui| ui.label(RichText::new(&device.name)));
                    });

                    strip.cell(|ui| {
                        if !device.is_this_computer {
                            ui.allocate_ui_at_rect(
                                Rect::from_center_size(
                                    ui.max_rect().center(),
                                    vec2(70.0, ui.available_height()),
                                ),
                                |ui| {
                                    MyDeviceCardConnectButtonGroup::default().ui(ui);
                                },
                            );
                        }
                    });
                });
        });

        ui.painter().rect_stroke(
            rect,
            Rounding::same(3.0),
            if response.hovered() {
                ui.style().visuals.widgets.active.bg_stroke
            } else {
                ui.style().visuals.widgets.noninteractive.bg_stroke
            },
        );
    }
}

#[derive(Default)]
pub struct MyDeviceCardConnectButtonGroup {}

impl Widget for MyDeviceCardConnectButtonGroup {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.style_mut().spacing.item_spacing = Vec2::ZERO;

        let width = ui.available_width() / 2.0;

        let (desktop_rect, desktop_response) =
            ui.allocate_exact_size(vec2(width, 24.0), Sense::click());

        let (files_rect, files_response) =
            ui.allocate_exact_size(vec2(width, 24.0), Sense::click());

        let (_, desktop_galley, _) = Label::new("\u{e30c}").layout_in_ui(ui);
        let desktop_icon_rect =
            Rect::from_center_size(desktop_rect.center(), desktop_galley.size());

        let (_, files_galley, _) = Label::new("\u{e2c7}").layout_in_ui(ui);
        let files_icon_rect = Rect::from_center_size(files_rect.center(), files_galley.size());

        let desktop_rounding = Rounding {
            nw: 3.0,
            ne: 0.0,
            sw: 3.0,
            se: 0.0,
        };

        let files_rounding = Rounding {
            nw: 0.0,
            ne: 3.0,
            sw: 0.0,
            se: 3.0,
        };

        let desktop_visuals = ui.style().interact(&desktop_response);
        let files_visuals = ui.style().interact(&files_response);

        ui.painter()
            .rect_filled(desktop_rect, desktop_rounding, desktop_visuals.bg_fill);

        ui.painter()
            .rect_filled(files_rect, files_rounding, files_visuals.bg_fill);

        ui.painter().add(TextShape {
            pos: desktop_icon_rect.min,
            galley: desktop_galley.galley().clone(),
            underline: Stroke::NONE,
            override_text_color: Some(desktop_visuals.fg_stroke.color),
            angle: 0.0,
        });

        ui.painter().add(TextShape {
            pos: files_icon_rect.min,
            galley: files_galley.galley().clone(),
            underline: Stroke::NONE,
            override_text_color: Some(files_visuals.fg_stroke.color),
            angle: 0.0,
        });

        if desktop_response.hovered() || desktop_response.clicked() {
            ui.painter()
                .rect_stroke(desktop_rect, desktop_rounding, desktop_visuals.bg_stroke);
        }

        if files_response.hovered() || files_response.clicked() {
            ui.painter()
                .rect_stroke(files_rect, files_rounding, files_visuals.bg_stroke);
        }

        desktop_response.union(files_response)
    }
}
