mod dial;
mod key_exchange;
mod register;
mod subscribe;
mod visit;
mod visit_reply;

use super::config::{Config, DomainConfig};
use crate::{core_error, error::CoreResult};
use signaling_proto::message::RegisterRequest;
use std::{path::PathBuf, sync::Arc};
use tokio::sync::{
    mpsc::{Receiver, Sender},
    RwLock,
};
use tonic::transport::Channel;

pub use register::RegisterResponse;
pub use subscribe::PublishMessage;
pub use visit::{ResourceType, VisitRequest, VisitResponse};

#[derive(Clone)]
pub struct SignalingClient {
    client: signaling_proto::service::signaling_client::SignalingClient<Channel>,
    _exit_tx: Sender<()>,
}

impl SignalingClient {
    pub async fn new(
        mut config: Config,
        config_path: PathBuf,
    ) -> CoreResult<(Self, Config, Receiver<PublishMessage>)> {
        let domain_config = config
            .domain_configs
            .get_mut(&config.primary_domain)
            .ok_or(core_error!("no primary domain's config"))?;

        let mut client = dial::dial(domain_config.addr.as_str()).await?;

        let register_response = client
            .register(RegisterRequest {
                device_id: if domain_config.device_id != 0 {
                    Some(domain_config.device_id)
                } else {
                    None
                },
                device_finger_print: domain_config.device_finger_print.clone(),
            })
            .await?;

        let register_response = register_response.into_inner();

        domain_config.device_id = register_response.device_id;
        config.primary_domain = register_response.domain.clone();

        let (publish_message_tx, publish_message_rx) = tokio::sync::mpsc::channel(8);
        let (exit_tx, exit_rx) = tokio::sync::mpsc::channel(1);

        subscribe::subscribe(
            &mut client,
            domain_config.clone(),
            config_path,
            publish_message_tx,
            exit_rx,
        )
        .await;

        let signaling_client = Self {
            client,
            _exit_tx: exit_tx,
        };

        Ok((signaling_client, config, publish_message_rx))
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
