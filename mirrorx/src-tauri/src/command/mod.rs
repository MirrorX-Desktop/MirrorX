pub mod config;
pub mod lan;
pub mod signaling;
pub mod utility;

use mirrorx_core::{
    api::{config::LocalStorage, signaling::SignalingClient},
    component::lan::{discover::Discover, server::Server},
};
use tauri::async_runtime::Mutex;

pub struct AppState {
    storage: Mutex<Option<LocalStorage>>,
    signaling_client: Mutex<Option<(i64, SignalingClient)>>,
    lan_components: Mutex<Option<(Discover, Server)>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            storage: Mutex::new(None),
            signaling_client: Mutex::new(None),
            lan_components: Mutex::new(None),
        }
    }
}
