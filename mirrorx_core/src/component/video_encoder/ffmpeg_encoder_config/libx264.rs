use super::{set_codec_ctx_option, FFMPEGEncoderConfig};
use crate::{error::CoreResult, ffi::ffmpeg::avcodec::AVCodecContext};
use std::ffi::CString;

pub struct Libx264Config {
    ffmpeg_encoder_name: CString,
}

impl Libx264Config {
    pub fn new() -> CoreResult<Box<dyn FFMPEGEncoderConfig>> {
        let ffmpeg_encoder_name = CString::new("libx264")?;

        Ok(Box::new(Self {
            ffmpeg_encoder_name,
        }))
    }
}

impl FFMPEGEncoderConfig for Libx264Config {
    fn apply_option(&self, codec_ctx: *mut AVCodecContext) -> CoreResult<()> {
        set_codec_ctx_option(codec_ctx, "profile", "high", 0)?;
        set_codec_ctx_option(codec_ctx, "level", "5.0", 0)?;
        set_codec_ctx_option(codec_ctx, "preset", "ultrafast", 0)?;
        set_codec_ctx_option(codec_ctx, "tune", "zerolatency", 0)?;

        Ok(())
    }

    fn ffmpeg_encoder_name(&self) -> *const i8 {
        self.ffmpeg_encoder_name.as_ptr()
    }
}
