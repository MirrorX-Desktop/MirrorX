pub struct CaptureFrame {
    pub width: u16,
    pub height: u16,
    pub luminance_bytes: Vec<u8>,
    pub luminance_stride: u16,
    pub chrominance_bytes: Vec<u8>,
    pub chrominance_stride: u16,
}

unsafe impl Send for CaptureFrame {}
