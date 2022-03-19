use lazy_static::lazy_static;
use std::{collections::HashMap, time::Duration};
use tokio::sync::RwLock;

use crate::network::{proto, Transporter};

lazy_static! {
    static ref DESKTOP_CONNECTION_MAP: RwLock<HashMap<String, Transporter>> =
        RwLock::new(HashMap::new());
}

pub async fn connect_to(device_id: String) -> anyhow::Result<()> {
    let transporter = Transporter::new().await?;

    let desktop_connect_response: proto::DesktopConnectOfferResp = transporter
        .call(
            &proto::DesktopConnectOfferReq { device_id },
            Duration::from_secs(10),
        )
        .await?;

    Ok(())
}
