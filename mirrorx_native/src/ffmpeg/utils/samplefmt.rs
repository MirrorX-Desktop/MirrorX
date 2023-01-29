pub type AVSampleFormat = i32;
pub const AV_SAMPLE_FMT_NONE: AVSampleFormat = -1;
pub const AV_SAMPLE_FMT_U8: AVSampleFormat = 0;
pub const AV_SAMPLE_FMT_S16: AVSampleFormat = 1;
pub const AV_SAMPLE_FMT_S32: AVSampleFormat = 2;
pub const AV_SAMPLE_FMT_FLT: AVSampleFormat = 3;
pub const AV_SAMPLE_FMT_DBL: AVSampleFormat = 4;
pub const AV_SAMPLE_FMT_U8P: AVSampleFormat = 5;
pub const AV_SAMPLE_FMT_S16P: AVSampleFormat = 6;
pub const AV_SAMPLE_FMT_S32P: AVSampleFormat = 7;
pub const AV_SAMPLE_FMT_FLTP: AVSampleFormat = 8;
pub const AV_SAMPLE_FMT_DBLP: AVSampleFormat = 9;
pub const AV_SAMPLE_FMT_S64: AVSampleFormat = 10;
pub const AV_SAMPLE_FMT_S64P: AVSampleFormat = 11;

extern "C" {
    pub fn av_get_bytes_per_sample(sample_fmt: AVSampleFormat) -> i32;
    pub fn av_samples_alloc_array_and_samples(
        audio_data: *mut *mut *mut u8,
        linesize: *mut i32,
        nb_channels: i32,
        nb_samples: i32,
        sample_fmt: AVSampleFormat,
        align: i32,
    ) -> i32;
    pub fn av_samples_alloc(
        audio_data: *mut *mut u8,
        linesize: *mut i32,
        nb_channels: i32,
        nb_samples: i32,
        sample_fmt: AVSampleFormat,
        align: i32,
    ) -> i32;
    pub fn av_samples_get_buffer_size(
        linesize: *mut i32,
        nb_channels: i32,
        nb_samples: i32,
        sample_fmt: AVSampleFormat,
        align: i32,
    ) -> i32;
}
