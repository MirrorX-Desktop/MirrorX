use anyhow::bail;
use dashmap::DashMap;
use flutter_rust_bridge::{StreamSink, ZeroCopyBuffer};
use once_cell::sync::OnceCell;

static CURRENT_FRAME_STREAM_PROVIDER: OnceCell<FrameStreamProvider> = OnceCell::new();

pub struct FrameStreamProvider {
    streams: DashMap<String, StreamSink<ZeroCopyBuffer<Vec<u8>>>>,
}

impl FrameStreamProvider {
    pub fn current() -> anyhow::Result<&'static FrameStreamProvider> {
        CURRENT_FRAME_STREAM_PROVIDER
            .get()
            .ok_or_else(|| anyhow::anyhow!("FrameStreamProvider: uninitialized"))
    }

    pub fn make_current() -> anyhow::Result<()> {
        match CURRENT_FRAME_STREAM_PROVIDER.get_or_try_init(
            || -> anyhow::Result<FrameStreamProvider> {
                let provider = FrameStreamProvider {
                    streams: DashMap::new(),
                };

                Ok(provider)
            },
        ) {
            Ok(_) => Ok(()),
            Err(err) => bail!("FrameStreamProvider: make current failed: {}", err),
        }
    }

    pub fn add(&self, remote_device_id: String, stream_sink: StreamSink<ZeroCopyBuffer<Vec<u8>>>) {
        self.streams.insert(remote_device_id, stream_sink);
    }

    pub fn remove(&self, remote_device_id: &str) {
        self.streams.remove(remote_device_id);
    }

    pub fn send(&self, remote_device_id: &str, buffer: Vec<u8>) {
        if let Some(stream) = self.streams.get(remote_device_id) {
            let success = stream.add(ZeroCopyBuffer(buffer));
            drop(stream);
            if !success {
                self.streams.remove(remote_device_id);
                tracing::error!("frame stream send failed");
            }
        }
    }
}
