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
