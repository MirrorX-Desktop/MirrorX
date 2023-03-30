use super::{
    asset,
    state::{start_ui_event_processor, SharedState, UIEvent},
    viewport::Viewport,
};
use crossbeam::atomic::AtomicCell;
use eframe::{
    egui::*,
    epaint::{Shadow, TextShape},
};
use std::{rc::Rc, sync::atomic::AtomicPtr, time::Duration};
use tokio::sync::mpsc::UnboundedReceiver;

pub struct App {
    viewport: Viewport,
    atomic_state: AtomicCell<SharedState>,
}

impl App {
    pub fn new(cc: &eframe::CreationContext) -> Self {
        // cc.egui_ctx.set_debug_on_hover(true);

        prepare_fonts(cc);

        #[cfg(target_os = "windows")]
        set_window_shadow();

        let atomic_state = AtomicCell::new(SharedState::default());

        start_ui_event_processor(atomic_state);

        Self {
            viewport: Viewport::new(),
            atomic_state,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, frame: &mut eframe::Frame) {
        let state_snapshot = self.atomic_state.into_inner();

        // let mut style = (*ctx.style()).clone();
        // style.visuals.window_fill = self.ui_state.theme_color.background_popup;
        // style.visuals.window_stroke =
        //     Stroke::new(1.0, self.ui_state.theme_color.neutral_outlined_border);
        // style.visuals.popup_shadow = Shadow {
        //     extrusion: 1.5,
        //     color: self.ui_state.theme_color.background_level3,
        // };
        // style.visuals.widgets.noninteractive.bg_stroke.color =
        //     self.ui_state.theme_color.neutral_outlined_color;
        // style.visuals.widgets.noninteractive.fg_stroke.color =
        //     self.ui_state.theme_color.text_primary;
        // style.visuals.widgets.inactive.fg_stroke.color = self.ui_state.theme_color.text_primary;

        // ctx.set_style(style);

        self.viewport.draw(ctx, frame, &state_snapshot);
        // draw_notifications(ctx, &state_snapshot);
    }
}

// fn draw_notifications(ctx: &Context, ui_state: &SharedState) {
//     if ui_state.notifications.is_empty() {
//         return;
//     }

//     let id = Id::new("notifications");
//     let rect = ctx.screen_rect().shrink2(vec2(120.0, 60.0));
//     let notifications = ui_state.notifications.poll_notifications();
//     let ui = Ui::new(
//         ctx.clone(),
//         LayerId::new(eframe::egui::Order::Tooltip, id),
//         id,
//         rect,
//         rect.expand(2.0),
//     );

//     let mut start_pos = ui.next_widget_position();
//     for notification in notifications {
//         const PADDING: f32 = 8.0;
//         let galley = ui.painter().layout(
//             notification.content.clone(),
//             FontId::proportional(14.0),
//             Color32::WHITE,
//             ui.available_width() - PADDING * 2.0,
//         );

//         let notification_rect = Rect::from_min_size(
//             start_pos,
//             vec2(ui.available_width(), galley.size().y + PADDING * 2.0),
//         );

//         ui.painter().rect(
//             notification_rect,
//             Rounding::same(6.0),
//             Color32::LIGHT_GREEN,
//             Stroke::new(1.0, Color32::DARK_GREEN),
//         );

//         ui.painter().add(TextShape {
//             pos: start_pos + Vec2::splat(PADDING),
//             galley,
//             underline: Stroke::NONE,
//             override_text_color: None,
//             angle: 0.0,
//         });

//         start_pos += vec2(0.0, notification_rect.height() + PADDING * 1.5);
//     }

//     ctx.request_repaint_after(Duration::from_secs(1));
// }

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
