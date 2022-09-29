use crate::component::input::key::{KeyboardKey, MouseKey};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct EndPointHandshakeRequest {
    pub visit_credentials: String,
    pub device_id: i64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct EndPointHandshakeResponse {
    pub remote_device_id: i64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum EndPointMessage {
    Error,
    NegotiateVisitDesktopParamsRequest(EndPointNegotiateVisitDesktopParamsRequest),
    NegotiateVisitDesktopParamsResponse(EndPointNegotiateVisitDesktopParamsResponse),
    NegotiateSelectMonitorRequest(EndPointNegotiateSelectMonitorRequest),
    NegotiateSelectMonitorResponse(EndPointNegotiateSelectMonitorResponse),
    NegotiateFinishedRequest(EndPointNegotiateFinishedRequest),
    VideoFrame(EndPointVideoFrame),
    AudioFrame(EndPointAudioFrame),
    Input(EndPointInput),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum VideoCodec {
    H264,
    Hevc,
    VP8,
    VP9,
}

impl Default for VideoCodec {
    fn default() -> Self {
        VideoCodec::H264
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum AudioSampleFormat {
    I16,
    U16,
    F32,
}

impl Default for AudioSampleFormat {
    fn default() -> Self {
        AudioSampleFormat::I16
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct EndPointNegotiateVisitDesktopParamsRequest {
    pub video_codecs: Vec<VideoCodec>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum EndPointNegotiateVisitDesktopParamsResponse {
    Error,
    Params(EndPointNegotiateVisitDesktopParams),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Default)]
pub struct EndPointNegotiateVisitDesktopParams {
    pub video_codec: VideoCodec,
    pub audio_sample_rate: u32,
    pub audio_sample_format: AudioSampleFormat,
    pub audio_channels: u8,
    pub os_type: String,
    pub os_version: String,
    pub monitor_id: String,
    pub monitor_width: u16,
    pub monitor_height: u16,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct EndPointNegotiateSelectMonitorRequest {}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct MonitorDescription {
    pub id: String,
    pub name: String,
    pub frame_rate: u8,
    pub width: u16,
    pub height: u16,
    pub is_primary: bool,
    #[serde(with = "serde_bytes")]
    pub screen_shot: Option<Vec<u8>>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct EndPointNegotiateSelectMonitorResponse {
    pub monitor_descriptions: Vec<MonitorDescription>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct EndPointNegotiateFinishedRequest {
    // pub selected_monitor_id: String,
    pub expected_frame_rate: u8,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct EndPointVideoFrame {
    pub width: i32,
    pub height: i32,
    pub pts: i64,

    #[serde(with = "serde_bytes")]
    pub buffer: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct EndPointAudioFrame {
    pub params: Option<(u32, AudioSampleFormat, u8, u16)>, // sample_rate, sample_format, channels, frame_size
    #[serde(with = "serde_bytes")]
    pub buffer: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum MouseEvent {
    Up(MouseKey, f32, f32),
    Down(MouseKey, f32, f32),
    Move(MouseKey, f32, f32),
    ScrollWheel(f32),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
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
pub struct EndPointInput {
    pub event: InputEvent,
}
