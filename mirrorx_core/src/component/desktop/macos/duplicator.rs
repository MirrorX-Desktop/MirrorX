use super::{
    av_capture_screen_input::AVCaptureScreenInput,
    av_capture_session::{AVCaptureSession, AVCaptureSessionPreset},
    av_capture_video_data_output::AVCaptureVideoDataOutput,
};
use crate::{component::desktop::Frame, ffi::os::CMTimeMake};
use anyhow::bail;
use crossbeam::channel::Sender;
use tracing::info;

pub struct Duplicator {
    capture_session: AVCaptureSession,
}

unsafe impl Send for Duplicator {}

impl Duplicator {
    pub fn new(capture_frame_tx: Sender<Frame>, display_id: &str, fps: u8) -> anyhow::Result<Self> {
        let display_id: u32 = match display_id.parse() {
            Ok(v) => v,
            Err(_) => return Err(anyhow::anyhow!("convert display id failed")),
        };

        let mut capture_session = AVCaptureSession::new();
        capture_session.begin_configuration();
        capture_session.set_session_preset(AVCaptureSessionPreset::AVCaptureSessionPresetHigh);

        let capture_screen_input = AVCaptureScreenInput::new(display_id);
        capture_screen_input.set_captures_cursor(true);
        capture_screen_input.set_captures_mouse_clicks(false);
        capture_screen_input.set_min_frame_duration(unsafe { CMTimeMake(1, fps as i32) });

        if capture_session.can_add_input(&capture_screen_input) {
            capture_session.add_input(capture_screen_input);
        } else {
            bail!("can't add input");
        }

        let capture_video_data_output = AVCaptureVideoDataOutput::new(capture_frame_tx);

        if capture_session.can_add_output(&capture_video_data_output) {
            capture_session.add_output(capture_video_data_output);
        } else {
            bail!("can't add output");
        }

        capture_session.commit_configuration();

        Ok(Duplicator { capture_session })
    }

    pub fn start(&mut self) -> anyhow::Result<()> {
        self.capture_session.start_running();
        Ok(())
    }

    pub fn stop(&mut self) {
        self.capture_session.stop_running();
    }
}

impl Drop for Duplicator {
    fn drop(&mut self) {
        self.capture_session.stop_running();
        info!("DesktopDuplicator dropped");
    }
}
