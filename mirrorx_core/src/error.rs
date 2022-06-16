use crate::socket::signaling::message::SignalingMessageError;

pub type MirrorXResult<T> = anyhow::Result<T, MirrorXError>;

pub enum MirrorXError {
    Raw(String),
    Timeout,
    Signaling(SignalingMessageError),
}
