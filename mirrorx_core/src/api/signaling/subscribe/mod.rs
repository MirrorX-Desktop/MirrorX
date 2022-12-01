mod key_exchange;

use scopeguard::defer;
use serde::Serialize;
use signaling_proto::message::{publish_message::InnerPublishMessage, ResourceType};
use signaling_proto::service::signaling_client::SignalingClient;
use std::time::Duration;
use tokio::{
    select,
    sync::mpsc::{error::TryRecvError, Receiver, Sender},
};
use tonic::transport::Channel;

#[derive(Debug, Clone, Serialize)]
pub enum PublishMessage {
    VisitRequest {
        active_device_id: i64,
        passive_device_id: i64,
        resource_type: crate::api::signaling::visit::ResourceType,
    },
}

pub async fn subscribe(
    client: &mut SignalingClient<Channel>,
    domain_id: i64,
    device_id: i64,
    device_finger_print: String,
    publish_message_tx: Sender<PublishMessage>,
    mut exit_tx: Receiver<()>,
) {
    let mut subscribe_client = client.clone();
    tokio::spawn(async move {
        defer! {
            tracing::info!("subscribe exit");
        }

        loop {
            match exit_tx.try_recv() {
                Ok(_) => return,
                Err(err) => {
                    if let TryRecvError::Disconnected = err {
                        return;
                    }
                }
            }

            let mut server_stream = match subscribe_client
                .subscribe(signaling_proto::message::SubscribeRequest {
                    device_id,
                    device_finger_print: device_finger_print.clone(),
                })
                .await
            {
                Ok(stream) => stream.into_inner(),
                Err(err) => {
                    tracing::error!(?err, "subscribe stream failed, try again after 1s");
                    tokio::time::sleep(Duration::from_secs(1)).await;
                    continue;
                }
            };

            loop {
                let publish_message = select! {
                    biased;
                    _ = exit_tx.recv() => return,
                    publish_message = server_stream.message() => publish_message,
                };

                let message = match publish_message {
                    Ok(message) => {
                        if let Some(message) = message {
                            match message.inner_publish_message {
                                Some(message) => message,
                                None => {
                                    tracing::warn!("publish message inner is None");
                                    continue;
                                }
                            }
                        } else {
                            tracing::error!(
                                "subscribe server stream disconnect, reconnect after 1s"
                            );
                            tokio::time::sleep(Duration::from_secs(1)).await;
                            break;
                        }
                    }
                    Err(err) => {
                        tracing::error!(?err, "subscribe server stream received an error");
                        continue;
                    }
                };

                match message {
                    InnerPublishMessage::VisitRequest(visit_request) => {
                        let resource_type =
                            match ResourceType::from_i32(visit_request.resource_type) {
                                Some(typ) => match typ {
                                    ResourceType::Desktop => {
                                        crate::api::signaling::visit::ResourceType::Desktop
                                    }
                                    ResourceType::Files => {
                                        crate::api::signaling::visit::ResourceType::Files
                                    }
                                },
                                None => {
                                    tracing::warn!("remote device require unknown resource type");
                                    continue;
                                }
                            };

                        let publish_message = PublishMessage::VisitRequest {
                            active_device_id: visit_request.active_device_id,
                            passive_device_id: visit_request.passive_device_id,
                            resource_type,
                        };

                        if let Err(err) = publish_message_tx.send(publish_message).await {
                            tracing::error!(?err, "publish message channel send failed");
                        }
                    }
                    InnerPublishMessage::KeyExchangeRequest(key_exchange_request) => {
                        let mut client = subscribe_client.clone();

                        tokio::spawn(async move {
                            key_exchange::handle(&mut client, domain_id, &key_exchange_request)
                                .await;
                        });
                    }
                }
            }
        }
    });
}
