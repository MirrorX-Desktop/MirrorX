use std::time::Duration;

pub struct DesktopEncodeFrame {
    pub capture_time: Duration,
    pub width: i32,
    pub height: i32,
    pub luminance_bytes: Vec<u8>,
    pub luminance_stride: i32,
    pub chrominance_bytes: Vec<u8>,
    pub chrominance_stride: i32,
}

unsafe impl Send for DesktopEncodeFrame {}

#[derive(Clone)]
pub enum DesktopDecodeFrameFormat {
    NV12,
    YUV420P,
}

// todo: remove clone after stable
#[derive(Clone)]
pub struct DesktopDecodeFrame {
    pub width: i32,
    pub height: i32,
    pub plane_data: Vec<Vec<u8>>,
    pub line_sizes: Vec<i32>,
    pub format: DesktopDecodeFrameFormat,
}

pub struct AudioEncodeFrame {
    pub initial_encoder_params: Option<(u32, u8)>, // sample_rate, channels
    pub buffer: Vec<f32>,
}

unsafe impl Send for AudioEncodeFrame {}
