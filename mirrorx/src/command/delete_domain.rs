use std::net::{SocketAddr, ToSocketAddrs};

use mirrorx_core::{
    api::config::{entity::domain::Domain, LocalStorage},
    core_error,
    error::CoreResult,
    signaling_proto::message::GetDomainRequest,
};
use tauri::http::Uri;

#[tauri::command]
#[tracing::instrument]
pub async fn delete_domain(id: i64) -> CoreResult<()> {
    let storage = LocalStorage::current()?;
    storage.domain().delete_domain(id)
}
