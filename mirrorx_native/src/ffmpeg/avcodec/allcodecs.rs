use super::AVCodec;
use std::os::raw::c_void;

extern "C" {
    pub fn av_codec_iterate(opaque: *mut *mut c_void) -> *const AVCodec;
}
