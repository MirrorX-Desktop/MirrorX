use super::pixfmt::AVPixelFormat;

extern "C" {
    pub fn av_image_get_buffer_size(
        pix_fmt: AVPixelFormat,
        width: i32,
        height: i32,
        align: i32,
    ) -> i32;
}
