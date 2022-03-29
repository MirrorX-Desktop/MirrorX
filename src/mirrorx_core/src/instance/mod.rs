use std::{collections::HashMap, sync::Mutex};

use once_cell::sync::Lazy;
use ring::agreement::{EphemeralPrivateKey, PublicKey, UnparsedPublicKey};
use rsa::{RsaPrivateKey, RsaPublicKey};

// pub static LOCAL_KEY_PAIR_MAP: Lazy<Mutex<HashMap<String, (PublicKey, EphemeralPrivateKey)>>> =
//     Lazy::new(|| Mutex::new(HashMap::new()));

// pub static REMOTE_KEY_MAP: Lazy<Mutex<HashMap<String, UnparsedPublicKey<Vec<u8>>>>> =
//     Lazy::new(|| Mutex::new(HashMap::new()));

pub static LOCAL_PASSWORD_AUTH_KEY_PAIR_MAP: Lazy<
    Mutex<HashMap<String, (RsaPublicKey, RsaPrivateKey)>>,
> = Lazy::new(|| Mutex::new(HashMap::new()));

pub static REMOTE_PASSWORD_AUTH_PUBLIC_KEY_MAP: Lazy<Mutex<HashMap<String, RsaPublicKey>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));
