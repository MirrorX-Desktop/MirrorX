mod history;
mod home;
mod lan;
mod settings;

pub use history::HistoryPage;
pub use home::HomePage;
pub use lan::LanPage;
pub use settings::SettingsPage;

pub trait Page {
    fn draw(&mut self, ui: &mut eframe::egui::Ui);
}
