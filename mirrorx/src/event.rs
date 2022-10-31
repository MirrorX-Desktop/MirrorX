use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct PopupDialogVisitRequestEvent {
    pub active_device_id: String,
    pub passive_device_id: String,
    pub resource_type: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct PopupDialogInputRemotePasswordEvent {
    pub active_device_id: String,
    pub passive_device_id: String,
}
