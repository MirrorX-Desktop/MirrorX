use super::event::Event;
use crate::send_event;
use tokio::sync::mpsc::UnboundedSender;

pub struct StateUpdater {
    tx: UnboundedSender<Event>,
}

impl StateUpdater {
    pub fn new(tx: UnboundedSender<Event>) -> Self {
        Self { tx }
    }

    // pub fn emit_negotiate_desktop_params(&self) {
    //     send_event!(self.tx, Event::EmitNegotiateDesktopParams)
    // }
}
