#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[tokio::main]
async fn main() -> eframe::Result<()> {
    rust_i18n::set_locale("zh");

    let app = mirrorx::frame::create_app().unwrap();

    let options = eframe::NativeOptions {
        maximized: false,
        resizable: false,
        initial_window_size: Some(eframe::epaint::vec2(960.0, 660.0)),
        max_window_size: Some(eframe::epaint::vec2(960.0, 660.0)),
        min_window_size: Some(eframe::epaint::vec2(960.0, 660.0)),
        decorated: false,
        ..Default::default()
    };

    eframe::run_native("MirrorX", options, app)
}
