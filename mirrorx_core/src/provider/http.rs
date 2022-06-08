use anyhow::bail;
use once_cell::sync::OnceCell;
use reqwest::{Client, ClientBuilder, Url};
use serde::{Deserialize, Serialize};
use std::{str::FromStr, time::Duration};

static CURRENT_HTTP_PROVIDER: OnceCell<HTTPProvider> = OnceCell::new();

pub struct HTTPProvider {
    base_url: Url,
    client: Client,
}

impl HTTPProvider {
    pub fn current() -> anyhow::Result<&'static HTTPProvider> {
        CURRENT_HTTP_PROVIDER
            .get()
            .ok_or_else(|| anyhow::anyhow!("HTTPProvider: uninitialized"))
    }

    pub fn make_current() -> anyhow::Result<()> {
        match CURRENT_HTTP_PROVIDER.get_or_try_init(|| -> anyhow::Result<HTTPProvider> {
            let client = ClientBuilder::new()
                .timeout(Duration::from_secs(5))
                .build()?;

            let provider = HTTPProvider {
                base_url: Url::from_str("http://192.168.0.101:40000")?,
                client,
            };

            Ok(provider)
        }) {
            Ok(_) => Ok(()),
            Err(err) => bail!("HTTPProvider: make current failed: {}", err),
        }
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
