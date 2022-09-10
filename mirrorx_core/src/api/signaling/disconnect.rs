use super::SignalingClientManager;

pub async fn disconnect() {
    SignalingClientManager::set_client(None).await
}
