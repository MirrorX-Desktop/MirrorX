#[cfg(target_os = "windows")]
#[macro_export]
macro_rules! windows_api_check {
    ($exp:expr) => {
        $exp.map_err(|err| MirrorXError::WindowsAPI {
            code: err.code(),
            message: err.message().to_string(),
            file: file!().to_string(),
            line: line!().to_string(),
        })?
    };
}

#[macro_export]
macro_rules! api_error {
    ($message:expr) => {
        MirrorXError::API {
            message: $message.to_string(),
            file: file!().to_string(),
            line: line!().to_string(),
        }
    };
}
