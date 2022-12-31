pub mod config;
pub mod file_manager;
pub mod lan;
pub mod signaling;
pub mod utility;

use dashmap::DashMap;
use mirrorx_core::{
    api::{
        config::LocalStorage,
        endpoint::{client::EndPointClient, id::EndPointID, message::EndPointDirectoryResponse},
        signaling::SignalingClient,
    },
    component::lan::{discover::Discover, server::Server},
};
use std::sync::Arc;
use tauri::async_runtime::Mutex;

pub struct AppState {
    storage: Mutex<Option<LocalStorage>>,
    signaling_client: Mutex<Option<(i64, SignalingClient)>>,
    lan_components: Mutex<Option<(Discover, Server)>>,
    files_endpoints: DashMap<
        String,
        (
            Arc<EndPointClient>,
            tokio::sync::mpsc::Receiver<EndPointDirectoryResponse>,
        ),
    >,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            storage: Mutex::new(None),
            signaling_client: Mutex::new(None),
            lan_components: Mutex::new(None),
            files_endpoints: DashMap::new(),
        }
    }
}
