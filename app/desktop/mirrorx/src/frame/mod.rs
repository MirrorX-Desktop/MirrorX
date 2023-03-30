mod app;
mod asset;
mod color;
mod component;
mod state;
mod view;
mod viewport;
mod widget;
mod widgets;

pub fn create_app() -> anyhow::Result<eframe::AppCreator> {
    asset::StaticImageCache::load()?;

    Ok(Box::new(|cc| Box::new(app::App::new(cc))))
}
