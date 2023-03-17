use eframe::egui::FontTweak;
use once_cell::sync::Lazy;
use std::collections::HashMap;

static PROPORTIONAL_FONTS: Lazy<HashMap<&str, (&[u8], Option<FontTweak>)>> = Lazy::new(|| {
    [
        (
            "NotoSans",
            (
                include_bytes!("../../assets/fonts/NotoSans-Regular.ttf").as_ref(),
                None,
            ),
        ),
        (
            "NotoSansJP",
            (
                include_bytes!("../../assets/fonts/NotoSansJP-Regular.otf").as_ref(),
                None,
            ),
        ),
        (
            "NotoSansKR",
            (
                include_bytes!("../../assets/fonts/NotoSansKR-Regular.otf").as_ref(),
                None,
            ),
        ),
        (
            "NotoSansSC",
            (
                include_bytes!("../../assets/fonts/NotoSansSC-Regular.otf").as_ref(),
                None,
            ),
        ),
        (
            "NotoSansTC",
            (
                include_bytes!("../../assets/fonts/NotoSansTC-Regular.otf").as_ref(),
                None,
            ),
        ),
        (
            "MaterialIcons",
            (
                include_bytes!("../../assets/fonts/MaterialIconsRound-Regular.otf").as_ref(),
                Some(FontTweak {
                    y_offset_factor: 0.18,
                    y_offset: 0.9,
                    ..Default::default()
                }),
            ),
        ),
    ]
    .into_iter()
    .collect()
});

static MONOSPACE_FONTS: Lazy<HashMap<&str, (&[u8], Option<FontTweak>)>> = Lazy::new(|| {
    [(
        "NotoSansMono",
        (
            include_bytes!("../../assets/fonts/NotoSansMono-Regular.ttf").as_ref(),
            None,
        ),
    )]
    .into_iter()
    .collect()
});

pub fn set_fonts(ctx: &eframe::egui::Context) {
    let mut fonts = eframe::egui::FontDefinitions::default();

    add_normal_font_families(
        &mut fonts,
        &PROPORTIONAL_FONTS,
        eframe::epaint::FontFamily::Proportional,
    );

    add_normal_font_families(
        &mut fonts,
        &MONOSPACE_FONTS,
        eframe::epaint::FontFamily::Monospace,
    );

    ctx.set_fonts(fonts);
}

fn add_normal_font_families<'a>(
    fonts: &mut eframe::egui::FontDefinitions,
    font_data: &'a HashMap<&'a str, (&'a [u8], Option<FontTweak>)>,
    family: eframe::epaint::FontFamily,
) {
    let mut new_fonts = Vec::new();
    for (name, (font_data, tweak)) in font_data.iter() {
        new_fonts.push(name.to_string());

        let mut font = eframe::egui::FontData::from_owned(font_data.to_vec());
        if let Some(tweak) = tweak {
            font = font.tweak(*tweak);
        }

        fonts.font_data.insert(name.to_string(), font);
    }

    let old_fonts = fonts.families.entry(family.clone()).or_default();

    new_fonts.append(old_fonts);

    fonts.families.insert(family, new_fonts);
}
