use std::os::raw::c_void;

extern "C" {
    pub static kYuvI601Constants: c_void;
    pub static kYuvJPEGConstants: c_void;
    pub static kYuvH709Constants: c_void;
    pub static kYuvF709Constants: c_void;
    pub static kYuv2020Constants: c_void;
    pub static kYuvV2020Constants: c_void;
}

extern "C" {
    pub fn NV12ToARGBMatrix(
        src_y: *const u8,
        src_stride_y: isize,
        src_uv: *const u8,
        src_stride_uv: isize,
        dst_argb: *mut u8,
        dst_stride_argb: isize,
        yuvconstants: *const c_void,
        width: isize,
        height: isize,
    ) -> isize;
}
