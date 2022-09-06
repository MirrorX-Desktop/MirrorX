#[cfg(target_os = "windows")]
#[macro_export]
macro_rules! HRESULT {
    ($exp:expr) => {
        $exp.map_err(|err| CoreError::Other {
            message: format!(
                "Windows API returns failed HRESULT (code={:?}, message={:?})",
                err.code(),
                err.message()
            ),
            file: file!().to_string(),
            line: line!().to_string(),
        })?
    };
}

#[macro_export]
macro_rules! core_error {
    ($($arg:tt)*) => {
        CoreError::Other {
            message: format!($($arg)*),
            file: file!().to_string(),
            line: line!().to_string(),
        }
    };
}
