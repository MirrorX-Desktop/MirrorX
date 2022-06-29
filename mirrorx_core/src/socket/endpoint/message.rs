use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum EndPointMessagePacketType {
    Request,
    Response,
    Push,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct EndPointMessagePacket {
    pub typ: EndPointMessagePacketType,
    pub call_id: Option<u16>,
    pub message: EndPointMessage,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum EndPointMessage {
    Error,
    StartMediaTransmissionRequest(StartMediaTransmissionRequest),
    StartMediaTransmissionResponse(StartMediaTransmissionResponse),
    GetDisplayInfoRequest(GetDisplayInfoRequest),
    GetDisplayInfoResponse(GetDisplayInfoResponse),
    MediaFrame(MediaFrame),
}
//
// #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
// pub enum EndPointMessageError {
//     Mismatched,
//     Internal,
// }

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct GetDisplayInfoRequest {}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct DisplayInfo {
    pub id: u32,
    pub is_main: bool,
    #[serde(with = "serde_bytes")]
    pub screen_shot: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct GetDisplayInfoResponse {
    pub displays: Vec<DisplayInfo>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct StartMediaTransmissionRequest {
    pub expect_fps: u8,
    pub expect_display_id: u32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct StartMediaTransmissionResponse {
    pub os_name: String,
    pub os_version: String,
    pub screen_width: u16,
    pub screen_height: u16,
    pub video_type: String,
    pub audio_type: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct MediaFrame {
    #[serde(with = "serde_bytes")]
    pub data: Vec<u8>,
    pub timestamp: u64,
}
