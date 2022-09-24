#![allow(non_snake_case)]

pub mod audio;
pub mod desktop;
pub mod frame;
pub mod input;
pub mod video_decoder;
pub mod video_encoder;

pub const NALU_HEADER_LENGTH: usize = 4;
