pub struct DesktopDuplicator {
    fps: i32,
}

impl DesktopDuplicator {
    pub fn new(fps: i32, encoder: VideoEncoder) -> anyhow::Result<Self> {
        Ok(DesktopDuplicator { fps })
    }

    pub fn start(&mut self) {
        self.capture_session.start_running();
    }

    pub fn stop(&mut self) {
        self.capture_session.stop_running();
    }
}
