use std::{str::FromStr, time::Duration};

use reqwest::{Client, ClientBuilder, Url};
use serde::{Deserialize, Serialize};

pub struct HTTPProvider {
    base_url: Url,
    client: Client,
}

impl HTTPProvider {
    pub fn new() -> anyhow::Result<Self> {
        let client = ClientBuilder::new()
            .timeout(Duration::from_secs(5))
            .build()?;

        Ok(HTTPProvider {
            base_url: Url::from_str("http://localhost:40000")?,
            client,
        })
    }

    pub async fn device_register(&self, req: RegisterReq) -> anyhow::Result<RegisterResp> {
        let url = self.base_url.join("/device/register")?;

        let resp = self
            .client
            .post(url)
            .json(&req)
            .send()
            .await?
            .json::<RegisterResp>()
            .await?;

        Ok(resp)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterReq {
    pub device_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterResp {
    pub token: String,
}
