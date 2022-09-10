mod key_exchange;

use super::SignalingClientManager;
use crate::{error::CoreResult, utility::runtime::TOKIO_RUNTIME};
use flutter_rust_bridge::StreamSink;
use scopeguard::defer;

pub enum PublishMessage {
    StreamClosed,
    VisitRequest {
        active_device_id: String,
        passive_device_id: String,
        resource_type: crate::api::signaling::visit::ResourceType,
    },
}

pub struct SubscribeRequest {
    pub local_device_id: String,
    pub device_finger_print: String,
    pub config_path: String,
}

pub async fn subscribe(
    req: SubscribeRequest,
    stream: StreamSink<crate::api::signaling::subscribe::PublishMessage>,
) -> CoreResult<()> {
    let mut server_stream = SignalingClientManager::get_client()
        .await?
        .subscribe(crate::proto::signaling::SubscribeRequest {
            device_id: req.local_device_id.to_owned(),
            device_finger_print: req.device_finger_print,
        })
        .await?
        .into_inner();

    TOKIO_RUNTIME.spawn(async move {
        defer! {
            stream.add(PublishMessage::StreamClosed);
            stream.close();
        }

        loop {
            let publish_message = match server_stream.message().await {
                Ok(message) => {
                    if let Some(message) = message {
                        message
                    } else {
                        tracing::error!("subscribe server stream was closed");
                        crate::api::signaling::disconnect::disconnect().await;
                        break;
                    }
                }
                Err(err) => {
                    tracing::error!("subscribe server stream received an error: {:?}", err);
                    continue;
                }
            };

            if let Some(inner_message) = publish_message.inner {
                match inner_message {
                    crate::proto::signaling::publish_message::Inner::VisitRequest(visit_request) => {
                        let resource_type = if visit_request.resource_type == 0 {
                            crate::api::signaling::visit::ResourceType::Desktop
                        } else if visit_request.resource_type ==1 {
                            crate::api::signaling::visit::ResourceType::Files
                        } else {
                            return;
                        };

                        let publish_message = PublishMessage::VisitRequest{
                            active_device_id: visit_request.active_device_id,
                            passive_device_id: visit_request.passive_device_id,
                            resource_type
                        };

                        if !stream.add(publish_message){
                            tracing::error!(device_id=?req.local_device_id, message_type=stringify!(PublishMessage::VisitRequest), "add message to stream failed");
                        }
                    }
                    crate::proto::signaling::publish_message::Inner::KeyExchangeRequest( key_exchange_request) => {
                        let config_path = req.config_path.clone();
                        TOKIO_RUNTIME.spawn(async move{
                            key_exchange::handle(&config_path, &key_exchange_request).await
                        });
                    }
                }
            }
        }
    });

    Ok(())
}
