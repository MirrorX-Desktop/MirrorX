pub mod h264_videotoolbox;
pub mod hevc_videotoolbox;
pub mod libx264;

use crate::{core_error, error::CoreResult};
use mirrorx_native::ffmpeg::{
    codecs::{avcodec::AVCodecContext, codec_id::AVCodecID},
    utils::{
        error::{AVERROR, AVERROR_OPTION_NOT_FOUND},
        opt::av_opt_set,
    },
};
use std::ffi::CString;

pub trait EncoderConfig {
    fn apply_option(&self, codec_ctx: *mut AVCodecContext) -> CoreResult<()>;
    fn ffmpeg_encoder_name(&self) -> *const i8;
    fn av_codec_id(&self) -> AVCodecID;
}

fn set_codec_ctx_option(
    codec_ctx: *mut AVCodecContext,
    key: &str,
    value: &str,
    search_flags: i32,
) -> CoreResult<()> {
    let opt_name = CString::new(key.to_string())?;
    let opt_value = CString::new(value.to_string())?;

    unsafe {
        let ret = av_opt_set(
            (*codec_ctx).priv_data,
            opt_name.as_ptr(),
            opt_value.as_ptr(),
            search_flags,
        );

        if ret == AVERROR_OPTION_NOT_FOUND {
            Err(core_error!(
                "set AVCodecContext returns AVERROR_OPTION_NOT_FOUND {:?}:{:?}",
                opt_name,
                opt_value
            ))
        } else if ret == AVERROR(libc::ERANGE) {
            Err(core_error!("set AVCodecContext returns ERANGE"))
        } else if ret == AVERROR(libc::EINVAL) {
            Err(core_error!("set AVCodecContext returns EINVAL"))
        } else if ret != 0 {
            Err(core_error!(
                "set AVCodecContext returns error code: {}",
                ret
            ))
        } else {
            Ok(())
        }
    }
}
