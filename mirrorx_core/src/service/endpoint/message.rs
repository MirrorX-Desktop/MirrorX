use serde::{Deserialize, Serialize};

use crate::component::input::key::{KeyboardKey, MouseKey};

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
    VideoFrame(VideoFrame),
    AudioFrame(AudioFrame),
    Input(Input),
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
    pub id: String,
    pub name: String,
    pub refresh_rate: u8,
    pub width: u16,
    pub height: u16,
    pub is_primary: bool,
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
    pub expect_display_id: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct StartMediaTransmissionResponse {
    pub os_type: crate::constants::os::OperatingSystemType,
    pub os_version: String,
    pub screen_width: u16,
    pub screen_height: u16,
    pub video_type: String,
    pub audio_type: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct VideoFrame {
    #[serde(with = "serde_bytes")]
    pub sps: Option<Vec<u8>>,

    #[serde(with = "serde_bytes")]
    pub pps: Option<Vec<u8>>,

    #[serde(with = "serde_bytes")]
    pub buffer: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct AudioFrame {
    #[serde(with = "serde_bytes")]
    pub buffer: Vec<u8>,
    pub frame_size_per_channel: u16,
    pub elapsed: u128,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum MouseEvent {
    MouseUp(MouseKey, f32, f32),
    MouseDown(MouseKey, f32, f32),
    MouseMove(MouseKey, f32, f32),
    MouseScrollWheel(f32),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum KeyboardEvent {
    KeyUp(KeyboardKey),
    KeyDown(KeyboardKey),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum InputEvent {
    Mouse(MouseEvent),
    Keyboard(KeyboardEvent),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Input {
    pub event: InputEvent,
}
