mod dial;
mod heartbeat;
mod key_exchange;
mod register;
mod subscribe;
mod visit;
mod visit_reply;

use crate::error::CoreResult;
use tonic::transport::Channel;

pub use register::{RegisterRequest, RegisterResponse};
pub use visit::{ResourceType, VisitRequest, VisitResponse};

#[derive(Clone)]
pub struct SignalingClient {
    client: signaling_proto::service::signaling_client::SignalingClient<Channel>,
}

impl SignalingClient {
    pub async fn dial(uri: &str) -> CoreResult<Self> {
        let client = dial::dial(uri).await?;
        Ok(Self { client })
    }

    pub async fn heartbeat(&self, device_id: i64, timestamp: u32) -> CoreResult<u32> {
        heartbeat::heartbeat(&mut self.client.clone(), device_id, timestamp).await
    }

    pub async fn register(
        &self,
        req: register::RegisterRequest,
    ) -> CoreResult<register::RegisterResponse> {
        register::register(&mut self.client.clone(), req).await
    }

    pub async fn visit(&self, req: visit::VisitRequest) -> CoreResult<visit::VisitResponse> {
        visit::visit(&mut self.client.clone(), req).await
    }

    pub async fn visit_reply(&self, req: visit_reply::VisitReplyRequest) -> CoreResult<()> {
        visit_reply::visit_reply(&mut self.client.clone(), req).await
    }

    pub async fn key_exchange(
        &self,
        req: key_exchange::KeyExchangeRequest,
    ) -> CoreResult<key_exchange::KeyExchangeResponse> {
        key_exchange::key_exchange(&mut self.client.clone(), req).await
    }

    pub async fn subscribe(
        &self,
        req: subscribe::SubscribeRequest,
        publish_message_tx: crossbeam::channel::Sender<subscribe::PublishMessage>,
    ) -> CoreResult<()> {
        subscribe::subscribe(&mut self.client.clone(), req, publish_message_tx).await?;
        Ok(())
    }
}

// impl Clone for SignalingClient {
//     fn clone(&self) -> Self {
//         Self {
//             client: self.client.clone(),
//         }
//     }
// }
