use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum VideoDecoderError {
    InvalidDecoderName { decoder_name: String },
    UnknownDecoderName { decoder_name: String },
    CreateDecoderFailed,
}

impl Error for VideoDecoderError {}

impl Display for VideoDecoderError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            VideoDecoderError::InvalidDecoderName { decoder_name } => write!(
                f,
                "invalid decoder_name: {}, parse to C String failed",
                decoder_name
            ),
            VideoDecoderError::UnknownDecoderName { decoder_name } => {
                write!(f, "can't find an decoder named {}", decoder_name)
            }
            VideoDecoderError::CreateDecoderFailed => write!(f, "create decoder failed"),
        }
    }
}
