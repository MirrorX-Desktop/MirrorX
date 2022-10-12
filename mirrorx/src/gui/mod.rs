mod app;
mod connect;
mod history;
mod lan;
mod widgets;

trait View {
    fn build(&mut self, ui: &mut eframe::egui::Ui);
}

pub fn run_app() -> anyhow::Result<()> {
    let config_manager = if let Some(base_dir) = directories_next::BaseDirs::new() {
        let config_db_dir = base_dir.data_dir().join("MirrorX").join("mirrorx.db");
        mirrorx_core::api::config::ConfigManager::new(&config_db_dir)?
    } else {
        anyhow::bail!("read config db dir failed");
    };

    let native_options = eframe::NativeOptions {
        always_on_top: true,
        maximized: false,
        initial_window_size: Some(eframe::epaint::Vec2::new(380f32, 630f32)),
        resizable: false,
        renderer: eframe::Renderer::Wgpu,
        follow_system_theme: false,
        default_theme: eframe::Theme::Light,
        // centered: true,
        // fullsize_content: true,
        ..Default::default()
    };

    eframe::run_native(
        "MirrorX",
        native_options,
        Box::new(|cc| Box::new(app::App::new(cc, config_manager))),
    );

    Ok(())
}
