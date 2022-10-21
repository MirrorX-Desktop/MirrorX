mod event;
mod updater;

use event::Event;
use mirrorx_core::error::CoreError;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

pub struct State {
    tx: UnboundedSender<Event>,
    rx: UnboundedReceiver<Event>,

    local_device_id: i64,
    remote_device_id: i64,
}

impl State {
    pub fn new(
        local_device_id: i64,
        remote_device_id: i64,
        opening_key: Vec<u8>,
        opening_nonce: Vec<u8>,
        sealing_key: Vec<u8>,
        sealing_nonce: Vec<u8>,
        visit_credentials: String,
    ) -> Self {
        todo!()
    }
}

impl State {
    pub fn handle_event(&mut self) -> Option<CoreError> {
        while let Ok(event) = self.rx.try_recv() {
            match event {
                Event::ConnectEndPoint {} => todo!(),
            }
        }

        None
    }
}
