#[cfg(target_os = "windows")]
#[macro_export]
macro_rules! check_if_failed {
    ($exp:expr) => {
        $exp.map_err(|err| MirrorXError::Syscall {
            code: err.code(),
            message: err.message().to_string(),
            file: file!().to_string(),
            line: line!().to_string(),
        })?
    };
}
