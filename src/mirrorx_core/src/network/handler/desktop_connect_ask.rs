use crate::network::proto::{self, ProtoMessage};

pub async fn handle_desktop_connect_ask_req(
    req: &proto::DesktopConnectAskReq,
) -> Option<Box<dyn ProtoMessage>> {
    None
}
