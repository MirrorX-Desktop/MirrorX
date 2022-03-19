use super::{opcode::Opcode, ProtoMessage};
use log::error;

pub fn create_proto_message(opcode: u16) -> Option<Box<dyn ProtoMessage>> {
    let opcode_enum = match Opcode::try_from(opcode) {
        Ok(res) => res,
        Err(_) => {
            error!("unknown proto message opcode: {}", opcode);
            return None;
        }
    };

    Some(match opcode_enum {
        Opcode::HeartBeatReq => Box::new(super::HeartBeatReq::default()),
        Opcode::HeartBeatResp => Box::new(super::HeartBeatResp::default()),
        Opcode::DesktopConnectOfferReq => Box::new(super::DesktopConnectOfferReq::default()),
        Opcode::DesktopConnectOfferResp => Box::new(super::DesktopConnectOfferResp::default()),
        Opcode::DesktopConnectAskReq => Box::new(super::DesktopConnectAskReq::default()),
        Opcode::DesktopConnectAskResp => Box::new(super::DesktopConnectAskResp::default()),
    })
}
