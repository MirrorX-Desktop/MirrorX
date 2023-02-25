mod duplicator;

pub mod display;
pub mod input;

use super::video_encoder::{config::libx264::Libx264Config, encoder::VideoEncoder};
use crate::{
    component::client::endpoint::{
        client::ClientSendStream,
        message::{KeyboardEvent, MouseEvent},
    },
    error::{CoreError, CoreResult},
};

pub struct Screen {
    display: display::Display,

    duplicator_start_tx: Option<tokio::sync::oneshot::Sender<()>>,
    duplicator_stop_tx: Option<tokio::sync::oneshot::Sender<()>>,
}

impl Screen {
    pub fn new(display_id: &str, client_send_stream: ClientSendStream) -> CoreResult<Self> {
        let (display, adapter, output) = display::Display::query_display(display_id)?;
        let duplicator = duplicator::Duplicator::new(adapter, output)?;
        let encoder = VideoEncoder::new(Libx264Config::default(), client_send_stream)?;

        let (duplicator_start_tx, duplicator_start_rx) = tokio::sync::oneshot::channel();
        let (duplicator_stop_tx, duplicator_stop_rx) = tokio::sync::oneshot::channel();

        Self::serve_desktop_capture(duplicator, encoder, duplicator_start_rx, duplicator_stop_rx);

        Ok(Screen {
            display,
            duplicator_start_tx: Some(duplicator_start_tx),
            duplicator_stop_tx: Some(duplicator_stop_tx),
        })
    }

    pub fn start(&mut self) {
        if let Some(tx) = self.duplicator_start_tx.take() {
            let _ = tx.send(());
        }
    }

    pub fn stop(&mut self) {
        if let Some(tx) = self.duplicator_stop_tx.take() {
            let _ = tx.send(());
        }
    }

    pub fn input_mouse_event(&self, event: MouseEvent) {
        match event {
            MouseEvent::Up(key, x, y) => {
                let _ = input::mouse_up(&self.display, key, x, y);
            }
            MouseEvent::Down(key, x, y) => {
                let _ = input::mouse_down(&self.display, key, x, y);
            }
            MouseEvent::Move(key, x, y) => {
                let _ = input::mouse_move(&self.display, key, x, y);
            }
            MouseEvent::ScrollWheel(delta) => {
                let _ = input::mouse_scroll_wheel(&self.display, delta);
            }
        }
    }

    pub fn input_keyboard_event(&self, event: KeyboardEvent) {
        match event {
            KeyboardEvent::KeyUp(key) => {
                let _ = input::keyboard_up(key);
            }
            KeyboardEvent::KeyDown(key) => {
                let _ = input::keyboard_down(key);
            }
        }
    }

    fn serve_desktop_capture(
        mut duplicator: duplicator::Duplicator,
        mut encoder: VideoEncoder,
        duplicator_start_rx: tokio::sync::oneshot::Receiver<()>,
        mut duplicator_stop_rx: tokio::sync::oneshot::Receiver<()>,
    ) {
        let (capture_frame_tx, mut capture_frame_rx) = tokio::sync::mpsc::channel(180);

        tokio::task::spawn_blocking(move || {
            if duplicator_start_rx.blocking_recv().is_err() {
                return;
            }

            loop {
                match duplicator_stop_rx.try_recv() {
                    Ok(_) => return,
                    Err(err) => {
                        if let tokio::sync::oneshot::error::TryRecvError::Closed = err {
                            return;
                        }
                    }
                }

                match duplicator.capture() {
                    Ok(capture_frame) => {
                        if capture_frame_tx.blocking_send(capture_frame).is_err() {
                            return;
                        }
                    }
                    Err(err) => {
                        tracing::error!(?err, "desktop duplicator capture failed");
                        break;
                    }
                };
            }
        });

        tokio::task::spawn_blocking(move || loop {
            loop {
                match capture_frame_rx.blocking_recv() {
                    Some(capture_frame) => {
                        if let Err(err) = encoder.encode(capture_frame) {
                            if let CoreError::OutgoingMessageChannelDisconnect = err {
                                tracing::info!("desktop capture and encode process exit");
                                return;
                            } else {
                                tracing::error!(?err, "video encode failed");
                            }
                        }
                    }
                    None => {
                        tracing::error!("capture frame channel closed");
                        return;
                    }
                }
            }
        });
    }
}
