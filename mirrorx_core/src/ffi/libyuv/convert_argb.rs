use super::YuvConstants;

extern "C" {
    // pub static kYuvI601Constants: c_void;
    // pub static kYuvJPEGConstants: c_void;
    pub static kYuvH709Constants: YuvConstants;
    pub static kYuvF709Constants: YuvConstants;
    // pub static kYuv2020Constants: c_void;
    // pub static kYuvV2020Constants: c_void;
}

extern "C" {
    // pub static kYuvI601Constants: c_void;
    // pub static kYuvJPEGConstants: c_void;
    pub static kYvuH709Constants: YuvConstants;
    pub static kYvuF709Constants: YuvConstants;
    // pub static kYuv2020Constants: c_void;
    // pub static kYuvV2020Constants: c_void;
}

extern "C" {
    #[link(kind = "static")]
    pub fn NV12ToARGBMatrix(
        src_y: *const u8,
        src_stride_y: isize,
        src_uv: *const u8,
        src_stride_uv: isize,
        dst_argb: *mut u8,
        dst_stride_argb: isize,
        yuvconstants: *const YuvConstants,
        width: isize,
        height: isize,
    ) -> isize;

    #[link(kind = "static")]
    pub fn NV21ToARGBMatrix(
        src_y: *const u8,
        src_stride_y: isize,
        src_uv: *const u8,
        src_stride_uv: isize,
        dst_argb: *mut u8,
        dst_stride_argb: isize,
        yuvconstants: *const YuvConstants,
        width: isize,
        height: isize,
    ) -> isize;
}
