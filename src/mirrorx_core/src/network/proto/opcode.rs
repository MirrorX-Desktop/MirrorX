use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(TryFromPrimitive, IntoPrimitive, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum Opcode {
    HeartBeatReq = 1,
    HeartBeatResp = 2,
    DesktopConnectOfferReq = 3,
    DesktopConnectOfferResp = 4,
    DesktopConnectAskReq = 5,
    DesktopConnectAskResp = 6,
}
