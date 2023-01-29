use cpal::SampleFormat;
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

impl Default for DesktopDecodeFrame {
    fn default() -> Self {
        Self {
            width: 0,
            height: 0,
            plane_data: Vec::new(),
            line_sizes: Vec::new(),
            format: DesktopDecodeFrameFormat::NV12,
        }
    }
}

pub struct AudioEncodeFrame {
    pub channels: u16,
    pub sample_format: SampleFormat,
    pub sample_rate: u32,
    pub buffer: Vec<u8>,
}

unsafe impl Send for AudioEncodeFrame {}
