use super::subscribe_message::VisitFailureReason;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub enum HttpError {
    Internal,
    Timeout,
    InvalidArgs,
    ResourceExhausted,
    RemoteOffline,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Response<T> {
    Message(T),
    Error(HttpError),
}

#[derive(Debug, Deserialize)]
pub struct IdentityResponse {
    pub domain: String,
    pub min_client_version: String,
    pub signaling_port: u16,
    pub subscribe_port: u16,
}

#[derive(Debug, Serialize)]
pub struct RegisterRequest {
    pub device_id: i64,
    pub device_finger_print: String,
}

#[derive(Debug, Deserialize)]
pub struct RegisterResponse {
    pub device_id: i64,
    pub expire: i64,
}

#[derive(Serialize)]
pub struct VisitRequest {
    pub active_device_id: i64,
    pub passive_device_id: i64,
    pub visit_desktop: bool,
    pub password_salt: String,
    pub secret: String,
    pub secret_nonce: String,
}

#[derive(Deserialize)]
pub struct VisitResponse {
    pub endpoint_addr: String,
    pub visit_credentials: String,
    pub result: Result<String, VisitFailureReason>,
}
