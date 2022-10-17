mod app;
mod connect;
mod history;
mod lan;
mod widgets;

trait View {
    fn build(&mut self, ui: &mut eframe::egui::Ui);
}

pub fn run_app() -> anyhow::Result<()> {
    let native_options = eframe::NativeOptions {
        always_on_top: true,
        maximized: false,
        initial_window_size: Some(eframe::epaint::Vec2::new(380f32, 630f32)),
        resizable: false,
        follow_system_theme: false,
        default_theme: eframe::Theme::Light,
        // centered: true,
        // fullsize_content: true,
        ..Default::default()
    };

    eframe::run_native(
        "MirrorX",
        native_options,
        Box::new(|cc| Box::new(app::App::new(cc))),
    );

    Ok(())
}
