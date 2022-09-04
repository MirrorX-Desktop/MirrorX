pub struct CaptureFrame {
    pub width: u32,
    pub height: u32,
    pub luminance_bytes: Vec<u8>,
    pub luminance_stride: u32,
    pub chrominance_bytes: Vec<u8>,
    pub chrominance_stride: u32,
}

unsafe impl Send for CaptureFrame {}
