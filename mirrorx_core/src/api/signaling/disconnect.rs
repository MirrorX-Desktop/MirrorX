use crate::error::CoreResult;

use super::SignalingClientManager;

pub async fn disconnect() -> CoreResult<()> {
    SignalingClientManager::set_client(None).await;
    Ok(())
}
