use crate::utility::format_device_id;
use mirrorx_core::{
    api::endpoint::{client::EndPointClient, id::EndPointID},
    DesktopDecodeFrame,
};
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc::Receiver;

pub struct State {
    format_remote_device_id: String,
    endpoint_client: Arc<EndPointClient>,
    desktop_frame_scaled: bool,
    desktop_frame_scalable: bool,
    render_rx: Receiver<DesktopDecodeFrame>,
    frame_slot: Arc<Mutex<DesktopDecodeFrame>>,
    frame_size: (i32, i32),
}

impl State {
    pub fn new(
        endpoint_id: EndPointID,
        client: Arc<EndPointClient>,
        render_frame_rx: tokio::sync::mpsc::Receiver<DesktopDecodeFrame>,
        frame_slot: Arc<Mutex<DesktopDecodeFrame>>,
    ) -> Self {
        let format_remote_device_id = match endpoint_id {
            EndPointID::DeviceID {
                remote_device_id: remote,
                ..
            } => format_device_id(remote),
            EndPointID::LANID {
                remote_ip: remote, ..
            } => remote.to_string(),
        };

        Self {
            format_remote_device_id,
            endpoint_client: client,
            desktop_frame_scaled: true,
            desktop_frame_scalable: true,
            render_rx: render_frame_rx,
            frame_slot,
            frame_size: (0, 0),
        }
    }

    pub fn format_remote_device_id(&self) -> &str {
        self.format_remote_device_id.as_ref()
    }

    pub fn endpoint_client(&self) -> Arc<EndPointClient> {
        self.endpoint_client.clone()
    }

    pub fn desktop_frame_scaled(&self) -> bool {
        self.desktop_frame_scaled
    }

    pub fn update_desktop_frame(&mut self) -> (i32, i32) {
        let mut new_frame = None;
        while let Ok(frame) = self.render_rx.try_recv() {
            new_frame = Some(frame);
        }

        if let Some(new_frame) = new_frame {
            self.frame_size = (new_frame.width, new_frame.height);
            (*self.frame_slot.lock().unwrap()) = new_frame;
        }

        self.frame_size
    }

    pub fn desktop_frame_scalable(&self) -> bool {
        self.desktop_frame_scalable
    }
}

impl State {
    pub fn set_desktop_frame_scaled(&mut self, scaled: bool) {
        self.desktop_frame_scaled = scaled
    }

    pub fn set_desktop_frame_scalable(&mut self, scalable: bool) {
        self.desktop_frame_scalable = scalable
    }
}
