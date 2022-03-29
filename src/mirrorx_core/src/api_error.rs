use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum APIError {
    InternalError,
    Timeout,
    InvalidArguments,
    ConfigError,
    RemoteClientOfflineOrNotExist,
}

impl Display for APIError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            APIError::InternalError => write!(f, "internal error"),
            APIError::Timeout => write!(f, "timeout"),
            APIError::InvalidArguments => write!(f, "invalid arguments"),
            APIError::ConfigError => write!(f, "config error"),
            APIError::RemoteClientOfflineOrNotExist => {
                write!(f, "remote client offline or not exist")
            }
        }
    }
}
