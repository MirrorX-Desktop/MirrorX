use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum ProtoError {
    UnEnoughBytes(usize),
}

impl Error for ProtoError {}

impl Display for ProtoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProtoError::UnEnoughBytes(n) => write!(f, "ProtoError: un-enough bytes ({})", n),
        }
    }
}
