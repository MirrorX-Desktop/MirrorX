mod desktop;

use self::desktop::DesktopWindow;
use once_cell::sync::Lazy;
use std::{collections::HashMap, sync::Arc};
use tauri_egui::{
    eframe::CreationContext,
    egui::{FontData, FontDefinitions, FontFamily, FontTweak},
};

static PROPORTIONAL_FONTS: Lazy<HashMap<&str, &[u8]>> = Lazy::new(|| {
    [
        (
            "NotoSans",
            include_bytes!("../../assets/fonts/NotoSans-Regular.ttf").as_ref(),
        ),
        (
            "NotoSansJP",
            include_bytes!("../../assets/fonts/NotoSansJP-Regular.otf").as_ref(),
        ),
        (
            "NotoSansKR",
            include_bytes!("../../assets/fonts/NotoSansKR-Regular.otf").as_ref(),
        ),
        (
            "NotoSansSC",
            include_bytes!("../../assets/fonts/NotoSansSC-Regular.otf").as_ref(),
        ),
        (
            "NotoSansTC",
            include_bytes!("../../assets/fonts/NotoSansTC-Regular.otf").as_ref(),
        ),
    ]
    .into_iter()
    .collect()
});

static MONOSPACE_FONTS: Lazy<HashMap<&str, &[u8]>> = Lazy::new(|| {
    [(
        "NotoSansMono",
        include_bytes!("../../assets/fonts/NotoSansMono-Regular.ttf").as_ref(),
    )]
    .into_iter()
    .collect()
});

#[allow(clippy::too_many_arguments)]
pub fn create_desktop_window(
    cc: &CreationContext,
    gl_context: Arc<tauri_egui::eframe::glow::Context>,
    local_device_id: i64,
    remote_device_id: i64,
    opening_key: Vec<u8>,
    opening_nonce: Vec<u8>,
    sealing_key: Vec<u8>,
    sealing_nonce: Vec<u8>,
    visit_credentials: String,
) -> DesktopWindow {
    set_fonts(&cc.egui_ctx);

    cc.egui_ctx.set_debug_on_hover(true);

    crate::window::desktop::DesktopWindow::new(
        local_device_id,
        remote_device_id,
        opening_key,
        opening_nonce,
        sealing_key,
        sealing_nonce,
        visit_credentials,
        gl_context,
    )
}

fn set_fonts(ctx: &tauri_egui::egui::Context) {
    let mut fonts = tauri_egui::egui::FontDefinitions::default();

    add_normal_font_families(&mut fonts, &PROPORTIONAL_FONTS, FontFamily::Proportional);
    add_normal_font_families(&mut fonts, &MONOSPACE_FONTS, FontFamily::Monospace);
    // add_custom_family_font_families(&mut fonts);

    ctx.set_fonts(fonts);
}

fn add_normal_font_families<'a>(
    fonts: &mut FontDefinitions,
    font_data: &'a HashMap<&'a str, &'a [u8]>,
    family: FontFamily,
) {
    let mut new_fonts = Vec::new();
    for (name, font_data) in font_data.iter() {
        new_fonts.push(name.to_string());
        fonts
            .font_data
            .insert(name.to_string(), FontData::from_owned(font_data.to_vec()));
    }

    let old_fonts = fonts.families.entry(family.clone()).or_default();

    new_fonts.append(old_fonts);

    fonts.families.insert(family, new_fonts);
}

// fn add_custom_family_font_families(fonts: &mut FontDefinitions) {
//     fonts.font_data.insert(
//         "LiquidCrystal".into(),
//         FontData::from_static(
//             include_bytes!("../../assets/fonts/LiquidCrystal-Light.otf").as_ref(),
//         )
//         .tweak(FontTweak {
//             scale: 1.0,
//             y_offset_factor: 0.0,
//             y_offset: 0.5,
//         }),
//     );

//     fonts.families.insert(
//         FontFamily::Name("LiquidCrystal".into()),
//         vec!["LiquidCrystal".into()],
//     );
// }
