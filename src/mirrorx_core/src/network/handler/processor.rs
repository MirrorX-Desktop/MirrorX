use futures::FutureExt;
use log::error;

use crate::network::handler::desktop_connect_ask::handle_desktop_connect_ask_req;
use crate::network::handler::heart_beat::handle_heart_beat_req;
use crate::network::proto::opcode::Opcode;
use crate::network::proto::{DesktopConnectAskReq, HeartBeatReq, ProtoMessage};
use crate::network::Client;
use std::sync::Arc;

pub async fn process_handler(
    transporter: &Client,
    message: Box<dyn ProtoMessage>,
) -> Option<Box<dyn ProtoMessage>> {
    let opcode_enum = match Opcode::try_from(message.opcode()) {
        Ok(res) => res,
        Err(_) => return None,
    };

    let resp_future = match opcode_enum {
        Opcode::HeartBeatReq => message
            .downcast_ref::<HeartBeatReq>()
            .map(|req| async move { handle_heart_beat_req(transporter, req).await }.boxed()),

        // handle desktop connect request comes from remote machine
        Opcode::DesktopConnectAskReq => message.downcast_ref::<DesktopConnectAskReq>().map(|req| {
            async move { handle_desktop_connect_ask_req(transporter, req).await }.boxed()
        }),
        _ => None,
    };

    if let Some(future) = resp_future {
        match future.await {
            Ok(res) => res,
            Err(err) => {
                error!("process_handler: handler returns error: {:?}", err);
                None
            }
        }
    } else {
        None
    }
}
