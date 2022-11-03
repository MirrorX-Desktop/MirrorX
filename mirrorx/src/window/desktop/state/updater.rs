use super::event::Event;
use crate::send_event;
use mirrorx_core::api::endpoint::message::InputEvent;
use tokio::sync::mpsc::{Sender, UnboundedSender};

pub struct StateUpdater {
    tx: Sender<Event>,
}

impl StateUpdater {
    pub fn new(tx: Sender<Event>) -> Self {
        Self { tx }
    }

    pub fn update_use_original_resolution(&self, use_original_resolution: bool) {
        send_event!(
            self.tx,
            Event::UpdateUseOriginalResolution {
                use_original_resolution
            }
        )
    }

    pub fn input(&self, input_series: Vec<InputEvent>) {
        send_event!(self.tx, Event::Input { input_series })
    }
}
