use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[derive(Debug, Serialize, Deserialize)]
pub enum PortalClientMessage {
    Error(PortalError),
    ServerConfigRequest,
    ClientRegisterRequest(ClientRegisterRequest),
    CheckRemoteDeviceIsOnlineRequest(CheckRemoteDeviceIsOnlineRequest),
    ActiveVisitRequest(ActiveVisitRequest),
    VisitPassiveReply(VisitPassiveReply),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PortalServerMessage {
    Error(PortalError),
    ServerConfigReply(ServerConfigReply),
    ClientRegisterReply(ClientRegisterReply),
    CheckRemoteDeviceIsOnlineReply(bool),
    VisitPassiveRequest(VisitPassiveRequest),
    ActiveVisitReply(ActiveVisitReply),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActiveEndpointKeyExchangeSecret<'a> {
    pub exchange_reply_public_key_n: &'a [u8],
    pub exchange_reply_public_key_e: &'a [u8],
    pub active_exchange_public_key: &'a [u8],
    pub active_exchange_nonce: &'a [u8],
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PassiveEndpointKeyExchangeSecret<'a> {
    pub passive_exchange_public_key: &'a [u8],
    pub passive_exchange_nonce: &'a [u8],
}

#[derive(Debug, Serialize, Deserialize, thiserror::Error)]
pub enum PortalError {
    #[error("portal server internal error")]
    Internal,
    #[error("portal call timeout")]
    Timeout,
    #[error("portal call is invalid")]
    InvalidRequest,
    #[error("remote device internal error")]
    RemoteInternal,
    #[error("remote device refuse request")]
    RemoteRefuse,
    #[error("remote device is offline")]
    RemoteOffline,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerConfigReply {
    pub name: String,
    pub min_client_version: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientRegisterRequest {
    pub device_id: i64,
    pub device_finger_print: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientRegisterReply {
    pub device_id: i64,
    pub expire: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CheckRemoteDeviceIsOnlineRequest {
    pub device_id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActiveVisitRequest {
    pub active_device_id: i64,
    pub passive_device_id: i64,
    pub visit_desktop: bool,
    pub password_salt: Vec<u8>,
    pub secret: Vec<u8>,
    pub secret_nonce: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VisitPassiveRequest {
    pub active_visit_req: ActiveVisitRequest,
    pub relay_addr: SocketAddr,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActiveVisitReply {
    pub passive_reply: VisitPassiveReply,
    pub relay_addr: SocketAddr,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VisitPassiveReply {
    pub active_device_id: i64,
    pub passive_device_id: i64,
    pub visit_credentials: String,
    pub secret: Vec<u8>,
}
