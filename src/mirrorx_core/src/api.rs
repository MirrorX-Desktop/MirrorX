pub fn request_device_token() -> anyhow::Result<String> {
    crate::service::http::request_device_token()
}
