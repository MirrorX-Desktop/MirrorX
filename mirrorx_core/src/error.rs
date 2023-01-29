use std::{
    io,
    string::{FromUtf16Error, FromUtf8Error},
};
use thiserror::Error;

pub type CoreResult<T> = Result<T, CoreError>;

#[derive(Error, Debug)]
pub enum CoreError {
    #[error("other error (message={message:?}, file = \"{file}\", line = {line})")]
    Other {
        message: String,
        file: String,
        line: String,
    },

    #[error("outgoing message channel is full")]
    OutgoingMessageChannelFull,

    #[error("outgoing message channel is disconnect")]
    OutgoingMessageChannelDisconnect,

    #[error("io error ({0:?})")]
    IO(#[from] io::Error),

    #[error("convert string to cstring failed")]
    CStringNullError(#[from] std::ffi::NulError),

    #[error("parse string to int failed")]
    ParseIntError(#[from] std::num::ParseIntError),

    #[error("sqlite Error ({0:?})")]
    SQLiteError(#[from] rusqlite::Error),

    #[error("operation timeout")]
    Timeout,

    #[error("tokio oneshot channel receive error ({0:?})")]
    OneshotReceiveError(#[from] tokio::sync::oneshot::error::RecvError),

    #[error("bincode serialization or deserialization failed ({0:?})")]
    BincodeError(#[from] bincode::Error),

    #[error("rsa error ({0:?})")]
    RSAError(#[from] rsa::errors::Error),

    #[error("ring unspecified error")]
    RingUnspecifiedError(#[from] ring::error::Unspecified),

    #[error("reqwest error ({0:?})")]
    ReqwestError(#[from] reqwest::Error),

    #[error("url parse error ({0:?})")]
    UrlError(#[from] url::ParseError),

    #[error("serde json serialize/deserialize error ({0:?})")]
    SerdeJsonError(#[from] serde_json::Error),

    #[error("base64 error ({0:?})")]
    Base64Error(#[from] base64::DecodeError),

    #[cfg(target_os = "windows")]
    #[error("windows api hresult error ({error:?}, file = \"{file}\", line = {line})")]
    HResultError {
        error: windows::core::Error,
        file: String,
        line: String,
    },

    #[error("parse utf-8 string to rust string failed")]
    FromUTF8Error(#[from] FromUtf8Error),

    #[error("parse utf-16 string to rust string failed")]
    FromUTF16Error(#[from] FromUtf16Error),

    #[error("enum audio devices failed ({0:?})")]
    AudioDevicesError(#[from] cpal::DevicesError),

    #[error("audio device build stream failed ({0:?})")]
    AudioDeviceBuildStreamError(#[from] cpal::BuildStreamError),

    #[error("audio device play stream failed ({0:?})")]
    AudioDevicePlayStreamError(#[from] cpal::PlayStreamError),

    #[error("audio device get default config failed ({0:?})")]
    AudioDeviceDefaultConfigError(#[from] cpal::DefaultStreamConfigError),

    #[error("r2d2 connection pool error ({0:?})")]
    R2D2PoolError(#[from] r2d2::Error),

    #[error("convert error ({0:?})")]
    ConvertError(#[from] std::convert::Infallible),

    #[error("image process error ({0:?})")]
    ImageError(#[from] image::ImageError),

    #[error("get network interfaces error ({0:?})")]
    NetworkInterfacesError(#[from] network_interface::Error),
}

impl serde::Serialize for CoreError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}
