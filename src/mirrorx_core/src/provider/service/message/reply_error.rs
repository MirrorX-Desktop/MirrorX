use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum ReplyError {
    Internal,
    Timeout,
    DeviceNotFound,
    CastFailed,
    NotSatisfied,
    PasswordIncorrect,
}
