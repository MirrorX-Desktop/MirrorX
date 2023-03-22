use super::{
    asset,
    state::{update_ui_state, UIEvent, UIState},
    viewport::Viewport,
};
use eframe::egui::{FontData, FontTweak};
use tokio::sync::mpsc::UnboundedReceiver;

pub struct App {
    viewport: Viewport,
    ui_state: UIState,
    ui_event_rx: UnboundedReceiver<UIEvent>,
}

impl App {
    pub fn new(
        cc: &eframe::CreationContext,
        ui_state: UIState,
        ui_event_rx: UnboundedReceiver<UIEvent>,
    ) -> Self {
        // cc.egui_ctx.set_debug_on_hover(true);

        prepare_fonts(cc);

        #[cfg(target_os = "windows")]
        set_window_shadow();

        Self {
            viewport: Viewport::new(),
            ui_state,
            ui_event_rx,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        update_ui_state(&mut self.ui_state, &mut self.ui_event_rx);
        self.viewport.draw(ctx, frame, &mut self.ui_state);
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
        FontData::from_static(asset::FONT_NOTO_SANS).tweak(FontTweak {
            scale: 1.0,
            y_offset_factor: -0.24,
            y_offset: 0.0,
        }),
    );

    // Noto Sans Mono
    monospace_font_names.push(String::from("NotoSansMono"));
    fonts.font_data.insert(
        String::from("NotoSansMono"),
        FontData::from_static(asset::FONT_NOTO_SANS_MONO),
    );

    // Noto Sans SC
    proportional_font_names.push(String::from("NotoSansSC"));
    fonts.font_data.insert(
        String::from("NotoSansSC"),
        FontData::from_static(asset::FONT_NOTO_SANS_SC).tweak(FontTweak {
            scale: 1.0,
            y_offset_factor: -0.24,
            y_offset: 0.0,
        }),
    );

    // Noto Sans TC
    proportional_font_names.push(String::from("NotoSansTC"));
    fonts.font_data.insert(
        String::from("NotoSansTC"),
        FontData::from_static(asset::FONT_NOTO_SANS_TC),
    );

    // Noto Sans JP
    proportional_font_names.push(String::from("NotoSansJP"));
    fonts.font_data.insert(
        String::from("NotoSansJP"),
        FontData::from_static(asset::FONT_NOTO_SANS_JP),
    );

    // Noto Sans KR
    proportional_font_names.push(String::from("NotoSansKR"));
    fonts.font_data.insert(
        String::from("NotoSansKR"),
        FontData::from_static(asset::FONT_NOTO_SANS_KR),
    );

    // Material Symbols
    proportional_font_names.push(String::from("MaterialSymbols"));
    fonts.font_data.insert(
        String::from("MaterialSymbols"),
        FontData::from_static(asset::FONT_MATERIAL_SYMBOLS).tweak(FontTweak {
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
