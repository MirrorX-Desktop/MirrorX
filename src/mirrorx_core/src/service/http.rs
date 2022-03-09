use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Deserialize, Serialize)]
struct AllocIdRequest {
    device_hash: String,
    device_token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct AllocIdResponse {
    device_token: String,
}

pub fn request_device_token() -> anyhow::Result<String> {
    let mut hash_plain = String::new();

    #[cfg(any(target_os = "windows", target_os = "macos", target_os = "linux"))]
    {
        match machine_uid::get() {
            Ok(uid) => hash_plain.push_str(&uid),
            Err(err) => return Err(anyhow!("{}", err)),
        };
    }

    #[cfg(any(target_os = "windows", target_os = "macos", target_os = "linux"))]
    {
        let mac_address = mac_address::get_mac_address()?;
        match mac_address {
            Some(mac) => hash_plain.push_str(&mac.to_string()),
            None => hash_plain.push_str("FF:FF:FF:FF:FF:FF"),
        };
    }

    // todo: android and ios
    let mut hasher = Sha256::new();
    hasher.update(hash_plain);
    let result = hasher.finalize();
    let result_hex = hex::encode(result);

    let form_body = AllocIdRequest {
        device_hash: result_hex,
        device_token: None,
    };

    let client = reqwest::blocking::Client::new();
    let resp = client
        .patch("http://127.0.0.1:52500/v1/id")
        .form(&form_body)
        .send()?
        .json::<AllocIdResponse>()?;

    Ok(resp.device_token)
}
