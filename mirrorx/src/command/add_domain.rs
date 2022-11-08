use mirrorx_core::{
    api::{
        config::{entity::domain::Domain, LocalStorage},
        signaling::RegisterRequest,
    },
    core_error,
    error::CoreResult,
    signaling_proto::message::GetDomainRequest,
};
use std::net::SocketAddr;
use tauri::http::Uri;

#[tauri::command]
#[tracing::instrument]
pub async fn add_domain(addr: String, remarks: String) -> CoreResult<()> {
    let uri = addr
        .parse::<SocketAddr>()
        .map(|addr| {
            Uri::builder()
                .scheme("tcp")
                .authority(addr.to_string())
                .path_and_query("")
                .build()
                .map_err(|_| core_error!("invalid addr format"))
        })
        .unwrap_or_else(|_| Uri::try_from(addr).map_err(|_| core_error!("invalid uri format")))?;

    let mut client = mirrorx_core::api::signaling::dial(uri.clone()).await?;

    let get_domain_response = client.get_domain(GetDomainRequest {}).await?;
    let get_domain_response = get_domain_response.into_inner();

    let storage = LocalStorage::current()?;
    if storage.domain().domain_exist(&get_domain_response.domain)? {
        return Err(core_error!("domain is exists"));
    }

    let finger_print = mirrorx_core::utility::rand::generate_device_finger_print();
    let register_response = client
        .register(RegisterRequest {
            device_id: 0,
            device_finger_print: finger_print.to_owned(),
        })
        .await?;

    let register_response = register_response.into_inner();

    storage.domain().add_domain(Domain {
        id: 0,
        name: get_domain_response.domain,
        addr: uri.to_string(),
        is_primary: false,
        device_id: register_response.device_id,
        password: mirrorx_core::utility::rand::generate_random_password(),
        finger_print,
        remarks,
    })?;

    Ok(())
}
