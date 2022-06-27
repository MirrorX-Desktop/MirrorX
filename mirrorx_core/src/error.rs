use std::io;
use thiserror::Error;

use crate::socket::signaling::message::SignalingMessage;

#[derive(Error, Debug)]
pub enum MirrorXError {
    #[error(transparent)]
    Other(#[from] anyhow::Error),
    #[error("io operation failed ({0:?})")]
    IO(#[from] io::Error),
    #[error("operation timeout")]
    Timeout,
    #[error("component uninitialized")]
    ComponentUninitialized,
    #[error("serialize failed ({0:?})")]
    SerializeFailed(bincode::Error),
    #[error("deserialize failed ({0:?})")]
    DeserializeFailed(bincode::Error),
    #[error("signaling returns unexcepted message {0:?}")]
    SignalingError(SignalingMessage),
    #[error("endpoint '{0}' returns error")]
    EndPointError(String),
    #[error("local device id is empty or invalid")]
    LocalDeviceIDInvalid,
    #[error("local device password is empty or invalid")]
    LocalDevicePasswordInvalid,
    // endpoint
    #[error("endpoint '{0}' not found")]
    EndPointNotFound(String),
    #[error("endpoint '{0}' texture_id already set")]
    EndPointTextureIDAlreadySet(String),
    #[error("endpoint '{0}' video_texture_ptr already set")]
    EndPointVideoTexturePtrAlreadySet(String),
    #[error("endpoint '{0}' update_frame_callback_ptr already set")]
    EndPointUpdateFrameCallbackPtrAlreadySet(String),
    // media video encoder
    #[error("media video encoder can't find encoder by name '{0}'")]
    MediaVideoEncoderNotFound(String),
    #[error("media video encoder alloc context failed")]
    MediaVideoEncoderAllocContextFailed,
    #[error("media video encoder option '{0}' not found")]
    MediaVideoEncoderOptionNotFound(String),
    #[error("media video encoder option '{key}':'{value}' is out of range")]
    MediaVideoEncoderOptionValueOutOfRange { key: String, value: String },
    #[error("media video encoder option '{key}':'{value}' is invalid")]
    MediaVideoEncoderOptionValueInvalid { key: String, value: String },
    #[error(
        "media video encoder option '{key}':'{value}' set failed, av_opt_set returns {error_code}"
    )]
    MediaVideoEncoderOptionSetFailed {
        key: String,
        value: String,
        error_code: i32,
    },
    #[error("media video encoder already opened")]
    MediaVideoEncoderAlreadyOpened,
    #[error("media video encoder open returns {0}")]
    MediaVideoEncoderOpenFailed(i32),
    #[error("media video encoder av_frame_alloc failed")]
    MediaVideoEncoderAVFrameAllocFailed,
    #[error("media video encoder av_frame_get_buffer failed {0}")]
    MediaVideoEncoderAVFrameGetBufferFailed(i32),
    #[error("media video encoder av_packet_alloc failed")]
    MediaVideoEncoderAVPacketAllocFailed,
    #[error("media video encoder av_new_packet returns {0}")]
    MediaVideoEncoderAVPacketCreateFailed(i32),
    #[error("media video encoder av_frame_make_writable returns {0}")]
    MediaVideoEncoderAVFrameMakeWritableFailed(i32),
    #[error("media video encoder can't accept more frames")]
    MediaVideoEncoderFrameUnacceptable,
    #[error("media video encoder had closed")]
    MediaVideoEncoderClosed,
    #[error("media video encoder avcodec_send_frame returns {0}")]
    MediaVideoEncoderSendFrameFailed(i32),
    #[error("media video encoder avcodec_receive_packet returns {0}")]
    MediaVideoEncoderReceivePacketFailed(i32),
    #[error("media video encoder output tx send failed")]
    MediaVideoEncoderOutputTxSendFailed,
    // media video decoder
    #[error("media video decoder can't find decoder by name '{0}'")]
    MediaVideoDecoderNotFound(String),
    #[error("media video decoder alloc context failed")]
    MediaVideoDecoderAllocContextFailed,
    #[error("media video decoder option '{0}' not found")]
    MediaVideoDecoderOptionNotFound(String),
    #[error("media video decoder option '{key}':'{value}' is out of range")]
    MediaVideoDecoderOptionValueOutOfRange { key: String, value: String },
    #[error("media video decoder option '{key}':'{value}' is invalid")]
    MediaVideoDecoderOptionValueInvalid { key: String, value: String },
    #[error(
        "media video decoder option '{key}':'{value}' set failed, av_opt_set returns {error_code}"
    )]
    MediaVideoDecoderOptionSetFailed {
        key: String,
        value: String,
        error_code: i32,
    },
    #[error("media video decoder av_packet_alloc failed")]
    MediaVideoDecoderAVPacketAllocFailed,
    #[error("media video decoder av_frame_alloc failed")]
    MediaVideoDecoderAVFrameAllocFailed,
    #[error("media video decoder av_parser_init failed")]
    MediaVideoDecoderParserInitFailed,
    #[error("media video decoder av_hwdevice_ctx_create returns {0}")]
    MediaVideoDecoderHWDeviceCreateFailed(i32),
    #[error("media video decoder av_frame_alloc for hw failed")]
    MediaVideoDecoderHWAVFrameAllocFailed,
    #[error("media video decoder already opened")]
    MediaVideoDecoderAlreadyOpened,
    #[error("media video decoder open returns {0}")]
    MediaVideoDecoderOpenFailed(i32),
    #[error("media video decoder av_parser_parse2 returns {0}")]
    MediaVideoDecoderParser2Failed(i32),
    #[error("media video decoder can't accept more packets")]
    MediaVideoDecoderPacketUnacceptable,
    #[error("media video decoder had closed")]
    MediaVideoDecoderClosed,
    #[error("media video decoder avcodec_send_packet returns {0}")]
    MediaVideoDecoderSendPacketFailed(i32),
    #[error("media video decoder avcodec_receive_frame returns {0}")]
    MediaVideoDecoderReceiveFrameFailed(i32),
    #[error("media video decoder output tx send failed")]
    MediaVideoDecoderOutputTxSendFailed,
}
