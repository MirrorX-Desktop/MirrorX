pub mod dial;
pub mod disconnect;
pub mod heartbeat;
pub mod key_exchange;
pub mod register;
pub mod subscribe;
pub mod visit;
pub mod visit_reply;

use crate::{
    core_error,
    error::{CoreError, CoreResult},
};
use once_cell::sync::Lazy;
use signaling_proto::service::signaling_client::SignalingClient;
use tokio::sync::RwLock;
use tonic::transport::Channel;

static CURRENT_SIGNALING_CLIENT_MANAGER: Lazy<SignalingClientManager> =
    Lazy::new(|| SignalingClientManager {
        client: RwLock::new(None),
    });

struct SignalingClientManager {
    client: RwLock<Option<SignalingClient<Channel>>>,
}

impl SignalingClientManager {
    pub async fn get_client() -> CoreResult<SignalingClient<Channel>> {
        let client = CURRENT_SIGNALING_CLIENT_MANAGER.client.read().await;
        if let Some(client) = client.as_ref() {
            Ok(client.clone())
        } else {
            Err(core_error!("signaling client instance not exists"))
        }
    }

    pub async fn set_client(new_client: Option<SignalingClient<Channel>>) {
        let mut client = CURRENT_SIGNALING_CLIENT_MANAGER.client.write().await;
        *client = new_client
    }
}
