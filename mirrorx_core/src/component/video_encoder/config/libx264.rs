use super::{set_codec_ctx_option, EncoderConfig};
use crate::error::CoreResult;
use mirrorx_native::ffmpeg::avcodec::*;
use std::ffi::CString;

pub struct Libx264Config {
    ffmpeg_encoder_name: CString,
}

impl Libx264Config {
    pub fn new() -> Self {
        let ffmpeg_encoder_name = CString::new("libx264").unwrap();

        Libx264Config {
            ffmpeg_encoder_name,
        }
    }
}

impl EncoderConfig for Libx264Config {
    fn apply_option(&self, codec_ctx: *mut AVCodecContext) -> CoreResult<()> {
        set_codec_ctx_option(codec_ctx, "profile", "baseline", 0)?;
        set_codec_ctx_option(codec_ctx, "level", "5.0", 0)?;
        set_codec_ctx_option(codec_ctx, "preset", "ultrafast", 0)?;
        set_codec_ctx_option(codec_ctx, "tune", "zerolatency", 0)?;

        Ok(())
    }

    fn ffmpeg_encoder_name(&self) -> *const i8 {
        self.ffmpeg_encoder_name.as_ptr()
    }

    fn av_codec_id(&self) -> AVCodecID {
        AV_CODEC_ID_H264
    }
}
