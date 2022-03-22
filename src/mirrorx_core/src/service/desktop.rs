use lazy_static::lazy_static;
use std::{sync::Arc, time::Duration};
use tokio::net::TcpStream;

use crate::{network::Client, service::runtime::RUNTIME};

// static mut INNER_CLIENT: Option<Arc<Client>> = None;

// lazy_static! {
//     static ref CLIENT: &'static Client = unsafe { INNER_CLIENT.as_ref().unwrap() };
// }

pub fn init_client() -> anyhow::Result<()> {
    // let client = RUNTIME.block_on(async {
    //     let stream = TcpStream::connect("127.0.0.1:45555").await?;
    //     Client::new(stream).await
    // })?;

    // unsafe {
    //     INNER_CLIENT = Some(client);
    // }

    Ok(())
}

pub fn connect_to(device_id: String) -> anyhow::Result<bool> {
    // RUNTIME.block_on(async move {
    //     let resp: DesktopConnectOfferResp = CLIENT
    //         .call(
    //             &DesktopConnectOfferReq {
    //                 device_id: String::from("523344551"),
    //             },
    //             Duration::from_secs(1),
    //         )
    //         .await?;

    //     Ok(resp.allow)
    // })

    Ok(true)
}
