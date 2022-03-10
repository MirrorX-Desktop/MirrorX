use std::fmt::{Display, Formatter};
use std::os::raw::c_int;

#[repr(C)]
/// cbindgen:ignore
pub struct Frame {
    pub width: c_int,
    pub height: c_int,
    pub y_line_size: c_int,
    pub y_buffer: Vec<u8>,
    pub uv_line_size: c_int,
    pub uv_buffer: Vec<u8>,
}

unsafe impl Send for Frame {}

impl Frame {
    pub fn new(
        width: c_int,
        height: c_int,
        y_line_size: c_int,
        y_buffer: Vec<u8>,
        uv_line_size: c_int,
        uv_buffer: Vec<u8>,
    ) -> Self {
        Frame {
            width,
            height,
            y_line_size,
            y_buffer,
            uv_line_size,
            uv_buffer,
        }
    }
}

impl Display for Frame {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Frame {{ width: {}, height: {}, y_line_size: {},  y_buffer_length: {}, uv_line_size: {}, uv_buffer_length: {} }}",
            self.width, self.height, self.y_line_size, self.y_buffer.len(), self.uv_line_size, self.uv_buffer.len()
        )
    }
}
