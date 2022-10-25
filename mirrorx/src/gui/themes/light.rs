use egui::{
    style::{Selection, WidgetVisuals, Widgets},
    *,
};
use once_cell::sync::Lazy;

// color schema from https://colorffy.com/palettes/Lxl3iof66tvMUR8o0e0e

pub static TEXT_COLOR: Color32 = Color32::from_rgb(0x1F, 0x1F, 0x1F);
pub static BACKGROUND_COLOR: Color32 = Color32::from_rgb(0xEB, 0xEB, 0xEB);

// color schema from https://colorffy.com/palettes/erxzqnz4XD05XyTYlg9Z

pub static PRIMARY_COLOR: Color32 = Color32::from_rgb(0x19, 0x84, 0xE1);
pub static PRIMARY_COLOR_DARKER: Color32 = Color32::from_rgb(0x1F, 0x70, 0xBD);
pub static PRIMARY_COLOR_LIGHTER: Color32 = Color32::from_rgb(0x54, 0x94, 0xE6);

pub static THEME: Lazy<egui::Style> = Lazy::new(|| egui::Style {
    visuals: egui::Visuals {
        dark_mode: false,
        hyperlink_color: PRIMARY_COLOR,
        widgets: Widgets {
            noninteractive: WidgetVisuals {
                bg_fill: BACKGROUND_COLOR,
                bg_stroke: Stroke::none(),
                rounding: Rounding::none(),
                fg_stroke: Stroke::new(1.0, TEXT_COLOR),
                expansion: 0.0,
            },
            inactive: WidgetVisuals {
                bg_fill: PRIMARY_COLOR,
                bg_stroke: Stroke::none(),
                rounding: Rounding::same(2.0),
                fg_stroke: Stroke::new(1.0, Color32::WHITE),
                expansion: 0.0,
            },
            hovered: WidgetVisuals {
                bg_fill: PRIMARY_COLOR_LIGHTER,
                bg_stroke: Stroke::none(),
                rounding: Rounding::same(2.0),
                fg_stroke: Stroke::new(1.0, Color32::WHITE),
                expansion: 0.0,
            },
            active: WidgetVisuals {
                bg_fill: PRIMARY_COLOR_DARKER,
                bg_stroke: Stroke::none(),
                rounding: Rounding::same(2.0),
                fg_stroke: Stroke::new(1.0, Color32::WHITE),
                expansion: 0.0,
            },
            ..Default::default()
        },
        selection: Selection {
            bg_fill: PRIMARY_COLOR_LIGHTER,
            stroke: Stroke::new(1.0, TEXT_COLOR),
        },
        extreme_bg_color: BACKGROUND_COLOR,
        ..Default::default()
    },
    ..Default::default()
});
