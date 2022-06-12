use super::windows::duplication::Duplication;
use crate::media::frame::CaptureFrame;
use crossbeam_channel::{Receiver, Sender, TryRecvError};
use std::{sync::Arc, time::Duration};
use tokio::time::Instant;

pub struct DesktopDuplicator {
    fps: i32,
    exit_tx: Sender<()>,
    exit_rx: Receiver<()>,
    tx: Sender<Arc<CaptureFrame>>,
}

impl DesktopDuplicator {
    pub fn new(fps: i32) -> anyhow::Result<(DesktopDuplicator, Receiver<Arc<CaptureFrame>>)> {
        let (exit_tx, exit_rx) = crossbeam_channel::bounded(1);
        let (tx, rx) = crossbeam_channel::bounded::<Arc<CaptureFrame>>(1);

        Ok((
            DesktopDuplicator {
                fps,
                exit_tx,
                exit_rx,
                tx,
            },
            rx,
        ))
    }

    pub fn start(&self) -> anyhow::Result<()> {
        let except_wait_time = Duration::from_millis((1000 / self.fps) as u64);

        let exit_rx = self.exit_rx.clone();
        let tx = self.tx.clone();

        let mut duplication = Duplication::new(0, tx)?;

        std::thread::spawn(move || loop {
            match exit_rx.try_recv() {
                Ok(_) => break,
                Err(err) => match err {
                    TryRecvError::Disconnected => break,
                    TryRecvError::Empty => {}
                },
            };

            let start_time = Instant::now();
            if let Err(err) = duplication.capture_frame() {
                tracing::error!("{}", err);
            }

            let remaining = except_wait_time.checked_sub(start_time.elapsed());
            if let Some(remaining) = remaining {
                std::thread::sleep(remaining);
            }
        });

        Ok(())
    }

    pub fn stop(&mut self) {
        let _ = self.exit_tx.send(());
    }
}
