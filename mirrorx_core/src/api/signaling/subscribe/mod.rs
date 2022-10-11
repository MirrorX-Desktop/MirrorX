mod key_exchange;

use crate::{error::CoreResult, utility::runtime::TOKIO_RUNTIME};
use std::path::PathBuf;
use tonic::transport::Channel;

pub enum PublishMessage {
    VisitRequest {
        active_device_id: i64,
        passive_device_id: i64,
        resource_type: crate::api::signaling::visit::ResourceType,
    },
}

pub struct SubscribeRequest {
    pub local_device_id: i64,
    pub device_finger_print: String,
    pub config_path: String,
}

pub async fn subscribe(
    client: &mut signaling_proto::service::signaling_client::SignalingClient<Channel>,
    req: SubscribeRequest,
    publish_message_tx: crossbeam::channel::Sender<PublishMessage>,
) -> CoreResult<()> {
    let mut server_stream = client
        .subscribe(signaling_proto::message::SubscribeRequest {
            device_id: req.local_device_id,
            device_finger_print: req.device_finger_print,
        })
        .await?
        .into_inner();

    let subscribe_client = client.clone();

    TOKIO_RUNTIME.spawn(async move {
        loop {
            let publish_message = match server_stream.message().await {
                Ok(message) => {
                    if let Some(message) = message {
                        message
                    } else {
                        tracing::error!("subscribe server stream was closed");
                        // let _ = crate::api::signaling::disconnect::disconnect().await;
                        break;
                    }
                }
                Err(err) => {
                    tracing::error!("subscribe server stream received an error: {:?}", err);
                    continue;
                }
            };

            if let Some(inner_message) = publish_message.inner_publish_message {
                match inner_message {
                    signaling_proto::message::publish_message::InnerPublishMessage::VisitRequest(visit_request) => {
                        let resource_type =  match signaling_proto::message::ResourceType::from_i32(visit_request.resource_type){
                            Some(typ) => match typ{
                                signaling_proto::message::ResourceType::Desktop => crate::api::signaling::visit::ResourceType::Desktop,
                                signaling_proto::message::ResourceType::Files => crate::api::signaling::visit::ResourceType::Files,
                            },
                            None => {
                                tracing::warn!(resource_type= visit_request.resource_type, "remote device require unknown resource type, ignore this request");
                                continue;
                            },
                        };
                        let publish_message = PublishMessage::VisitRequest{
                            active_device_id: visit_request.active_device_id,
                            passive_device_id: visit_request.passive_device_id,
                            resource_type,
                        };

                        let _ = publish_message_tx.try_send(publish_message);
                    }
                    signaling_proto::message::publish_message::InnerPublishMessage::KeyExchangeRequest( key_exchange_request) => {
                        let mut client = subscribe_client.clone();
                        let config_path = PathBuf::from(req.config_path.clone());
                        TOKIO_RUNTIME.spawn(async move {
                            key_exchange::handle(&mut client,&config_path, &key_exchange_request).await
                        });
                    }
                }
            }
        }
    });

    Ok(())
}
