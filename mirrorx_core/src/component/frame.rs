use flutter_rust_bridge::ZeroCopyBuffer;

pub struct DesktopEncodeFrame {
    pub width: i32,
    pub height: i32,
    pub luminance_bytes: Vec<u8>,
    pub luminance_stride: i32,
    pub chrominance_bytes: Vec<u8>,
    pub chrominance_stride: i32,
}

unsafe impl Send for DesktopEncodeFrame {}

#[derive(Clone)]
pub struct DesktopDecodeFrame {
    pub width: i32,
    pub height: i32,
    pub luminance_bytes: ZeroCopyBuffer<Vec<u8>>,
    pub luminance_stride: i32,
    pub chrominance_bytes: ZeroCopyBuffer<Vec<u8>>,
    pub chrominance_stride: i32,
}

unsafe impl Send for DesktopDecodeFrame {}

pub struct AudioEncodeFrame {
    pub format_f32: bool,
    #[cfg(any(target_os = "windows", target_os = "macos"))]
    pub bytes: Vec<f32>,
}

unsafe impl Send for AudioEncodeFrame {}
