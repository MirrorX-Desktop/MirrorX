#[derive(Debug)]
pub struct Frame {
    pub width: u16,
    pub height: u16,
    pub luminance_buffer: Vec<u8>,
    pub luminance_stride: u16,
    pub chrominance_buffer: Vec<u8>,
    pub chrominance_stride: u16,
}

impl Frame {
    pub fn new(
        width: u16,
        height: u16,
        luminance_buffer: Vec<u8>,
        luminance_stride: u16,
        chrominance_buffer: Vec<u8>,
        chrominance_stride: u16,
    ) -> Frame {
        Frame {
            width,
            height,
            luminance_buffer,
            luminance_stride,
            chrominance_buffer,
            chrominance_stride,
        }
    }

    // pub fn width(&self) -> size_t {
    //     self.width
    // }

    // pub fn height(&self) -> size_t {
    //     self.height
    // }

    // pub fn luminance_buffer(&self) -> &[u8] {
    //     self.luminance_buffer
    // }

    // pub fn luminance_stride(&self) -> size_t {
    //     self.luminance_stride
    // }

    // pub fn chrominance_buffer(&self) -> &[u8] {
    //     self.chrominance_buffer
    // }

    // pub fn chrominance_stride(&self) -> size_t {
    //     self.chrominance_stride
    // }
}
