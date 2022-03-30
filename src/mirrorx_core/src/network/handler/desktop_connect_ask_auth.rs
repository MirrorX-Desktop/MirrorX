use crate::{
    instance::LOCAL_PASSWORD_AUTH_KEY_PAIR_MAP,
    network::{
        message::{Message, MessageError},
        Client,
    },
    service::config::read_device_password,
};
use log::{error, info};
use rsa::PaddingScheme;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct DesktopConnectAskAuthReq {
    pub offer_device_id: String,
    pub secret_message: Vec<u8>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct DesktopConnectAskAuthResp {
    pub password_correct: bool,
}

impl DesktopConnectAskAuthReq {
    pub async fn handle(self, client: Arc<Client>) -> anyhow::Result<Message, MessageError> {
        info!("handle desktop connect ask auth: {:?}", self);

        let mut local_password_auth_key_pair_map = LOCAL_PASSWORD_AUTH_KEY_PAIR_MAP.lock().unwrap();
        let (_, private_key) = match local_password_auth_key_pair_map.remove(&self.offer_device_id)
        {
            Some(key_pair) => key_pair,
            None => {
                error!(
                    "no password auth key pair found for offer device id: {}",
                    self.offer_device_id
                );
                return Err(MessageError::InternalError);
            }
        };
        drop(local_password_auth_key_pair_map);

        let plain_password_bytes = private_key
            .decrypt(PaddingScheme::PKCS1v15Encrypt, &self.secret_message)
            .map_err(|err| {
                error!("failed to decrypt secret password: {}", err);
                MessageError::InternalError
            })?;

        let plain_password = String::from_utf8(plain_password_bytes).map_err(|err| {
            error!("failed to convert secret password to string: {}", err);
            MessageError::InternalError
        })?;

        let local_password = read_device_password().map_err(|err| {
            error!("failed to read device id: {}", err);
            MessageError::InternalError
        })?;

        Ok(Message::DesktopConnectAskAuthResp(
            DesktopConnectAskAuthResp {
                password_correct: local_password.map_or(false, |v| v == plain_password),
            },
        ))
    }
}
