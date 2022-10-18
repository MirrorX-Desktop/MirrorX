mod dial;
mod key_exchange;
mod register;
mod subscribe;
mod visit;
mod visit_reply;

use crate::{core_error, error::CoreResult};
use signaling_proto::message::{GetDomainRequest, RegisterRequest};
use std::path::PathBuf;
use tokio::sync::mpsc::Sender;
use tonic::transport::Channel;

pub use key_exchange::{KeyExchangeRequest, KeyExchangeResponse};
pub use register::RegisterResponse;
pub use subscribe::PublishMessage;
pub use visit::{ResourceType, VisitRequest, VisitResponse};
pub use visit_reply::VisitReplyRequest;

use super::config::DomainConfig;

#[derive(Debug, Clone)]
pub struct SignalingClient {
    client: signaling_proto::service::signaling_client::SignalingClient<Channel>,
    _exit_tx: Sender<()>,
}

impl SignalingClient {
    pub async fn new(
        domain: String,
        domain_config: DomainConfig,
        config_path: PathBuf,
        publish_message_fn: Box<dyn Fn(PublishMessage) + Send>,
    ) -> CoreResult<(Self, i64)> {
        let mut client = dial::dial(&domain_config.addr).await?;

        let get_domain_response = client.get_domain(GetDomainRequest {}).await?;
        let get_domain_response = get_domain_response.into_inner();

        if get_domain_response.domain != domain {
            return Err(core_error!(
                "mismatch domain, please delete current domain and re-add new one"
            ));
        }

        let register_response = client
            .register(RegisterRequest {
                device_id: domain_config.device_id,
                device_finger_print: domain_config.device_finger_print.to_string(),
            })
            .await?;

        let register_response = register_response.into_inner();

        let (exit_tx, exit_rx) = tokio::sync::mpsc::channel(1);

        subscribe::subscribe(
            &mut client,
            get_domain_response.domain.clone(),
            register_response.device_id,
            domain_config.device_finger_print,
            config_path,
            publish_message_fn,
            exit_rx,
        )
        .await;

        let signaling_client = Self {
            client,
            _exit_tx: exit_tx,
        };

        Ok((signaling_client, register_response.device_id))
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
}
