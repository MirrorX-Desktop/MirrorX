pub enum OpusEncoder {}

pub const OPUS_APPLICATION_VOIP: isize = 2048;
pub const OPUS_APPLICATION_AUDIO: isize = 2049;
pub const OPUS_APPLICATION_RESTRICTED_LOWDELAY: isize = 2051;

extern "C" {
    pub fn opus_encoder_get_size(channels: isize) -> isize;
    pub fn opus_encoder_create(
        fs: i32,
        channels: isize,
        application: isize,
        error: *mut isize,
    ) -> *mut OpusEncoder;
    pub fn opus_encoder_init(
        st: *mut OpusEncoder,
        fs: i32,
        channels: isize,
        application: isize,
    ) -> isize;
    pub fn opus_encode(
        st: *mut OpusEncoder,
        pcm: *const i16,
        frame_size: isize,
        data: *mut u8,
        max_data_bytes: i32,
    ) -> i32;
    pub fn opus_encode_float(
        st: *mut OpusEncoder,
        pcm: *const f32,
        frame_size: isize,
        data: *mut u8,
        max_data_bytes: i32,
    ) -> i32;
    pub fn opus_encoder_destroy(st: *mut OpusEncoder);
    pub fn opus_encoder_ctl(st: *mut OpusEncoder, request: isize, ...) -> isize;
}
