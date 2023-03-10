use std::{fmt::Display, net::IpAddr};

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum EndPointID {
    DeviceID {
        local_device_id: i64,
        remote_device_id: i64,
    },
    IP {
        local_ip: IpAddr,
        remote_ip: IpAddr,
    },
}

impl Display for EndPointID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EndPointID::DeviceID {
                local_device_id,
                remote_device_id,
            } => {
                write!(
                    f,
                    "DeviceID (local:{local_device_id}, remote:{remote_device_id})"
                )
            }
            EndPointID::IP {
                local_ip,
                remote_ip,
            } => {
                write!(f, "IP (local:{local_ip}, remote:{remote_ip})")
            }
        }
    }
}
