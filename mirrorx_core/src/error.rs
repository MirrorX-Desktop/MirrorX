use std::io;
use thiserror::Error;

pub type CoreResult<T> = Result<T, CoreError>;

#[derive(Error, Debug)]
pub enum CoreError {
    #[error("other error (message={message:?}, file=\"{file}\", line={line})")]
    Other {
        message: String,
        file: String,
        line: String,
    },

    #[error("io error ({0:?})")]
    IO(#[from] io::Error),

    #[error("convert string to cstring failed")]
    CStringNullError(#[from] std::ffi::NulError),

    #[error("parse string to int failed")]
    ParseIntError(#[from] std::num::ParseIntError),

    #[error("sqlite Error ({0:?})")]
    SQLiteError(#[from] rusqlite::Error),

    #[error("operation timeout")]
    Timeout(#[from] tokio::time::error::Elapsed),

    #[error("bincode serialization or deserialization failed ({0:?})")]
    BincodeError(#[from] bincode::Error),
}
