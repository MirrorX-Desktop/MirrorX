use std::{fmt::Display, net::IpAddr};

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum EndPointID {
    DeviceID { local: i64, remote: i64 },
    LANID { local: IpAddr, remote: IpAddr },
}

// impl Copy for EndPointID {}

impl Display for EndPointID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EndPointID::DeviceID { local, remote } => {
                write!(f, "DeviceID(local:{}, remote:{})", local, remote)
            }
            EndPointID::LANID { local, remote } => {
                write!(f, "LANID(local:{}, remote:{})", local, remote)
            }
        }
    }
}
