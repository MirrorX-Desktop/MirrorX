use crate::socket::signaling::message::SignalingMessageError;

pub type MirrorXResult<T> = anyhow::Result<T, MirrorXError>;

#[derive(Debug)]
pub enum MirrorXError {
    Raw(String),
    Timeout,
    ProviderNotInitialized,
    Signaling(SignalingMessageError),
}
