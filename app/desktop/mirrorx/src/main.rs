#[tokio::main]
async fn main() -> eframe::Result<()> {
    mirrorx::asset::StaticImageCache::load().unwrap();

    rust_i18n::set_locale("zh");

    let options = eframe::NativeOptions {
        maximized: false,
        resizable: false,
        initial_window_size: Some(eframe::epaint::vec2(960.0, 680.0)),
        max_window_size: Some(eframe::epaint::vec2(960.0, 640.0)),
        min_window_size: Some(eframe::epaint::vec2(960.0, 640.0)),
        decorated: false,
        ..Default::default()
    };

    eframe::run_native(
        "MirrorX",
        options,
        Box::new(|cc| Box::new(mirrorx::app::App::new(cc))),
    )
}
