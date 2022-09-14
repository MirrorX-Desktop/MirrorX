mod key_exchange;

use super::SignalingClientManager;
use crate::{error::CoreResult, utility::runtime::TOKIO_RUNTIME};
use flutter_rust_bridge::StreamSink;
use scopeguard::defer;

pub enum PublishMessage {
    StreamClosed,
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
    req: SubscribeRequest,
    stream: StreamSink<PublishMessage>,
) -> CoreResult<()> {
    let mut server_stream = SignalingClientManager::get_client()
        .await?
        .subscribe(signaling_proto::message::SubscribeRequest {
            device_id: req.local_device_id,
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

                        if !stream.add(publish_message){
                            tracing::error!(device_id=?req.local_device_id, message_type=stringify!(PublishMessage::VisitRequest), "add message to stream failed");
                        }
                    }
                    signaling_proto::message::publish_message::InnerPublishMessage::KeyExchangeRequest( key_exchange_request) => {
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
