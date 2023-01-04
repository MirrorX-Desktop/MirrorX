use std::path::PathBuf;

use crate::component::{desktop::monitor::Monitor, fs::Directory, input::key::MouseKey};
use cpal::SampleFormat;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct EndPointHandshakeRequest {
    #[serde(with = "serde_bytes")]
    pub visit_credentials: Vec<u8>,
    pub device_id: i64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct EndPointHandshakeResponse {
    pub remote_device_id: i64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum EndPointMessage {
    Error,
    NegotiateDesktopParamsRequest(EndPointNegotiateDesktopParamsRequest),
    NegotiateDesktopParamsResponse(EndPointNegotiateDesktopParamsResponse),
    NegotiateFinishedRequest(EndPointNegotiateFinishedRequest),
    VideoFrame(EndPointVideoFrame),
    AudioFrame(EndPointAudioFrame),
    InputCommand(EndPointInput),
    DirectoryRequest(EndPointDirectoryRequest),
    DirectoryResponse(EndPointDirectoryResponse),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct EndPointNegotiateDesktopParamsRequest {
    pub video_codecs: Vec<VideoCodec>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct EndPointNegotiateVisitDesktopParams {
    pub video_codec: VideoCodec,
    pub os_type: String,
    pub os_version: String,
    pub primary_monitor: Monitor,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum EndPointNegotiateDesktopParamsResponse {
    VideoError(String),
    MonitorError(String),
    Params(EndPointNegotiateVisitDesktopParams),
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

#[allow(clippy::from_over_into)]
impl Into<SampleFormat> for AudioSampleFormat {
    fn into(self) -> SampleFormat {
        match self {
            AudioSampleFormat::I16 => SampleFormat::I16,
            AudioSampleFormat::U16 => SampleFormat::U16,
            AudioSampleFormat::F32 => SampleFormat::F32,
        }
    }
}

impl From<SampleFormat> for AudioSampleFormat {
    fn from(v: SampleFormat) -> Self {
        match v {
            SampleFormat::I16 => AudioSampleFormat::I16,
            SampleFormat::U16 => AudioSampleFormat::U16,
            SampleFormat::F32 => AudioSampleFormat::F32,
        }
    }
}

impl Default for AudioSampleFormat {
    fn default() -> Self {
        AudioSampleFormat::I16
    }
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
    pub channels: u8,
    pub sample_format: AudioSampleFormat,
    pub sample_rate: u32,
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
    KeyUp(tao::keyboard::KeyCode),
    KeyDown(tao::keyboard::KeyCode),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum InputEvent {
    Mouse(MouseEvent),
    Keyboard(KeyboardEvent),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct EndPointInput {
    pub events: Vec<InputEvent>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct EndPointDirectoryRequest {
    pub path: Option<PathBuf>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct EndPointDirectoryResponse {
    pub result: Result<Directory, String>,
}
