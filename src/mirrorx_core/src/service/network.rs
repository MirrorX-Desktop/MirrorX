use lazy_static::lazy_static;
use log::{error, warn};
use rustls::Certificate;
use std::{sync::Arc, time::Duration};
use tokio::{net::TcpStream, time::interval};
use tokio_rustls::{rustls::ClientConfig, TlsConnector};

use crate::{
    api_error::APIError,
    network::{
        message::{
            DesktopConnectOfferReq, DeviceGoesOnlineReq, HeartBeatReq, Message, MessageError,
        },
        Client,
    },
    service::runtime::RUNTIME,
};

static mut INNER_CLIENT: Option<Arc<Client>> = None;

static CERT_FILE: &'static str = r"
-----BEGIN CERTIFICATE-----
MIIBfDCCAS6gAwIBAgIUfrdyHYzwLTVCCObY6Y+oEJ/A2eswBQYDK2VwMCcxCzAJ
BgNVBAYTAkRFMRgwFgYDVQQDDA93d3cuZXhhbXBsZS5jb20wHhcNMjIwMzI0MDk0
MDQ2WhcNMjQwMjIyMDk0MDQ2WjAnMQswCQYDVQQGEwJERTEYMBYGA1UEAwwPd3d3
LmV4YW1wbGUuY29tMCowBQYDK2VwAyEAuteBOGMlj78x5HZC3z2pRvVLJ4g9jxDk
pA6KBQdyxUejbDBqMAsGA1UdDwQEAwIEMDATBgNVHSUEDDAKBggrBgEFBQcDATAn
BgNVHREEIDAegg93d3cuZXhhbXBsZS5jb22CC2V4YW1wbGUuY29tMB0GA1UdDgQW
BBTWqWCZznUXkrB6OI4FZVrfQ/FGBTAFBgMrZXADQQB5VasnuyJqfto2CgATZ3G3
o4VdZBLOrKi4Bu6ltMDfG1BCwKbfGp68pK6+7qKaEq0fFUaL+qIz3D0hn9zK0rUE
-----END CERTIFICATE-----
";

lazy_static! {
    static ref CLIENT: &'static Client = unsafe { INNER_CLIENT.as_ref().unwrap() };
}

pub fn init_client() -> anyhow::Result<(), APIError> {
    let client = RUNTIME.block_on(async {
        new_client(String::from("192.168.0.101:45555"))
            .await
            .map_err(|err| {
                error!("init client error: {}", err);
                APIError::InternalError
            })
    })?;

    unsafe {
        INNER_CLIENT = Some(client);
    }

    begin_heart_beat();

    Ok(())
}

pub async fn new_client(addr: String) -> anyhow::Result<Arc<Client>> {
    let mut root_cert_store = rustls::RootCertStore::empty();
    let mut rd = CERT_FILE.as_bytes();
    let certs = rustls_pemfile::certs(&mut rd)?;
    for ele in certs {
        root_cert_store.add(&Certificate(ele))?;
    }

    let config = ClientConfig::builder()
        .with_safe_defaults()
        .with_root_certificates(root_cert_store)
        .with_no_client_auth();

    let connector = TlsConnector::from(Arc::new(config));

    let stream = TcpStream::connect(&addr).await?;

    let domain = rustls::ServerName::try_from("www.example.com")
        .map_err(|_| anyhow::anyhow!("invalid dns name"))?;

    let stream = connector.connect(domain, stream).await?;

    Ok(Client::new(stream).await?)
}

fn begin_heart_beat() {
    RUNTIME.spawn(async move {
        let mut heart_beat_miss_counter = 0;
        let mut ticker = interval(Duration::from_secs(20));

        loop {
            if heart_beat_miss_counter >= 3 {
                error!("heart_beat: missed times >=3, break");
                break;
            }

            ticker.tick().await;

            let time_stamp_now = chrono::Utc::now().timestamp() as u32;

            let resp = CLIENT
                .call(
                    Message::HeartBeatReq(HeartBeatReq {
                        time_stamp: time_stamp_now,
                    }),
                    Duration::from_secs(5),
                )
                .await;

            let resp = match resp {
                Ok(Message::HeartBeatResp(message)) => message,
                Ok(_) => {
                    heart_beat_miss_counter += 1;
                    error!("heart_beat: mismatched response message type");
                    continue;
                }
                Err(err) => {
                    match err {
                        MessageError::Timeout=> error!("heart_beat: call timeout"),
                        _ => error!("heart_beat: request failed: {:?}", err),
                    };

                    heart_beat_miss_counter += 1;
                    continue;
                }
            };

            if resp.time_stamp > time_stamp_now + 5 {
                warn!("heart_beat: response received before deadline but inner timestamp is greater than deadline")
            }
        }

        // todo: should close client
    });
}

pub fn device_goes_online() -> anyhow::Result<(), APIError> {
    RUNTIME.block_on(async move {
        let device_id = super::config::read_device_id()?;

        let resp = CLIENT
            .call(
                Message::DeviceGoesOnlineReq(DeviceGoesOnlineReq { device_id }),
                Duration::from_secs(10),
            )
            .await
            .map_err(|err| map_message_error(err))?;

        let resp_message = match resp {
            Message::DeviceGoesOnlineResp(message) => message,
            _ => return Err(APIError::InternalError),
        };

        super::config::save_device_id(&resp_message.device_id)?;
        super::config::save_device_id_expire_at(&resp_message.device_id_expire_time_stamp)
    })
}

pub fn desktop_connect_offer(ask_device_id: String) -> anyhow::Result<bool, APIError> {
    RUNTIME.block_on(async move {
        let offer_device_id = match super::config::read_device_id()? {
            Some(device_id) => device_id,
            None => {
                error!("device_id is None");
                return Err(APIError::ConfigError);
            }
        };

        let resp = CLIENT
            .call(
                Message::DesktopConnectOfferReq(DesktopConnectOfferReq {
                    offer_device_id,
                    ask_device_id,
                }),
                Duration::from_secs(15),
            )
            .await
            .map_err(|err| map_message_error(err))?;

        let resp_message = match resp {
            Message::DesktopConnectOfferResp(message) => message,
            _ => return Err(APIError::InternalError),
        };

        Ok(resp_message.agree)
    })
}

fn map_message_error(message_error: MessageError) -> APIError {
    match message_error {
        MessageError::InternalError | MessageError::MismatchedResponseMessage => {
            APIError::InternalError
        }
        MessageError::Timeout => APIError::Timeout,
        MessageError::InvalidArguments => APIError::InvalidArguments,
        MessageError::RemoteClientOfflineOrNotExist => APIError::RemoteClientOfflineOrNotExist,
    }
}
