mod app;
mod asset;
mod component;
mod page;
mod viewport;
mod widget;

pub fn create_app() -> anyhow::Result<eframe::AppCreator> {
    asset::StaticImageCache::load()?;
    Ok(Box::new(|cc| Box::new(app::App::new(cc))))
}
