#![allow(non_upper_case_globals)]

macro_rules! syscall_check {
    ($exp:expr) => {
        $exp.map_err(|err| MirrorXError::Syscall {
            code: err.code(),
            message: err.message().to_string(),
            file: file!().to_string(),
            line: line!().to_string(),
        })?
    };
}

pub mod enumerator;
pub mod log;
pub mod video_encoder;
