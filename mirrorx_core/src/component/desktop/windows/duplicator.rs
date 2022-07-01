use crate::component::desktop::Frame;
use crossbeam::channel::{Receiver, Sender, TryRecvError};
use std::time::Duration;
use tokio::time::Instant;

use super::duplication::Duplication;

pub struct Duplicator {
    fps: i32,
    exit_tx: Sender<()>,
    exit_rx: Receiver<()>,
    tx: Sender<Frame<'static>>,
}

impl Duplicator {
    pub fn new(fps: i32) -> anyhow::Result<(Duplicator, Receiver<Frame<'static>>)> {
        let (exit_tx, exit_rx) = crossbeam::channel::bounded(1);
        let (tx, rx) = crossbeam::channel::bounded(1);

        Ok((
            Duplicator {
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
