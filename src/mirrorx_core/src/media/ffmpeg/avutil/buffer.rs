pub enum AVBuffer {}

#[repr(C)]
pub struct AVBufferRef {
    pub buffer: *mut AVBuffer,
    pub data: *mut u8,
    pub size: usize,
}

extern "C" {
    pub fn av_buffer_ref(buf: *const AVBufferRef) -> *mut AVBufferRef;
}
