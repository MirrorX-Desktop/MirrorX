use super::state::SharedState;
use eframe::egui::{Response, Ui};

pub trait StatefulWidget {
    fn update_state(&mut self, shared_state: &SharedState);

    fn update_view(&self, ui: &mut Ui) -> Option<Response>;

    fn update(&mut self, ui: &mut Ui, shared_state: &SharedState) -> Option<Response> {
        self.update_state(shared_state);
        self.update_view(ui)
    }
}
