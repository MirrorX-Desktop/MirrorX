use super::macos::{
    av_capture_screen_input::AVCaptureScreenInput,
    av_capture_session::{AVCaptureSession, AVCaptureSessionPreset},
    av_capture_video_data_output::AVCaptureVideoDataOutput,
};
use crate::media::{bindings::macos::*, frame::CaptureFrame};
use anyhow::bail;
use crossbeam::channel::Receiver;
use tracing::info;

pub struct DesktopDuplicator {
    capture_session: AVCaptureSession,
}

unsafe impl Send for DesktopDuplicator {}

impl DesktopDuplicator {
    pub fn new(fps: i32) -> anyhow::Result<(Self, Receiver<CaptureFrame>)> {
        let mut capture_session = AVCaptureSession::new();
        capture_session.begin_configuration();
        capture_session.set_session_preset(AVCaptureSessionPreset::AVCaptureSessionPresetHigh);

        let capture_screen_input = AVCaptureScreenInput::new(0);
        capture_screen_input.set_captures_cursor(true);
        capture_screen_input.set_captures_mouse_clicks(true);
        capture_screen_input.set_min_frame_duration(unsafe { CMTimeMake(1, fps) });

        if capture_session.can_add_input(&capture_screen_input) {
            capture_session.add_input(capture_screen_input);
        } else {
            bail!("can't add input");
        }

        let (tx, rx) = crossbeam::channel::bounded(1);

        let capture_video_data_output = AVCaptureVideoDataOutput::new(tx);

        if capture_session.can_add_output(&capture_video_data_output) {
            capture_session.add_output(capture_video_data_output);
        } else {
            bail!("can't add output");
        }

        capture_session.commit_configuration();

        Ok((DesktopDuplicator { capture_session }, rx))
    }

    pub fn start(&mut self) -> anyhow::Result<()> {
        self.capture_session.start_running();
        Ok(())
    }

    pub fn stop(&mut self) {
        self.capture_session.stop_running();
    }
}

impl Drop for DesktopDuplicator {
    fn drop(&mut self) {
        self.capture_session.stop_running();
        info!("DesktopDuplicator dropped");
    }
}
