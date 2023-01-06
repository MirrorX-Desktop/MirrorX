#[cfg(target_os = "windows")]
#[macro_export]
macro_rules! HRESULT {
    ($exp:expr) => {
        $exp.map_err(|err| $crate::error::CoreError::HResultError {
            error: err,
            file: file!().to_string(),
            line: line!().to_string(),
        })?
    };
}

#[macro_export]
macro_rules! core_error {
    ($($arg:tt)*) => {
        $crate::error::CoreError::Other {
            message: format!($($arg)*),
            file: file!().to_string(),
            line: line!().to_string(),
        }
    };
}

#[macro_export]
macro_rules! call {
    ($exp:expr) => {
        bincode_serialize(&$exp.map_err(|err| err.to_string()))
    };
}
