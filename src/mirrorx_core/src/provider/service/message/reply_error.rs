use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub enum ReplyError {
    Internal,
    Timeout,
    DeviceNotFound,
    CastFailed,
    RepeatedRequest,
    NotSatisfied,
    PasswordIncorrect,
}
