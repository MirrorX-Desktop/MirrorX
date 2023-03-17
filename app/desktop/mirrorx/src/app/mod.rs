mod component;
mod font;
mod viewport;
mod widget;

pub struct App {
    viewport: viewport::Viewport,
}

impl App {
    pub fn new(cc: &eframe::CreationContext) -> Self {
        font::set_fonts(&cc.egui_ctx);

        // cc.egui_ctx.set_debug_on_hover(true);

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
