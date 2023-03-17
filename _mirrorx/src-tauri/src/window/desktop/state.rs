use mirrorx_core::{
    component::screen::display::Display, error::CoreError,
    service::endpoint::message::EndPointNegotiateReply, DesktopDecodeFrame,
};
use std::sync::Arc;
use tokio::sync::mpsc::{channel, Receiver, Sender};

pub enum StateCommand {
    ErrorHappened(CoreError),
    NegotiateFinished(EndPointNegotiateReply),
    UpdateVideoFrame(DesktopDecodeFrame),
    SwitchDisplay(String),
}

pub struct State {
    egui_ctx: Option<tauri_egui::egui::Context>,
    state_command_rx: Receiver<StateCommand>,
    errors: Vec<CoreError>,
    displays: Vec<Display>,
    current_display_id: Option<String>,
    is_switching_current_display_id: bool,
    video_frame: Option<Arc<DesktopDecodeFrame>>,
}

impl State {
    pub fn new() -> (Self, Sender<StateCommand>) {
        let (state_command_tx, state_command_rx) = channel(180);

        (
            Self {
                egui_ctx: None,
                state_command_rx,
                errors: Vec::new(),
                displays: Vec::new(),
                current_display_id: None,
                is_switching_current_display_id: false,
                video_frame: None,
            },
            state_command_tx,
        )
    }

    pub fn process_state_command(&mut self) {
        while let Ok(command) = self.state_command_rx.try_recv() {
            match command {
                StateCommand::ErrorHappened(error) => {
                    self.push_error(error);
                    // recover state
                    self.is_switching_current_display_id = false;
                }
                StateCommand::NegotiateFinished(reply) => {
                    self.displays = reply.displays;
                }
                StateCommand::UpdateVideoFrame(frame) => self.video_frame = Some(Arc::new(frame)),
                StateCommand::SwitchDisplay(display_id) => {
                    self.current_display_id = Some(display_id);
                }
            }
        }
    }

    pub fn set_egui_context(&mut self, ctx: tauri_egui::egui::Context) {
        self.egui_ctx = Some(ctx)
    }

    pub fn get_errors(&self) -> &[CoreError] {
        self.errors.as_ref()
    }

    pub fn push_error(&mut self, error: CoreError) {
        self.errors.push(error)
    }

    pub fn get_displays(&self) -> &[Display] {
        self.displays.as_ref()
    }

    pub fn get_current_display_id(&self) -> Option<String> {
        self.current_display_id.clone()
    }

    pub fn take_video_frame(&mut self) -> Option<Arc<DesktopDecodeFrame>> {
        self.video_frame.take()
    }

    pub fn get_is_switching_current_display_id(&self) -> bool {
        self.is_switching_current_display_id
    }

    pub fn set_is_switching_current_display_id(&mut self, is_switching: bool) {
        self.is_switching_current_display_id = is_switching;
    }
}
