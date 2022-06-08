use std::ffi::CStr;
use std::os::raw::{c_char, c_int};

// this function is the "bridge" from C log function, it should be called from C, don't use it in rust.
#[no_mangle]
/// cbindgen:ignore
pub extern "C" fn log_to_rust(level: c_int, message: *const c_char) {
    unsafe {
        match CStr::from_ptr(message).to_str() {
            Ok(message) => {
                let message = message.trim();
                match level {
                    1 => tracing::trace!(message),
                    2 => tracing::debug!(message),
                    3 => tracing::info!(message),
                    4 => tracing::warn!(message),
                    5 => tracing::error!(message),
                    _ => {
                        tracing::warn!(
                            "unknown ffi_log level: {}, the message is '{}'",
                            level,
                            message
                        );
                    }
                }
            }
            Err(err) => tracing::error!(err = ?err,
                "invalid ffi_log message, convert from raw pointer(*const c_char) failed",
            ),
        }
    }
}
