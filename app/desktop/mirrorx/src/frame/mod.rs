mod app;
mod asset;
mod component;
mod page;
mod state;
mod viewport;
mod widget;

pub fn create_app() -> anyhow::Result<eframe::AppCreator> {
    asset::StaticImageCache::load()?;
    let ui_state = state::UIState::new()?;
    let (ui_event_tx, ui_event_rx) = tokio::sync::mpsc::unbounded_channel();
    Ok(Box::new(|cc| {
        Box::new(app::App::new(cc, ui_state, ui_event_rx))
    }))
}
