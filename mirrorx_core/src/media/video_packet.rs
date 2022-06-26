use std::fmt::Display;

/// cbindgen:ignore
#[derive(Debug)]
pub struct VideoPacket {
    pub data: Vec<u8>,
    pub dts: i64,
    pub pts: i64,
}

impl Display for VideoPacket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Packet {{ data: {{ length: {} }}, dts: {}, pts: {} }}",
            self.data.len(),
            self.dts,
            self.pts
        )
    }
}
