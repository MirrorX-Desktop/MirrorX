use std::fmt::Display;

pub struct VideoFrame {
    pub width: u16,
    pub height: u16,
    pub y_plane_buffer: Vec<u8>,
    pub y_plane_stride: u32,
    pub uv_plane_buffer: Vec<u8>,
    pub uv_plane_stride: u32,
    pub dts: i64,
    pub dts_scale: i32,
    pub pts: i64,
    pub pts_scale: i32,
}

impl Display for VideoFrame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"VideoFrame {{ width: {}, height: {}, y_plane_buffer: {{ length: {} }}, y_plane_stride:{}, uv_plane_buffer: {{ length: {} }}, uv_plane_stride:{}, dts: {}, dts_scale: {}, pts: {}, pts_scale: {} }}",
            self.width,
            self.height,
            self.y_plane_buffer.len(),
            self.y_plane_stride,
            self.uv_plane_buffer.len(),
            self.uv_plane_stride,
            self.dts,
            self.dts_scale,
            self.pts,
            self.pts_scale
        )
    }
}
