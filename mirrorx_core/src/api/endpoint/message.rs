use crate::component::input::key::{KeyboardKey, MouseKey};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct EndPointHandshakeRequest {
    pub active_device_id: String,
    pub passive_device_id: String,
    pub visit_credentials: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct EndPointHandshakeResponse {}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum EndPointMessage {
    Error(Error),
    NegotiateVisitDesktopParamsRequest(NegotiateVisitDesktopParamsRequest),
    NegotiateVisitDesktopParamsResponse(NegotiateVisitDesktopParamsResponse),
    NegotiateSelectMonitorRequest(NegotiateSelectMonitorRequest),
    NegotiateSelectMonitorResponse(NegotiateSelectMonitorResponse),
    NegotiateFinishedRequest(NegotiateFinishedRequest),
    NegotiateFinishedResponse(NegotiateFinishedResponse),
    VideoFrame(VideoFrame),
    AudioFrame(AudioFrame),
    Input(Input),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum Error {}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum VideoCodec {
    H264,
    HEVC,
    VP8,
    VP9,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum AudioSampleRate {
    HZ8000,
    HZ12000,
    HZ160000,
    HZ240000,
    HZ480000,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum AudioSampleFormat {
    I16,
    U16,
    F32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct NegotiateVisitDesktopParamsRequest {
    pub video_codecs: Vec<VideoCodec>,
    pub audio_max_sample_rate: AudioSampleRate,
    pub audio_sample_formats: Vec<AudioSampleFormat>,
    pub audio_dual_channel: bool,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct NegotiateVisitDesktopParamsResponse {
    pub video_codec: VideoCodec,
    pub audio_sample_rate: AudioSampleRate,
    pub audio_sample_format: AudioSampleFormat,
    pub audio_dual_channel: bool,
    pub os_type: String,
    pub os_version: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct NegotiateSelectMonitorRequest {}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct MonitorDescription {
    pub id: String,
    pub name: String,
    pub frame_rate: u8,
    pub width: u16,
    pub height: u16,
    pub is_primary: bool,
    #[serde(with = "serde_bytes")]
    pub screen_shot: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct NegotiateSelectMonitorResponse {
    pub monitor_descriptions: Vec<MonitorDescription>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct NegotiateFinishedRequest {
    pub selected_monitor_id: String,
    pub expected_frame_rate: u8,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct NegotiateFinishedResponse {}

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
