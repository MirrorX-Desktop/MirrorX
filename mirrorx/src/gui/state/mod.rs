mod app_state;
mod app_state_updater;
mod event;

pub use app_state::AppState;
pub use app_state_updater::AppStateUpdater;

#[macro_export]
macro_rules! send_event {
    ($tx:expr, $event:expr) => {
        if let Err(err) = $tx.send($event) {
            tracing::error!("send event {:?} failed", err.0.as_ref());
        }
    };
}
