use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum VideoEncoderError {
    InvalidEncoderName { encoder_name: String },
    UnknownEncoderName { encoder_name: String },
    CreateEncoderFailed,
}

impl Error for VideoEncoderError {}

impl Display for VideoEncoderError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            VideoEncoderError::InvalidEncoderName { encoder_name } => write!(
                f,
                "invalid encoder_name: {}, parse to C String failed",
                encoder_name
            ),
            VideoEncoderError::UnknownEncoderName { encoder_name } => {
                write!(f, "can't find an encoder named {}", encoder_name)
            }
            VideoEncoderError::CreateEncoderFailed => write!(f, "create encoder failed"),
        }
    }
}
