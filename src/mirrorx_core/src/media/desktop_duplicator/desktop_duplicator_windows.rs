use super::windows::duplication::Duplication;
use crate::media::video_encoder::VideoEncoder;
use std::{sync::atomic::{AtomicBool,Ordering}, time::Duration, ops::Sub};
use log::error;
use tokio::time::Instant;

pub struct DesktopDuplicator {
    fps: i32,
    stop: AtomicBool,
    video_encoder: VideoEncoder,
}

impl DesktopDuplicator {
    pub fn new(fps: i32, encoder: VideoEncoder) -> anyhow::Result<Self> {
        Ok(DesktopDuplicator {
            fps,
            stop: AtomicBool::new(false),
            video_encoder: encoder,
        })
    }

    pub fn start(&'static self) -> anyhow::Result<()> {
        let (done_tx, done_rx) = std::sync::mpsc::sync_channel(1);

        std::thread::spawn(move || {
            let mut duplication = match Duplication::new(0) {
                Ok(dup) => dup,
                Err(err) => {
                    done_tx.send(Some(err));
                    return;
                }
            };

            let except_wait_time = Duration::from_millis((1000 / self.fps) as u64);

            loop {
                if self.stop.load(Ordering::Relaxed) {
                    break;
                }

                let start_time = Instant::now();
                if let Err(err) = duplication.capture_frame(
                    |width,
                     height,
                     lumina_plane_bytes_address,
                     lumina_plane_stride,
                     chrominance_plane_bytes_address,
                     chrominance_plane_stride| {
                        self.video_encoder.encode(
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
                    error!("{}", err);
                }

                let remaining = except_wait_time.checked_sub(start_time.elapsed());
                if let Some(remaining)= remaining{
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
        self.stop.store(true, Ordering::Relaxed);
    }
}
