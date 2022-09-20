pub struct CaptureFrame {
    pub width: i32,
    pub height: i32,
    pub luminance_bytes: Vec<u8>,
    pub luminance_stride: i32,
    pub chrominance_bytes: Vec<u8>,
    pub chrominance_stride: i32,
}

unsafe impl Send for CaptureFrame {}
