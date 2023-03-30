// colors are derive from https://mui.com/joy-ui/customization/theme-colors/
#![allow(unused)]

use eframe::epaint::Color32;
use once_cell::sync::Lazy;
use std::rc::Rc;

macro_rules! hex_color {
    ($color:literal) => {{
        let color_array = color_hex::color_from_hex!($color);
        Color32::from_rgb(color_array[0], color_array[1], color_array[2])
    }};
}

#[derive(Debug, Clone, Copy)]
pub enum ThemeColorStyle {
    Light,
    Dark,
}

pub struct ThemeColor {
    pub style: ThemeColorStyle,
    pub background_backdrop: Color32,
    pub background_body: Color32,
    pub background_level1: Color32,
    pub background_level2: Color32,
    pub background_level3: Color32,
    pub background_popup: Color32,
    pub background_surface: Color32,
    pub background_tooltip: Color32,
    pub common_black: Color32,
    pub common_white: Color32,
    pub danger_50: Color32,
    pub danger_100: Color32,
    pub danger_200: Color32,
    pub danger_300: Color32,
    pub danger_400: Color32,
    pub danger_500: Color32,
    pub danger_600: Color32,
    pub danger_700: Color32,
    pub danger_800: Color32,
    pub danger_900: Color32,
    pub danger_outlined_active_bg: Color32,
    pub danger_outlined_border: Color32,
    pub danger_outlined_color: Color32,
    pub danger_outlined_disabled_border: Color32,
    pub danger_outlined_disabled_color: Color32,
    pub danger_outlined_hover_bg: Color32,
    pub danger_outlined_hover_border: Color32,
    pub danger_plain_active_bg: Color32,
    pub danger_plain_color: Color32,
    pub danger_plain_disabled_color: Color32,
    pub danger_plain_hover_bg: Color32,
    pub danger_soft_active_bg: Color32,
    pub danger_soft_bg: Color32,
    pub danger_soft_color: Color32,
    pub danger_soft_disabled_bg: Color32,
    pub danger_soft_disabled_color: Color32,
    pub danger_soft_hover_bg: Color32,
    pub danger_solid_active_bg: Color32,
    pub danger_solid_bg: Color32,
    pub danger_solid_color: Color32,
    pub danger_solid_disabled_bg: Color32,
    pub danger_solid_disabled_color: Color32,
    pub danger_solid_hover_bg: Color32,
    pub divider: Color32,
    pub focus_visible: Color32,
    pub info_50: Color32,
    pub info_100: Color32,
    pub info_200: Color32,
    pub info_300: Color32,
    pub info_400: Color32,
    pub info_500: Color32,
    pub info_600: Color32,
    pub info_700: Color32,
    pub info_800: Color32,
    pub info_900: Color32,
    pub info_outlined_active_bg: Color32,
    pub info_outlined_border: Color32,
    pub info_outlined_color: Color32,
    pub info_outlined_disabled_border: Color32,
    pub info_outlined_disabled_color: Color32,
    pub info_outlined_hover_bg: Color32,
    pub info_outlined_hover_border: Color32,
    pub info_plain_active_bg: Color32,
    pub info_plain_color: Color32,
    pub info_plain_disabled_color: Color32,
    pub info_plain_hover_bg: Color32,
    pub info_soft_active_bg: Color32,
    pub info_soft_bg: Color32,
    pub info_soft_color: Color32,
    pub info_soft_disabled_bg: Color32,
    pub info_soft_disabled_color: Color32,
    pub info_soft_hover_bg: Color32,
    pub info_solid_active_bg: Color32,
    pub info_solid_bg: Color32,
    pub info_solid_color: Color32,
    pub info_solid_disabled_bg: Color32,
    pub info_solid_disabled_color: Color32,
    pub info_solid_hover_bg: Color32,
    pub neutral_50: Color32,
    pub neutral_100: Color32,
    pub neutral_200: Color32,
    pub neutral_300: Color32,
    pub neutral_400: Color32,
    pub neutral_500: Color32,
    pub neutral_600: Color32,
    pub neutral_700: Color32,
    pub neutral_800: Color32,
    pub neutral_900: Color32,
    pub neutral_outlined_active_bg: Color32,
    pub neutral_outlined_border: Color32,
    pub neutral_outlined_color: Color32,
    pub neutral_outlined_disabled_border: Color32,
    pub neutral_outlined_disabled_color: Color32,
    pub neutral_outlined_hover_bg: Color32,
    pub neutral_outlined_hover_border: Color32,
    pub neutral_outlined_hover_color: Color32,
    pub neutral_plain_active_bg: Color32,
    pub neutral_plain_color: Color32,
    pub neutral_plain_disabled_color: Color32,
    pub neutral_plain_hover_bg: Color32,
    pub neutral_plain_hover_color: Color32,
    pub neutral_soft_active_bg: Color32,
    pub neutral_soft_bg: Color32,
    pub neutral_soft_color: Color32,
    pub neutral_soft_disabled_bg: Color32,
    pub neutral_soft_disabled_color: Color32,
    pub neutral_soft_hover_bg: Color32,
    pub neutral_soft_hover_color: Color32,
    pub neutral_solid_active_bg: Color32,
    pub neutral_solid_bg: Color32,
    pub neutral_solid_color: Color32,
    pub neutral_solid_disabled_bg: Color32,
    pub neutral_solid_disabled_color: Color32,
    pub neutral_solid_hover_bg: Color32,
    pub primary_50: Color32,
    pub primary_100: Color32,
    pub primary_200: Color32,
    pub primary_300: Color32,
    pub primary_400: Color32,
    pub primary_500: Color32,
    pub primary_600: Color32,
    pub primary_700: Color32,
    pub primary_800: Color32,
    pub primary_900: Color32,
    pub primary_outlined_active_bg: Color32,
    pub primary_outlined_border: Color32,
    pub primary_outlined_color: Color32,
    pub primary_outlined_disabled_border: Color32,
    pub primary_outlined_disabled_color: Color32,
    pub primary_outlined_hover_bg: Color32,
    pub primary_outlined_hover_border: Color32,
    pub primary_plain_active_bg: Color32,
    pub primary_plain_color: Color32,
    pub primary_plain_disabled_color: Color32,
    pub primary_plain_hover_bg: Color32,
    pub primary_soft_active_bg: Color32,
    pub primary_soft_bg: Color32,
    pub primary_soft_color: Color32,
    pub primary_soft_disabled_bg: Color32,
    pub primary_soft_disabled_color: Color32,
    pub primary_soft_hover_bg: Color32,
    pub primary_solid_active_bg: Color32,
    pub primary_solid_bg: Color32,
    pub primary_solid_color: Color32,
    pub primary_solid_disabled_bg: Color32,
    pub primary_solid_disabled_color: Color32,
    pub primary_solid_hover_bg: Color32,
    pub success_50: Color32,
    pub success_100: Color32,
    pub success_200: Color32,
    pub success_300: Color32,
    pub success_400: Color32,
    pub success_500: Color32,
    pub success_600: Color32,
    pub success_700: Color32,
    pub success_800: Color32,
    pub success_900: Color32,
    pub success_outlined_active_bg: Color32,
    pub success_outlined_border: Color32,
    pub success_outlined_color: Color32,
    pub success_outlined_disabled_border: Color32,
    pub success_outlined_disabled_color: Color32,
    pub success_outlined_hover_bg: Color32,
    pub success_outlined_hover_border: Color32,
    pub success_plain_active_bg: Color32,
    pub success_plain_color: Color32,
    pub success_plain_disabled_color: Color32,
    pub success_plain_hover_bg: Color32,
    pub success_soft_active_bg: Color32,
    pub success_soft_bg: Color32,
    pub success_soft_color: Color32,
    pub success_soft_disabled_bg: Color32,
    pub success_soft_disabled_color: Color32,
    pub success_soft_hover_bg: Color32,
    pub success_solid_active_bg: Color32,
    pub success_solid_bg: Color32,
    pub success_solid_color: Color32,
    pub success_solid_disabled_bg: Color32,
    pub success_solid_disabled_color: Color32,
    pub success_solid_hover_bg: Color32,
    pub text_primary: Color32,
    pub text_secondary: Color32,
    pub text_tertiary: Color32,
    pub warning_50: Color32,
    pub warning_100: Color32,
    pub warning_200: Color32,
    pub warning_300: Color32,
    pub warning_400: Color32,
    pub warning_500: Color32,
    pub warning_600: Color32,
    pub warning_700: Color32,
    pub warning_800: Color32,
    pub warning_900: Color32,
    pub warning_outlined_active_bg: Color32,
    pub warning_outlined_border: Color32,
    pub warning_outlined_color: Color32,
    pub warning_outlined_disabled_border: Color32,
    pub warning_outlined_disabled_color: Color32,
    pub warning_outlined_hover_bg: Color32,
    pub warning_outlined_hover_border: Color32,
    pub warning_plain_active_bg: Color32,
    pub warning_plain_color: Color32,
    pub warning_plain_disabled_color: Color32,
    pub warning_plain_hover_bg: Color32,
    pub warning_soft_active_bg: Color32,
    pub warning_soft_bg: Color32,
    pub warning_soft_color: Color32,
    pub warning_soft_disabled_bg: Color32,
    pub warning_soft_disabled_color: Color32,
    pub warning_soft_hover_bg: Color32,
    pub warning_solid_active_bg: Color32,
    pub warning_solid_bg: Color32,
    pub warning_solid_color: Color32,
    pub warning_solid_disabled_bg: Color32,
    pub warning_solid_disabled_color: Color32,
    pub warning_solid_hover_bg: Color32,
}

impl ThemeColor {
    pub fn select_style(style: ThemeColorStyle) -> &'static ThemeColor {
        match style {
            ThemeColorStyle::Light => &THEME_COLOR_LIGHT,
            ThemeColorStyle::Dark => &THEME_COLOR_DARK,
        }
    }
}

static THEME_COLOR_LIGHT: Lazy<ThemeColor> = Lazy::new(|| ThemeColor {
    style: ThemeColorStyle::Light,
    background_backdrop: Color32::from_rgba_unmultiplied(255, 255, 255, 128),
    background_body: hex_color!("#FFFFFF"),
    background_level1: hex_color!("#F7F7F8"),
    background_level2: hex_color!("#EBEBEF"),
    background_level3: hex_color!("#D8D8DF"),
    background_popup: hex_color!("#FFFFFF"),
    background_surface: hex_color!("#FFFFFF"),
    background_tooltip: hex_color!("#25252D"),
    common_black: hex_color!("#09090D"),
    common_white: hex_color!("#FFFFFF"),
    danger_50: hex_color!("#FFF8F6"),
    danger_100: hex_color!("#FFE9E8"),
    danger_200: hex_color!("#FFC7C5"),
    danger_300: hex_color!("#FF9192"),
    danger_400: hex_color!("#FA5255"),
    danger_500: hex_color!("#D3232F"),
    danger_600: hex_color!("#A10E25"),
    danger_700: hex_color!("#77061B"),
    danger_800: hex_color!("#580013"),
    danger_900: hex_color!("#39000D"),
    danger_outlined_active_bg: hex_color!("#FFC7C5"),
    danger_outlined_border: hex_color!("#FFC7C5"),
    danger_outlined_color: hex_color!("#D3232F"),
    danger_outlined_disabled_border: hex_color!("#FFE9E8"),
    danger_outlined_disabled_color: hex_color!("#FFE9E8"),
    danger_outlined_hover_bg: hex_color!("#FFE9E8"),
    danger_outlined_hover_border: hex_color!("#FF9192"),
    danger_plain_active_bg: hex_color!("#FFC7C5"),
    danger_plain_color: hex_color!("#A10E25"),
    danger_plain_disabled_color: hex_color!("#FFC7C5"),
    danger_plain_hover_bg: hex_color!("#FFE9E8"),
    danger_soft_active_bg: hex_color!("#FF9192"),
    danger_soft_bg: hex_color!("#FFE9E8"),
    danger_soft_color: hex_color!("#A10E25"),
    danger_soft_disabled_bg: hex_color!("#FFF8F6"),
    danger_soft_disabled_color: hex_color!("#FF9192"),
    danger_soft_hover_bg: hex_color!("#FFC7C5"),
    danger_solid_active_bg: hex_color!("#77061B"),
    danger_solid_bg: hex_color!("#D3232F"),
    danger_solid_color: hex_color!("#FFFFFF"),
    danger_solid_disabled_bg: hex_color!("#FFC7C5"),
    danger_solid_disabled_color: hex_color!("#FFFFFF"),
    danger_solid_hover_bg: hex_color!("#A10E25"),
    divider: Color32::from_rgba_unmultiplied(115, 115, 140, 71),
    focus_visible: hex_color!("#096BDE"),
    info_50: hex_color!("#FDF7FF"),
    info_100: hex_color!("#F4EAFF"),
    info_200: hex_color!("#E1CBFF"),
    info_300: hex_color!("#C69EFF"),
    info_400: hex_color!("#A374F9"),
    info_500: hex_color!("#814DDE"),
    info_600: hex_color!("#5F35AE"),
    info_700: hex_color!("#452382"),
    info_800: hex_color!("#301761"),
    info_900: hex_color!("#1D0A42"),
    info_outlined_active_bg: hex_color!("#E1CBFF"),
    info_outlined_border: hex_color!("#E1CBFF"),
    info_outlined_color: hex_color!("#814DDE"),
    info_outlined_disabled_border: hex_color!("#F4EAFF"),
    info_outlined_disabled_color: hex_color!("#F4EAFF"),
    info_outlined_hover_bg: hex_color!("#F4EAFF"),
    info_outlined_hover_border: hex_color!("#C69EFF"),
    info_plain_active_bg: hex_color!("#E1CBFF"),
    info_plain_color: hex_color!("#5F35AE"),
    info_plain_disabled_color: hex_color!("#E1CBFF"),
    info_plain_hover_bg: hex_color!("#F4EAFF"),
    info_soft_active_bg: hex_color!("#C69EFF"),
    info_soft_bg: hex_color!("#F4EAFF"),
    info_soft_color: hex_color!("#5F35AE"),
    info_soft_disabled_bg: hex_color!("#FDF7FF"),
    info_soft_disabled_color: hex_color!("#C69EFF"),
    info_soft_hover_bg: hex_color!("#E1CBFF"),
    info_solid_active_bg: hex_color!("#452382"),
    info_solid_bg: hex_color!("#814DDE"),
    info_solid_color: hex_color!("#FFFFFF"),
    info_solid_disabled_bg: hex_color!("#E1CBFF"),
    info_solid_disabled_color: hex_color!("#FFFFFF"),
    info_solid_hover_bg: hex_color!("#5F35AE"),
    neutral_50: hex_color!("#F7F7F8"),
    neutral_100: hex_color!("#EBEBEF"),
    neutral_200: hex_color!("#D8D8DF"),
    neutral_300: hex_color!("#B9B9C6"),
    neutral_400: hex_color!("#8F8FA3"),
    neutral_500: hex_color!("#73738C"),
    neutral_600: hex_color!("#5A5A72"),
    neutral_700: hex_color!("#434356"),
    neutral_800: hex_color!("#25252D"),
    neutral_900: hex_color!("#131318"),
    neutral_outlined_active_bg: hex_color!("#D8D8DF"),
    neutral_outlined_border: hex_color!("#D8D8DF"),
    neutral_outlined_color: hex_color!("#25252D"),
    neutral_outlined_disabled_border: hex_color!("#EBEBEF"),
    neutral_outlined_disabled_color: hex_color!("#B9B9C6"),
    neutral_outlined_hover_bg: hex_color!("#EBEBEF"),
    neutral_outlined_hover_border: hex_color!("#B9B9C6"),
    neutral_outlined_hover_color: hex_color!("#131318"),
    neutral_plain_active_bg: hex_color!("#D8D8DF"),
    neutral_plain_color: hex_color!("#25252D"),
    neutral_plain_disabled_color: hex_color!("#B9B9C6"),
    neutral_plain_hover_bg: hex_color!("#EBEBEF"),
    neutral_plain_hover_color: hex_color!("#131318"),
    neutral_soft_active_bg: hex_color!("#B9B9C6"),
    neutral_soft_bg: hex_color!("#EBEBEF"),
    neutral_soft_color: hex_color!("#25252D"),
    neutral_soft_disabled_bg: hex_color!("#F7F7F8"),
    neutral_soft_disabled_color: hex_color!("#B9B9C6"),
    neutral_soft_hover_bg: hex_color!("#D8D8DF"),
    neutral_soft_hover_color: hex_color!("#131318"),
    neutral_solid_active_bg: hex_color!("#25252D"),
    neutral_solid_bg: hex_color!("#5A5A72"),
    neutral_solid_color: hex_color!("#FFFFFF"),
    neutral_solid_disabled_bg: hex_color!("#F7F7F8"),
    neutral_solid_disabled_color: hex_color!("#B9B9C6"),
    neutral_solid_hover_bg: hex_color!("#434356"),
    primary_50: hex_color!("#F4FAFF"),
    primary_100: hex_color!("#DDF1FF"),
    primary_200: hex_color!("#ADDBFF"),
    primary_300: hex_color!("#6FB6FF"),
    primary_400: hex_color!("#3990FF"),
    primary_500: hex_color!("#096BDE"),
    primary_600: hex_color!("#054DA7"),
    primary_700: hex_color!("#02367D"),
    primary_800: hex_color!("#072859"),
    primary_900: hex_color!("#00153C"),
    primary_outlined_active_bg: hex_color!("#ADDBFF"),
    primary_outlined_border: hex_color!("#ADDBFF"),
    primary_outlined_color: hex_color!("#096BDE"),
    primary_outlined_disabled_border: hex_color!("#DDF1FF"),
    primary_outlined_disabled_color: hex_color!("#DDF1FF"),
    primary_outlined_hover_bg: hex_color!("#DDF1FF"),
    primary_outlined_hover_border: hex_color!("#6FB6FF"),
    primary_plain_active_bg: hex_color!("#ADDBFF"),
    primary_plain_color: hex_color!("#054DA7"),
    primary_plain_disabled_color: hex_color!("#ADDBFF"),
    primary_plain_hover_bg: hex_color!("#DDF1FF"),
    primary_soft_active_bg: hex_color!("#6FB6FF"),
    primary_soft_bg: hex_color!("#DDF1FF"),
    primary_soft_color: hex_color!("#054DA7"),
    primary_soft_disabled_bg: hex_color!("#F4FAFF"),
    primary_soft_disabled_color: hex_color!("#6FB6FF"),
    primary_soft_hover_bg: hex_color!("#ADDBFF"),
    primary_solid_active_bg: hex_color!("#02367D"),
    primary_solid_bg: hex_color!("#096BDE"),
    primary_solid_color: hex_color!("#FFFFFF"),
    primary_solid_disabled_bg: hex_color!("#ADDBFF"),
    primary_solid_disabled_color: hex_color!("#FFFFFF"),
    primary_solid_hover_bg: hex_color!("#054DA7"),
    success_50: hex_color!("#F3FEF5"),
    success_100: hex_color!("#D7F5DD"),
    success_200: hex_color!("#77EC95"),
    success_300: hex_color!("#4CC76E"),
    success_400: hex_color!("#2CA24D"),
    success_500: hex_color!("#1A7D36"),
    success_600: hex_color!("#0F5D26"),
    success_700: hex_color!("#034318"),
    success_800: hex_color!("#002F0F"),
    success_900: hex_color!("#001D09"),
    success_outlined_active_bg: hex_color!("#77EC95"),
    success_outlined_border: hex_color!("#77EC95"),
    success_outlined_color: hex_color!("#1A7D36"),
    success_outlined_disabled_border: hex_color!("#D7F5DD"),
    success_outlined_disabled_color: hex_color!("#D7F5DD"),
    success_outlined_hover_bg: hex_color!("#D7F5DD"),
    success_outlined_hover_border: hex_color!("#4CC76E"),
    success_plain_active_bg: hex_color!("#77EC95"),
    success_plain_color: hex_color!("#0F5D26"),
    success_plain_disabled_color: hex_color!("#77EC95"),
    success_plain_hover_bg: hex_color!("#D7F5DD"),
    success_soft_active_bg: hex_color!("#4CC76E"),
    success_soft_bg: hex_color!("#D7F5DD"),
    success_soft_color: hex_color!("#0F5D26"),
    success_soft_disabled_bg: hex_color!("#F3FEF5"),
    success_soft_disabled_color: hex_color!("#4CC76E"),
    success_soft_hover_bg: hex_color!("#77EC95"),
    success_solid_active_bg: hex_color!("#034318"),
    success_solid_bg: hex_color!("#1A7D36"),
    success_solid_color: hex_color!("#FFFFFF"),
    success_solid_disabled_bg: hex_color!("#77EC95"),
    success_solid_disabled_color: hex_color!("#FFFFFF"),
    success_solid_hover_bg: hex_color!("#0F5D26"),
    text_primary: hex_color!("#25252D"),
    text_secondary: hex_color!("#5A5A72"),
    text_tertiary: hex_color!("#73738C"),
    warning_50: hex_color!("#FFF8C5"),
    warning_100: hex_color!("#FAE17D"),
    warning_200: hex_color!("#EAC54F"),
    warning_300: hex_color!("#D4A72C"),
    warning_400: hex_color!("#BF8700"),
    warning_500: hex_color!("#9A6700"),
    warning_600: hex_color!("#7D4E00"),
    warning_700: hex_color!("#633C01"),
    warning_800: hex_color!("#4D2D00"),
    warning_900: hex_color!("#3B2300"),
    warning_outlined_active_bg: hex_color!("#EAC54F"),
    warning_outlined_border: hex_color!("#EAC54F"),
    warning_outlined_color: hex_color!("#4D2D00"),
    warning_outlined_disabled_border: hex_color!("#FAE17D"),
    warning_outlined_disabled_color: hex_color!("#FAE17D"),
    warning_outlined_hover_bg: hex_color!("#FFF8C5"),
    warning_outlined_hover_border: hex_color!("#D4A72C"),
    warning_plain_active_bg: hex_color!("#EAC54F"),
    warning_plain_color: hex_color!("#4D2D00"),
    warning_plain_disabled_color: hex_color!("#EAC54F"),
    warning_plain_hover_bg: hex_color!("#FFF8C5"),
    warning_soft_active_bg: hex_color!("#EAC54F"),
    warning_soft_bg: hex_color!("#FFF8C5"),
    warning_soft_color: hex_color!("#4D2D00"),
    warning_soft_disabled_bg: hex_color!("#FFF8C5"),
    warning_soft_disabled_color: hex_color!("#EAC54F"),
    warning_soft_hover_bg: hex_color!("#FAE17D"),
    warning_solid_active_bg: hex_color!("#BF8700"),
    warning_solid_bg: hex_color!("#EAC54F"),
    warning_solid_color: hex_color!("#4D2D00"),
    warning_solid_disabled_bg: hex_color!("#FFF8C5"),
    warning_solid_disabled_color: hex_color!("#EAC54F"),
    warning_solid_hover_bg: hex_color!("#D4A72C"),
});

static THEME_COLOR_DARK: Lazy<ThemeColor> = Lazy::new(|| ThemeColor {
    style: ThemeColorStyle::Dark,
    background_backdrop: Color32::from_rgba_unmultiplied(37, 37, 45, 128),
    background_body: hex_color!("#131318"),
    background_level1: hex_color!("#25252D"),
    background_level2: hex_color!("#434356"),
    background_level3: hex_color!("#5A5A72"),
    background_popup: hex_color!("#25252D"),
    background_surface: hex_color!("#09090D"),
    background_tooltip: hex_color!("#5A5A72"),
    common_black: hex_color!("#09090D"),
    common_white: hex_color!("#FFFFFF"),
    danger_50: hex_color!("#FFF8F6"),
    danger_100: hex_color!("#FFE9E8"),
    danger_200: hex_color!("#FFC7C5"),
    danger_300: hex_color!("#FF9192"),
    danger_400: hex_color!("#FA5255"),
    danger_500: hex_color!("#D3232F"),
    danger_600: hex_color!("#A10E25"),
    danger_700: hex_color!("#77061B"),
    danger_800: hex_color!("#580013"),
    danger_900: hex_color!("#39000D"),
    danger_outlined_active_bg: hex_color!("#39000D"),
    danger_outlined_border: hex_color!("#77061B"),
    danger_outlined_color: hex_color!("#FFC7C5"),
    danger_outlined_disabled_border: hex_color!("#580013"),
    danger_outlined_disabled_color: hex_color!("#580013"),
    danger_outlined_hover_bg: hex_color!("#580013"),
    danger_outlined_hover_border: hex_color!("#A10E25"),
    danger_plain_active_bg: hex_color!("#77061B"),
    danger_plain_color: hex_color!("#FF9192"),
    danger_plain_disabled_color: hex_color!("#580013"),
    danger_plain_hover_bg: hex_color!("#580013"),
    danger_soft_active_bg: hex_color!("#77061B"),
    danger_soft_bg: hex_color!("#39000D"),
    danger_soft_color: hex_color!("#FFC7C5"),
    danger_soft_disabled_bg: hex_color!("#39000D"),
    danger_soft_disabled_color: hex_color!("#580013"),
    danger_soft_hover_bg: hex_color!("#580013"),
    danger_solid_active_bg: hex_color!("#580013"),
    danger_solid_bg: hex_color!("#A10E25"),
    danger_solid_color: hex_color!("#FFFFFF"),
    danger_solid_disabled_bg: hex_color!("#39000D"),
    danger_solid_disabled_color: hex_color!("#77061B"),
    danger_solid_hover_bg: hex_color!("#77061B"),
    divider: Color32::from_rgba_unmultiplied(115, 115, 140, 61),
    focus_visible: hex_color!("#096BDE"),
    info_50: hex_color!("#FDF7FF"),
    info_100: hex_color!("#F4EAFF"),
    info_200: hex_color!("#E1CBFF"),
    info_300: hex_color!("#C69EFF"),
    info_400: hex_color!("#A374F9"),
    info_500: hex_color!("#814DDE"),
    info_600: hex_color!("#5F35AE"),
    info_700: hex_color!("#452382"),
    info_800: hex_color!("#301761"),
    info_900: hex_color!("#1D0A42"),
    info_outlined_active_bg: hex_color!("#1D0A42"),
    info_outlined_border: hex_color!("#452382"),
    info_outlined_color: hex_color!("#E1CBFF"),
    info_outlined_disabled_border: hex_color!("#301761"),
    info_outlined_disabled_color: hex_color!("#301761"),
    info_outlined_hover_bg: hex_color!("#301761"),
    info_outlined_hover_border: hex_color!("#5F35AE"),
    info_plain_active_bg: hex_color!("#452382"),
    info_plain_color: hex_color!("#C69EFF"),
    info_plain_disabled_color: hex_color!("#301761"),
    info_plain_hover_bg: hex_color!("#301761"),
    info_soft_active_bg: hex_color!("#452382"),
    info_soft_bg: hex_color!("#1D0A42"),
    info_soft_color: hex_color!("#E1CBFF"),
    info_soft_disabled_bg: hex_color!("#1D0A42"),
    info_soft_disabled_color: hex_color!("#301761"),
    info_soft_hover_bg: hex_color!("#301761"),
    info_solid_active_bg: hex_color!("#301761"),
    info_solid_bg: hex_color!("#5F35AE"),
    info_solid_color: hex_color!("#FFFFFF"),
    info_solid_disabled_bg: hex_color!("#1D0A42"),
    info_solid_disabled_color: hex_color!("#452382"),
    info_solid_hover_bg: hex_color!("#452382"),
    neutral_50: hex_color!("#F7F7F8"),
    neutral_100: hex_color!("#EBEBEF"),
    neutral_200: hex_color!("#D8D8DF"),
    neutral_300: hex_color!("#B9B9C6"),
    neutral_400: hex_color!("#8F8FA3"),
    neutral_500: hex_color!("#73738C"),
    neutral_600: hex_color!("#5A5A72"),
    neutral_700: hex_color!("#434356"),
    neutral_800: hex_color!("#25252D"),
    neutral_900: hex_color!("#131318"),
    neutral_outlined_active_bg: hex_color!("#25252D"),
    neutral_outlined_border: hex_color!("#25252D"),
    neutral_outlined_color: hex_color!("#D8D8DF"),
    neutral_outlined_disabled_border: hex_color!("#25252D"),
    neutral_outlined_disabled_color: hex_color!("#25252D"),
    neutral_outlined_hover_bg: hex_color!("#25252D"),
    neutral_outlined_hover_border: hex_color!("#434356"),
    neutral_outlined_hover_color: hex_color!("#F7F7F8"),
    neutral_plain_active_bg: hex_color!("#434356"),
    neutral_plain_color: hex_color!("#D8D8DF"),
    neutral_plain_disabled_color: hex_color!("#434356"),
    neutral_plain_hover_bg: hex_color!("#25252D"),
    neutral_plain_hover_color: hex_color!("#F7F7F8"),
    neutral_soft_active_bg: hex_color!("#5A5A72"),
    neutral_soft_bg: hex_color!("#25252D"),
    neutral_soft_color: hex_color!("#D8D8DF"),
    neutral_soft_disabled_bg: hex_color!("#131318"),
    neutral_soft_disabled_color: hex_color!("#434356"),
    neutral_soft_hover_bg: hex_color!("#434356"),
    neutral_soft_hover_color: hex_color!("#F7F7F8"),
    neutral_solid_active_bg: hex_color!("#25252D"),
    neutral_solid_bg: hex_color!("#5A5A72"),
    neutral_solid_color: hex_color!("#FFFFFF"),
    neutral_solid_disabled_bg: hex_color!("#131318"),
    neutral_solid_disabled_color: hex_color!("#434356"),
    neutral_solid_hover_bg: hex_color!("#434356"),
    primary_50: hex_color!("#F4FAFF"),
    primary_100: hex_color!("#DDF1FF"),
    primary_200: hex_color!("#ADDBFF"),
    primary_300: hex_color!("#6FB6FF"),
    primary_400: hex_color!("#3990FF"),
    primary_500: hex_color!("#096BDE"),
    primary_600: hex_color!("#054DA7"),
    primary_700: hex_color!("#02367D"),
    primary_800: hex_color!("#072859"),
    primary_900: hex_color!("#00153C"),
    primary_outlined_active_bg: hex_color!("#00153C"),
    primary_outlined_border: hex_color!("#02367D"),
    primary_outlined_color: hex_color!("#ADDBFF"),
    primary_outlined_disabled_border: hex_color!("#072859"),
    primary_outlined_disabled_color: hex_color!("#072859"),
    primary_outlined_hover_bg: hex_color!("#072859"),
    primary_outlined_hover_border: hex_color!("#054DA7"),
    primary_plain_active_bg: hex_color!("#02367D"),
    primary_plain_color: hex_color!("#6FB6FF"),
    primary_plain_disabled_color: hex_color!("#072859"),
    primary_plain_hover_bg: hex_color!("#072859"),
    primary_soft_active_bg: hex_color!("#02367D"),
    primary_soft_bg: hex_color!("#00153C"),
    primary_soft_color: hex_color!("#ADDBFF"),
    primary_soft_disabled_bg: hex_color!("#00153C"),
    primary_soft_disabled_color: hex_color!("#072859"),
    primary_soft_hover_bg: hex_color!("#072859"),
    primary_solid_active_bg: hex_color!("#072859"),
    primary_solid_bg: hex_color!("#054DA7"),
    primary_solid_color: hex_color!("#FFFFFF"),
    primary_solid_disabled_bg: hex_color!("#00153C"),
    primary_solid_disabled_color: hex_color!("#02367D"),
    primary_solid_hover_bg: hex_color!("#02367D"),
    success_50: hex_color!("#F3FEF5"),
    success_100: hex_color!("#D7F5DD"),
    success_200: hex_color!("#77EC95"),
    success_300: hex_color!("#4CC76E"),
    success_400: hex_color!("#2CA24D"),
    success_500: hex_color!("#1A7D36"),
    success_600: hex_color!("#0F5D26"),
    success_700: hex_color!("#034318"),
    success_800: hex_color!("#002F0F"),
    success_900: hex_color!("#001D09"),
    success_outlined_active_bg: hex_color!("#001D09"),
    success_outlined_border: hex_color!("#034318"),
    success_outlined_color: hex_color!("#77EC95"),
    success_outlined_disabled_border: hex_color!("#002F0F"),
    success_outlined_disabled_color: hex_color!("#002F0F"),
    success_outlined_hover_bg: hex_color!("#002F0F"),
    success_outlined_hover_border: hex_color!("#0F5D26"),
    success_plain_active_bg: hex_color!("#034318"),
    success_plain_color: hex_color!("#4CC76E"),
    success_plain_disabled_color: hex_color!("#002F0F"),
    success_plain_hover_bg: hex_color!("#002F0F"),
    success_soft_active_bg: hex_color!("#034318"),
    success_soft_bg: hex_color!("#001D09"),
    success_soft_color: hex_color!("#77EC95"),
    success_soft_disabled_bg: hex_color!("#001D09"),
    success_soft_disabled_color: hex_color!("#002F0F"),
    success_soft_hover_bg: hex_color!("#002F0F"),
    success_solid_active_bg: hex_color!("#002F0F"),
    success_solid_bg: hex_color!("#0F5D26"),
    success_solid_color: hex_color!("#FFFFFF"),
    success_solid_disabled_bg: hex_color!("#001D09"),
    success_solid_disabled_color: hex_color!("#034318"),
    success_solid_hover_bg: hex_color!("#034318"),
    text_primary: hex_color!("#EBEBEF"),
    text_secondary: hex_color!("#B9B9C6"),
    text_tertiary: hex_color!("#8F8FA3"),
    warning_50: hex_color!("#FFF8C5"),
    warning_100: hex_color!("#FAE17D"),
    warning_200: hex_color!("#EAC54F"),
    warning_300: hex_color!("#D4A72C"),
    warning_400: hex_color!("#BF8700"),
    warning_500: hex_color!("#9A6700"),
    warning_600: hex_color!("#7D4E00"),
    warning_700: hex_color!("#633C01"),
    warning_800: hex_color!("#4D2D00"),
    warning_900: hex_color!("#3B2300"),
    warning_outlined_active_bg: hex_color!("#3B2300"),
    warning_outlined_border: hex_color!("#633C01"),
    warning_outlined_color: hex_color!("#EAC54F"),
    warning_outlined_disabled_border: hex_color!("#4D2D00"),
    warning_outlined_disabled_color: hex_color!("#4D2D00"),
    warning_outlined_hover_bg: hex_color!("#4D2D00"),
    warning_outlined_hover_border: hex_color!("#7D4E00"),
    warning_plain_active_bg: hex_color!("#633C01"),
    warning_plain_color: hex_color!("#D4A72C"),
    warning_plain_disabled_color: hex_color!("#4D2D00"),
    warning_plain_hover_bg: hex_color!("#4D2D00"),
    warning_soft_active_bg: hex_color!("#633C01"),
    warning_soft_bg: hex_color!("#3B2300"),
    warning_soft_color: hex_color!("#EAC54F"),
    warning_soft_disabled_bg: hex_color!("#3B2300"),
    warning_soft_disabled_color: hex_color!("#4D2D00"),
    warning_soft_hover_bg: hex_color!("#4D2D00"),
    warning_solid_active_bg: hex_color!("#9A6700"),
    warning_solid_bg: hex_color!("#D4A72C"),
    warning_solid_color: hex_color!("#09090D"),
    warning_solid_disabled_bg: hex_color!("#3B2300"),
    warning_solid_disabled_color: hex_color!("#633C01"),
    warning_solid_hover_bg: hex_color!("#BF8700"),
});
