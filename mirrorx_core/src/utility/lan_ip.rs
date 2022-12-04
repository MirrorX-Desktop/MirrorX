use crate::error::CoreResult;
use std::net::IpAddr;

pub async fn get_lan_ip() -> CoreResult<IpAddr> {
    let socket = tokio::net::UdpSocket::bind("0.0.0.0:0").await?;
    socket.connect("255.255.255.255:80").await?;
    let addr = socket.local_addr()?;
    Ok(addr.ip())
}
