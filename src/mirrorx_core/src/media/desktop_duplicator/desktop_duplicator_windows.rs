use super::windows::duplication::Duplication;
use crate::media::video_encoder::VideoEncoder;
use crossbeam_channel::{Receiver, Sender, TryRecvError};
use std::{sync::Arc, time::Duration};
use tokio::time::Instant;

pub struct DesktopDuplicator {
    fps: i32,
    video_encoder: Arc<VideoEncoder>,
    exit_tx: Sender<()>,
    exit_rx: Receiver<()>,
}

impl DesktopDuplicator {
    pub fn new(fps: i32, encoder: VideoEncoder) -> anyhow::Result<Self> {
        let (exit_tx, exit_rx) = crossbeam_channel::bounded(1);

        Ok(DesktopDuplicator {
            fps,
            exit_tx,
            exit_rx,
            video_encoder: Arc::new(encoder),
        })
    }

    pub fn start(&self) -> anyhow::Result<()> {
        let (done_tx, done_rx) = std::sync::mpsc::sync_channel(1);
        let except_wait_time = Duration::from_millis((1000 / self.fps) as u64);

        let exit_rx = self.exit_rx.clone();
        let encoder = self.video_encoder.clone();

        std::thread::spawn(move || {
            let mut duplication = match Duplication::new(0) {
                Ok(dup) => {
                    let _ = done_tx.send(None);
                    dup
                }
                Err(err) => {
                    let _ = done_tx.send(Some(err));
                    return;
                }
            };

            loop {
                match exit_rx.try_recv() {
                    Ok(_) => break,
                    Err(err) => match err {
                        TryRecvError::Disconnected => break,
                        TryRecvError::Empty => {}
                    },
                };

                let start_time = Instant::now();
                if let Err(err) = duplication.capture_frame(
                    |width,
                     height,
                     lumina_plane_bytes_address,
                     lumina_plane_stride,
                     chrominance_plane_bytes_address,
                     chrominance_plane_stride| {
                        encoder.encode(
                            width,
                            height,
                            lumina_plane_bytes_address,
                            lumina_plane_stride as i32,
                            chrominance_plane_bytes_address,
                            chrominance_plane_stride as i32,
                            0,
                            0,
                            0,
                            0,
                        );
                    },
                ) {
                    tracing::error!("{}", err);
                }

                let remaining = except_wait_time.checked_sub(start_time.elapsed());
                if let Some(remaining) = remaining {
                    std::thread::sleep(remaining);
                }
            }
        });

        match done_rx.recv_timeout(Duration::from_secs(3)) {
            Ok(res) => match res {
                None => Ok(()),
                Some(err) => Err(anyhow::anyhow!(err)),
            },
            Err(timeout_err) => Err(anyhow::anyhow!(timeout_err)),
        }
    }

    pub fn stop(&mut self) {
        let _ = self.exit_tx.send(());
    }
}
