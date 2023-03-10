mod duplicator;

pub mod display;
pub mod input;

use self::display::Display;

use super::video_encoder::{config::libx264::Libx264Config, encoder::VideoEncoder};
use crate::{
    core_error,
    error::{CoreError, CoreResult},
    service::endpoint::{
        self,
        message::{EndPointInputEvent, KeyboardEvent, MouseEvent},
    },
};
use std::sync::Arc;

pub struct Screen {
    display: display::Display,
    duplicator_start_tx: Option<tokio::sync::oneshot::Sender<()>>,
    duplicator_stop_tx: Option<tokio::sync::oneshot::Sender<()>>,
    input_event_tx: tokio::sync::mpsc::Sender<EndPointInputEvent>,
}

impl Screen {
    pub fn new(display_id: &str, service: Arc<endpoint::Service>) -> CoreResult<Self> {
        let (display, adapter, output) = display::Display::query_display(display_id)?;
        let duplicator = duplicator::Duplicator::new(adapter, output)?;
        let encoder = VideoEncoder::new(Libx264Config::default(), service)?;

        let (duplicator_start_tx, duplicator_start_rx) = tokio::sync::oneshot::channel();
        let (duplicator_stop_tx, duplicator_stop_rx) = tokio::sync::oneshot::channel();
        let (input_event_tx, input_event_rx) = tokio::sync::mpsc::channel(1);

        Self::serve_desktop_capture(duplicator, encoder, duplicator_start_rx, duplicator_stop_rx);
        Self::serve_input_event(display.clone(), input_event_rx);

        Ok(Screen {
            display,
            duplicator_start_tx: Some(duplicator_start_tx),
            duplicator_stop_tx: Some(duplicator_stop_tx),
            input_event_tx,
        })
    }

    pub fn start_capture_desktop(&mut self) {
        if let Some(tx) = self.duplicator_start_tx.take() {
            let _ = tx.send(());
        }
    }

    pub fn stop_capture_desktop(&mut self) {
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

    pub async fn send_input_event(&self, input_event: EndPointInputEvent) -> CoreResult<()> {
        self.input_event_tx
            .send(input_event)
            .await
            .map_err(|_| core_error!("input event channel closed"))
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
                            break;
                        }
                    }
                }

                match duplicator.capture() {
                    Ok(capture_frame) => {
                        if capture_frame_tx.blocking_send(capture_frame).is_err() {
                            break;
                        }
                    }
                    Err(err) => {
                        tracing::error!(?err, "desktop duplicator capture failed");
                        break;
                    }
                };
            }

            tracing::info!("desktop duplicator exited");
        });

        tokio::task::spawn_blocking(move || loop {
            loop {
                match capture_frame_rx.blocking_recv() {
                    Some(capture_frame) => {
                        if let Err(err) = encoder.encode(capture_frame) {
                            if let CoreError::OutgoingMessageChannelDisconnect = err {
                                break;
                            } else {
                                tracing::warn!(?err, "video encode failed");
                            }
                        }
                    }
                    None => {
                        tracing::error!("capture frame channel closed");
                        break;
                    }
                }
            }

            tracing::info!("video encoder exited");
        });
    }

    fn serve_input_event(
        display: Display,
        mut input_event_rx: tokio::sync::mpsc::Receiver<EndPointInputEvent>,
    ) {
        tokio::spawn(async move {
            while let Some(input_event) = input_event_rx.blocking_recv() {
                for event in input_event.events {
                    match event {
                        endpoint::message::InputEvent::Mouse(mouse_event) => match mouse_event {
                            MouseEvent::Up(key, x, y) => {
                                let _ = input::mouse_up(&display, key, x, y);
                            }
                            MouseEvent::Down(key, x, y) => {
                                let _ = input::mouse_down(&display, key, x, y);
                            }
                            MouseEvent::Move(key, x, y) => {
                                let _ = input::mouse_move(&display, key, x, y);
                            }
                            MouseEvent::ScrollWheel(delta) => {
                                let _ = input::mouse_scroll_wheel(&display, delta);
                            }
                        },
                        endpoint::message::InputEvent::Keyboard(keyboard_event) => {
                            match keyboard_event {
                                KeyboardEvent::KeyUp(key) => {
                                    let _ = input::keyboard_up(key);
                                }
                                KeyboardEvent::KeyDown(key) => {
                                    let _ = input::keyboard_down(key);
                                }
                            }
                        }
                    }
                }
            }
        });
    }
}
