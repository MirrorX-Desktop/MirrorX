pub struct CaptureFrame {
    pub width: u16,
    pub height: u16,
    pub bytes: Vec<u8>,
    pub stride: u16,
}

unsafe impl Send for CaptureFrame {}
