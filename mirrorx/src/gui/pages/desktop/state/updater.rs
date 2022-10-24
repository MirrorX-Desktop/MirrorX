use super::event::Event;
use crate::send_event;
use tokio::sync::mpsc::{Sender, UnboundedSender};

pub struct StateUpdater {
    tx: Sender<Event>,
}

impl StateUpdater {
    pub fn new(tx: Sender<Event>) -> Self {
        Self { tx }
    }

    // pub fn emit_negotiate_desktop_params(&self) {
    //     send_event!(self.tx, Event::EmitNegotiateDesktopParams)
    // }
}
