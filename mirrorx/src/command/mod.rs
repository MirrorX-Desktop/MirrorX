mod generate_random_password;
mod get_current_domain;
mod init_config;
mod init_signaling;
mod set_current_domain_device_password;
mod signaling_key_exchange;
mod signaling_reply_visit_request;
mod signaling_visit_request;

use mirrorx_core::api::{config::entity::domain::Domain, signaling::SignalingProvider};
use tauri::async_runtime::Mutex;

pub use generate_random_password::*;
pub use get_current_domain::*;
pub use init_config::*;
pub use init_signaling::*;
pub use set_current_domain_device_password::*;
pub use signaling_key_exchange::*;
pub use signaling_reply_visit_request::*;
pub use signaling_visit_request::*;

pub struct UIState {
    domain: Mutex<Option<Domain>>,
    signaling_client: Mutex<Option<SignalingProvider>>,
}

impl UIState {
    pub fn new() -> Self {
        Self {
            domain: Mutex::new(None),
            signaling_client: Mutex::new(None),
        }
    }
}
