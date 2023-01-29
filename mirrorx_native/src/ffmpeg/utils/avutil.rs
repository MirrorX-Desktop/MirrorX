use super::rational::AVRational;

pub const AV_NOPTS_VALUE: i64 = 0x8000000000000000u64 as i64;

pub const AV_TIME_BASE: i32 = 1000000;

pub const AV_TIME_BASE_Q: AVRational = AVRational {
    num: 1,
    den: AV_TIME_BASE,
};

pub type AVPictureType = u32;
pub const AV_PICTURE_TYPE_NONE: AVPictureType = 0;
pub const AV_PICTURE_TYPE_I: AVPictureType = 1;
pub const AV_PICTURE_TYPE_P: AVPictureType = 2;
pub const AV_PICTURE_TYPE_B: AVPictureType = 3;
pub const AV_PICTURE_TYPE_S: AVPictureType = 4;
pub const AV_PICTURE_TYPE_SI: AVPictureType = 5;
pub const AV_PICTURE_TYPE_SP: AVPictureType = 6;
pub const AV_PICTURE_TYPE_BI: AVPictureType = 7;

pub type AVMediaType = i32;
pub const AVMEDIA_TYPE_UNKNOWN: AVMediaType = -1;
pub const AVMEDIA_TYPE_VIDEO: AVMediaType = 0;
pub const AVMEDIA_TYPE_AUDIO: AVMediaType = 1;
pub const AVMEDIA_TYPE_DATA: AVMediaType = 2;
pub const AVMEDIA_TYPE_SUBTITLE: AVMediaType = 3;
pub const AVMEDIA_TYPE_ATTACHMENT: AVMediaType = 4;
