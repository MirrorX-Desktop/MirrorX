use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct VisitRequest {
    pub active_device_id: String,
    pub passive_device_id: String,
    pub resource_type: String,
}
