use std::fmt::Display;

#[derive(Debug)]
pub enum APIError {
    ConfigNotInitialized,
    ConfigReadFailed,
    ConfigSaveFailed,
    ConfigDeviceIdNotFound,
    RuntimeNotInitialized,
    ServiceNotInitialized,
    ServiceInternal,
    ServiceReplyMismatched,
    ServiceReplyInvalid,
    ServiceNotSatisfied,
}

impl Display for APIError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
