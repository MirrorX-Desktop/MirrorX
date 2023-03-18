use eframe::egui::FontTweak;

mod component;
mod viewport;
mod widget;

pub struct App {
    viewport: viewport::Viewport,
}

impl App {
    pub fn new(cc: &eframe::CreationContext) -> Self {
        // cc.egui_ctx.set_debug_on_hover(true);

        prepare_fonts(cc);

        #[cfg(target_os = "windows")]
        set_window_shadow();

        Self {
            viewport: viewport::Viewport::new(),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        self.viewport.draw(ctx, frame);
    }
}

fn prepare_fonts(cc: &eframe::CreationContext) {
    let mut fonts = eframe::egui::FontDefinitions::empty();
    let mut proportional_font_names = Vec::new();
    let mut monospace_font_names = Vec::new();

    // Noto Sans
    proportional_font_names.push(String::from("NotoSans"));
    fonts.font_data.insert(
        String::from("NotoSans"),
        eframe::egui::FontData::from_static(crate::asset::FONT_NOTO_SANS).tweak(FontTweak {
            scale: 1.0,
            y_offset_factor: -0.24,
            y_offset: 0.0,
        }),
    );

    // Noto Sans Mono
    monospace_font_names.push(String::from("NotoSansMono"));
    fonts.font_data.insert(
        String::from("NotoSansMono"),
        eframe::egui::FontData::from_static(crate::asset::FONT_NOTO_SANS_MONO),
    );

    // Noto Sans SC
    proportional_font_names.push(String::from("NotoSansSC"));
    fonts.font_data.insert(
        String::from("NotoSansSC"),
        eframe::egui::FontData::from_static(crate::asset::FONT_NOTO_SANS_SC).tweak(FontTweak {
            scale: 1.0,
            y_offset_factor: -0.24,
            y_offset: 0.0,
        }),
    );

    // Noto Sans TC
    proportional_font_names.push(String::from("NotoSansTC"));
    fonts.font_data.insert(
        String::from("NotoSansTC"),
        eframe::egui::FontData::from_static(crate::asset::FONT_NOTO_SANS_TC),
    );

    // Noto Sans JP
    proportional_font_names.push(String::from("NotoSansJP"));
    fonts.font_data.insert(
        String::from("NotoSansJP"),
        eframe::egui::FontData::from_static(crate::asset::FONT_NOTO_SANS_JP),
    );

    // Noto Sans KR
    proportional_font_names.push(String::from("NotoSansKR"));
    fonts.font_data.insert(
        String::from("NotoSansKR"),
        eframe::egui::FontData::from_static(crate::asset::FONT_NOTO_SANS_KR),
    );

    // Material Icons
    proportional_font_names.push(String::from("MaterialSymbols"));
    fonts.font_data.insert(
        String::from("MaterialSymbols"),
        eframe::egui::FontData::from_static(crate::asset::FONT_MATERIAL_SYMBOLS).tweak(FontTweak {
            scale: 1.0,
            y_offset_factor: 0.0,
            y_offset: 0.0,
        }),
    );

    fonts.families.insert(
        eframe::epaint::FontFamily::Proportional,
        proportional_font_names,
    );

    fonts
        .families
        .insert(eframe::epaint::FontFamily::Monospace, monospace_font_names);

    cc.egui_ctx.set_fonts(fonts);
}

#[cfg(target_os = "windows")]
fn set_window_shadow() {
    use windows::Win32::Foundation::BOOL;
    use windows::Win32::Foundation::HWND;
    use windows::Win32::Foundation::LPARAM;
    use windows::Win32::Graphics::Dwm::DwmExtendFrameIntoClientArea;
    use windows::Win32::System::Threading::GetCurrentThreadId;
    use windows::Win32::UI::Controls::MARGINS;
    use windows::Win32::UI::WindowsAndMessaging::EnumThreadWindows;

    unsafe {
        unsafe extern "system" fn callback(hwnd: HWND, _: LPARAM) -> BOOL {
            DwmExtendFrameIntoClientArea(
                hwnd,
                &MARGINS {
                    cxLeftWidth: -1,
                    cxRightWidth: -1,
                    cyTopHeight: -1,
                    cyBottomHeight: -1,
                },
            )
            .unwrap();
            true.into()
        }

        EnumThreadWindows(GetCurrentThreadId(), Some(callback), None);
    }
}
