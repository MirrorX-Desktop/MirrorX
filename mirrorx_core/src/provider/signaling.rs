use crate::socket::signaling::client::SignalingClient;
use anyhow::bail;
use arc_swap::ArcSwapOption;
use std::sync::Arc;
use tokio::net::ToSocketAddrs;

static CURRENT_SIGNALING_CLIENT: ArcSwapOption<SignalingClient> = ArcSwapOption::const_empty();

pub async fn init<A>(addr: A) -> anyhow::Result<()>
where
    A: ToSocketAddrs,
{
    let client = SignalingClient::connect(addr).await?;
    CURRENT_SIGNALING_CLIENT.store(Some(Arc::new(client)));
    Ok(())
}

pub async fn handshake(token: String) -> anyhow::Result<()> {
    match CURRENT_SIGNALING_CLIENT.load().as_ref() {
        Some(provider) => provider.handshake(token).await,
        None => bail!("handshake: signaling provider not initialized"),
    }
}

pub async fn heartbeat() -> anyhow::Result<()> {
    match CURRENT_SIGNALING_CLIENT.load().as_ref() {
        Some(provider) => provider.heartbeat().await,
        None => bail!("heartbeat: signaling provider not initialized"),
    }
}
