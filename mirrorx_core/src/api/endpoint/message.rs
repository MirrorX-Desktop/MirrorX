use crate::component::input::key::{KeyboardKey, MouseKey};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct EndPointHandshakeRequest {
    pub visit_credentials: String,
    pub device_id: i64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
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
    // NegotiateFinishedResponse(EndPointNegotiateFinishedResponse),
    VideoFrame(EndPointVideoFrame),
    AudioFrame(EndPointAudioFrame),
    Input(EndPointInput),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum VideoCodec {
    H264,
    HEVC,
    VP8,
    VP9,
}

impl Default for VideoCodec {
    fn default() -> Self {
        return VideoCodec::H264;
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum AudioSampleRate {
    HZ8000,
    HZ12000,
    HZ160000,
    HZ240000,
    HZ480000,
}

impl Default for AudioSampleRate {
    fn default() -> Self {
        return AudioSampleRate::HZ8000;
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum AudioSampleFormat {
    I16,
    U16,
    F32,
}

impl Default for AudioSampleFormat {
    fn default() -> Self {
        return AudioSampleFormat::I16;
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct EndPointNegotiateVisitDesktopParamsRequest {
    pub video_codecs: Vec<VideoCodec>,
    pub audio_max_sample_rate: AudioSampleRate,
    pub audio_sample_formats: Vec<AudioSampleFormat>,
    pub audio_dual_channel: bool,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum EndPointNegotiateVisitDesktopParamsResponse {
    Error,
    Params(EndPointNegotiateVisitDesktopParams),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct EndPointNegotiateVisitDesktopParams {
    pub video_codec: VideoCodec,
    pub audio_sample_rate: AudioSampleRate,
    pub audio_sample_format: AudioSampleFormat,
    pub audio_dual_channel: bool,
    pub os_type: String,
    pub os_version: String,
    pub monitor_id: String,
    pub monitor_width: u16,
    pub monitor_height: u16,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct EndPointNegotiateSelectMonitorRequest {}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
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

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct EndPointNegotiateSelectMonitorResponse {
    pub monitor_descriptions: Vec<MonitorDescription>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct EndPointNegotiateFinishedRequest {
    // pub selected_monitor_id: String,
    pub expected_frame_rate: u8,
}

// #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
// pub struct EndPointNegotiateFinishedResponse {}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct EndPointVideoFrame {
    #[serde(with = "serde_bytes")]
    pub sps: Option<Vec<u8>>,

    #[serde(with = "serde_bytes")]
    pub pps: Option<Vec<u8>>,

    #[serde(with = "serde_bytes")]
    pub buffer: Vec<u8>,
}

// unsafe impl Send for EndPointVideoFrame {}
// unsafe impl Sync for EndPointVideoFrame {}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct EndPointAudioFrame {
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
pub struct EndPointInput {
    pub event: InputEvent,
}
