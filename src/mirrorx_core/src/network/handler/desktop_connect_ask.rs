use crate::{
    constant::LOCAL_PASSWORD_AUTH_KEY_PAIR_MAP,
    network::{
        message::{Message, MessageError},
        Client,
    },
};
use log::{error, info};
use rsa::{PublicKeyParts, RsaPrivateKey, RsaPublicKey};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct DesktopConnectAskReq {
    pub offer_device_id: String,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct DesktopConnectAskResp {
    pub agree: bool,
    pub password_auth_public_key_n: Vec<u8>,
    pub password_auth_public_key_e: Vec<u8>,
}

impl DesktopConnectAskReq {
    pub async fn handle(self, _: Arc<Client>) -> anyhow::Result<Message, MessageError> {
        info!("handle desktop connect ask: {:?}", self);

        // generate rsa key pair for end-2-end device password authentication
        let password_auth_private_key =
            RsaPrivateKey::new(&mut rand::rngs::OsRng, 2048).map_err(|err| {
                error!("failed to generate password auth private key: {}", err);
                MessageError::InternalError
            })?;

        let password_auth_public_key = RsaPublicKey::from(&password_auth_private_key);
        let n = password_auth_public_key.n().to_bytes_le();
        let e = password_auth_public_key.e().to_bytes_le();

        let mut local_password_auth_key_pair_map = LOCAL_PASSWORD_AUTH_KEY_PAIR_MAP.lock().unwrap();
        local_password_auth_key_pair_map.insert(
            self.offer_device_id,
            (password_auth_public_key, password_auth_private_key),
        );
        drop(local_password_auth_key_pair_map);

        Ok(Message::DesktopConnectAskResp(DesktopConnectAskResp {
            agree: true,
            password_auth_public_key_n: n,
            password_auth_public_key_e: e,
        }))
    }
}
