use crate::utility::format_device_id;
use mirrorx_core::{
    api::endpoint::{client::EndPointClient, id::EndPointID, message::EndPointDirectoryResponse},
    error::CoreError,
    DesktopDecodeFrame,
};
use std::sync::Arc;
use tokio::sync::mpsc::Receiver;

#[derive(Debug)]
pub enum VisitState {
    Connecting,
    Negotiating,
    Serving,
    ErrorOccurred,
}

pub struct State {
    format_remote_device_id: String,
    visit_state: VisitState,
    endpoint_client: Arc<EndPointClient>,
    desktop_frame_scaled: bool,
    desktop_frame_scalable: bool,
    last_error: Option<CoreError>,
    render_rx: Receiver<DesktopDecodeFrame>,
    current_frame: Option<DesktopDecodeFrame>,
    directory_rx: Receiver<EndPointDirectoryResponse>,
}

impl State {
    pub fn new(
        endpoint_id: EndPointID,
        client: Arc<EndPointClient>,
        render_frame_rx: tokio::sync::mpsc::Receiver<DesktopDecodeFrame>,
        directory_rx: tokio::sync::mpsc::Receiver<EndPointDirectoryResponse>,
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
            visit_state: VisitState::Serving,
            endpoint_client: client,
            desktop_frame_scaled: true,
            desktop_frame_scalable: true,
            last_error: None,
            render_rx: render_frame_rx,
            current_frame: None,
            directory_rx,
        }
    }

    pub fn format_remote_device_id(&self) -> &str {
        self.format_remote_device_id.as_ref()
    }

    pub fn endpoint_client(&self) -> Arc<EndPointClient> {
        self.endpoint_client.clone()
    }

    pub fn visit_state(&self) -> &VisitState {
        &self.visit_state
    }

    pub fn desktop_frame_scaled(&self) -> bool {
        self.desktop_frame_scaled
    }

    pub fn last_error(&self) -> Option<&CoreError> {
        self.last_error.as_ref()
    }

    pub fn current_frame(&mut self) -> Option<DesktopDecodeFrame> {
        while let Ok(frame) = self.render_rx.try_recv() {
            self.current_frame = Some(frame);
        }

        self.current_frame.clone()
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
