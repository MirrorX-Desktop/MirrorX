mod dial;
mod key_exchange;
mod subscribe;
mod visit;
mod visit_reply;

use crate::{core_error, error::CoreResult};

use signaling_proto::service::signaling_client::SignalingClient;
use tokio::sync::mpsc::Sender;
use tonic::transport::Channel;

pub use dial::dial;
pub use key_exchange::{KeyExchangeRequest, KeyExchangeResponse};
pub use signaling_proto::message::{GetDomainRequest, RegisterRequest};
pub use subscribe::PublishMessage;
pub use visit::{ResourceType, VisitRequest, VisitResponse};
pub use visit_reply::VisitReplyRequest;

use super::config::entity::domain::Domain;

#[derive(Debug, Clone)]
pub struct SignalingProvider {
    client: SignalingClient<Channel>,
    _exit_tx: Sender<()>,
}

impl SignalingProvider {
    pub async fn dial(
        domain: &mut Domain,
        publish_message_tx: Sender<PublishMessage>,
    ) -> CoreResult<(Self, bool)> {
        let mut client = dial::dial(&domain.addr).await?;

        let get_domain_response = client.get_domain(GetDomainRequest {}).await?;
        let get_domain_response = get_domain_response.into_inner();

        if get_domain_response.domain != domain.name {
            return Err(core_error!(
                "mismatched domain, please delete current domain and re-add new one"
            ));
        }

        let register_response = client
            .register(RegisterRequest {
                device_id: domain.device_id,
                device_finger_print: domain.finger_print.to_string(),
            })
            .await?;

        let register_response = register_response.into_inner();
        let should_update_domain_device_id = register_response.device_id != domain.device_id;
        domain.device_id = register_response.device_id;

        let (exit_tx, exit_rx) = tokio::sync::mpsc::channel(1);

        subscribe::subscribe(
            &mut client,
            domain.id,
            domain.device_id,
            domain.finger_print.clone(),
            publish_message_tx,
            exit_rx,
        )
        .await;

        Ok((
            Self {
                client,
                _exit_tx: exit_tx,
            },
            should_update_domain_device_id,
        ))
    }

    pub async fn visit(&self, req: visit::VisitRequest) -> CoreResult<visit::VisitResponse> {
        visit::visit(self.client.clone(), req).await
    }

    pub async fn visit_reply(&self, req: visit_reply::VisitReplyRequest) -> CoreResult<()> {
        visit_reply::visit_reply(self.client.clone(), req).await
    }

    pub async fn key_exchange(
        &self,
        req: key_exchange::KeyExchangeRequest,
    ) -> CoreResult<key_exchange::KeyExchangeResponse> {
        key_exchange::key_exchange(self.client.clone(), req).await
    }
}
