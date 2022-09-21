use super::{set_codec_ctx_option, EncoderConfig};
use crate::{
    error::CoreResult,
    ffi::ffmpeg::avcodec::{AVCodecContext, AVCodecID, AV_CODEC_ID_H264},
};
use std::ffi::CString;

pub struct H264VideoToolboxConfig {
    ffmpeg_encoder_name: CString,
}

impl H264VideoToolboxConfig {
    pub fn new() -> Self {
        let ffmpeg_encoder_name = CString::new("h264_videotoolbox").unwrap();

        H264VideoToolboxConfig {
            ffmpeg_encoder_name,
        }
    }
}

impl EncoderConfig for H264VideoToolboxConfig {
    fn apply_option(&self, codec_ctx: *mut AVCodecContext) -> CoreResult<()> {
        set_codec_ctx_option(codec_ctx, "profile", "high", 0)?;
        set_codec_ctx_option(codec_ctx, "level", "5.0", 0)?;
        set_codec_ctx_option(codec_ctx, "realtime", "true", 0)?;
        // set_codec_ctx_option(codec_ctx, "prio_speed", "true", 0)?;

        Ok(())
    }

    fn ffmpeg_encoder_name(&self) -> *const i8 {
        self.ffmpeg_encoder_name.as_ptr()
    }

    fn av_codec_id(&self) -> AVCodecID {
        AV_CODEC_ID_H264
    }
}
