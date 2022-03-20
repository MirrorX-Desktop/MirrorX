mod desktop_connect_ask;

use log::error;

use self::desktop_connect_ask::handle_desktop_connect_ask_req;
use super::proto::{opcode::Opcode, DesktopConnectAskReq, ProtoMessage};

pub async fn process_handler(message: Box<dyn ProtoMessage>) -> Option<Box<dyn ProtoMessage>> {
    let opcode_enum = match Opcode::try_from(message.opcode()) {
        Ok(res) => res,
        Err(_) => return None,
    };

    match opcode_enum {
        // handle desktop connect request comes from remote machine
        Opcode::DesktopConnectAskReq => match message.downcast_ref::<DesktopConnectAskReq>() {
            Some(req) => handle_desktop_connect_ask_req(req).await,
            None => {
                error!("process_handler: downcast message to DesktopConnectAskReq failed");
                None
            }
        },
        _ => None,
    }
}
