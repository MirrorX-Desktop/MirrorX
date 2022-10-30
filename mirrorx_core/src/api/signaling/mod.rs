mod dial;
mod key_exchange;
mod subscribe;
mod visit;
mod visit_reply;

use crate::{core_error, error::CoreResult};
use signaling_proto::message::{GetDomainRequest, RegisterRequest};
use std::path::{Path, PathBuf};
use tokio::sync::mpsc::Sender;
use tonic::transport::Channel;

pub use key_exchange::{KeyExchangeRequest, KeyExchangeResponse};
pub use subscribe::PublishMessage;
pub use visit::{ResourceType, VisitRequest, VisitResponse};
pub use visit_reply::VisitReplyRequest;

#[derive(Default, Debug, Clone)]
pub struct SignalingClient {
    domain: String,
    client: Option<signaling_proto::service::signaling_client::SignalingClient<Channel>>,
    _exit_tx: Option<Sender<()>>,
}

impl SignalingClient {
    pub async fn dial(
        &mut self,
        domain: &str,
        config_path: &Path,
        publish_message_tx: Sender<PublishMessage>,
    ) -> CoreResult<i64> {
        let config =
            crate::api::config::read(config_path)?.ok_or(core_error!("config is empty"))?;

        let domain_config = config
            .domain_configs
            .get(domain)
            .ok_or(core_error!("domain config is empty"))?;

        let mut client = dial::dial(&domain_config.addr).await?;

        let get_domain_response = client.get_domain(GetDomainRequest {}).await?;
        let get_domain_response = get_domain_response.into_inner();

        if get_domain_response.domain != domain {
            return Err(core_error!(
                "mismatched domain, please delete current domain and re-add new one"
            ));
        }

        tracing::info!("register device_id {:?}", domain_config.device_id);

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
            domain_config.device_finger_print.clone(),
            config_path.to_path_buf(),
            publish_message_tx,
            exit_rx,
        )
        .await;

        self.domain = domain.to_string();
        self.client = Some(client);
        self._exit_tx = Some(exit_tx);

        Ok(register_response.device_id)
    }

    pub fn domain(&self) -> &str {
        &self.domain
    }

    pub async fn visit(&self, req: visit::VisitRequest) -> CoreResult<visit::VisitResponse> {
        if let Some(client) = &self.client {
            visit::visit(client.clone(), req).await
        } else {
            Err(core_error!("current signaling client not initialized"))
        }
    }

    pub async fn visit_reply(&self, req: visit_reply::VisitReplyRequest) -> CoreResult<()> {
        if let Some(client) = &self.client {
            visit_reply::visit_reply(client.clone(), req).await
        } else {
            Err(core_error!("current signaling client not initialized"))
        }
    }

    pub async fn key_exchange(
        &self,
        req: key_exchange::KeyExchangeRequest,
    ) -> CoreResult<key_exchange::KeyExchangeResponse> {
        if let Some(client) = &self.client {
            key_exchange::key_exchange(client.clone(), req).await
        } else {
            Err(core_error!("current signaling client not initialized"))
        }
    }
}
