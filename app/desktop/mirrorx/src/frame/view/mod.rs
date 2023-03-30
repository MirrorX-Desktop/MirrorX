mod history;
mod home;
mod lan;
mod settings;

pub use history::HistoryView;
pub use home::HomePage;
pub use lan::LanView;
pub use settings::SettingsView;

#[derive(Debug, PartialEq, Eq)]
pub enum ViewId {
    Device,
    Lan,
    History,
    Settings,
}
