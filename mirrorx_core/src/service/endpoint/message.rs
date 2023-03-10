use crate::component::{
    fs::Directory,
    screen::{display::Display, input::key::MouseKey},
};
use cpal::SampleFormat;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct EndPointHandshakeRequest {
    #[serde(with = "serde_bytes")]
    pub visit_credentials: Vec<u8>,
    pub device_id: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EndPointHandshakeResponse {
    pub remote_device_id: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum EndPointMessage {
    Error,
    CallRequest(u16, EndPointCallRequest),
    CallReply(u16, #[serde(with = "serde_bytes")] Vec<u8>), // Vec -> Result<T, String>
    VideoFrame(EndPointVideoFrame),
    AudioFrame(EndPointAudioFrame),
    InputCommand(EndPointInputEvent),
    FileTransferBlock(EndPointFileTransferBlock),
    FileTransferError(EndPointFileTransferError),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum EndPointCallRequest {
    NegotiateRequest(EndPointNegotiateRequest),
    SwitchScreenRequest(EndPointSwitchScreenRequest),
    VisitDirectoryRequest(EndPointVisitDirectoryRequest),
    SendFileRequest(EndPointSendFileRequest),
    DownloadFileRequest(EndPointDownloadFileRequest),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EndPointNegotiateRequest {
    pub video_codecs: Vec<VideoCodec>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EndPointNegotiateReply {
    pub video_codec: VideoCodec,
    pub os_type: String,
    pub os_version: String,
    pub displays: Vec<Display>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EndPointSwitchScreenRequest {
    pub display_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EndPointSwitchScreenReply {
    pub display_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
pub enum AudioSampleFormat {
    I8,
    U8,
    I16,
    U16,
    I32,
    U32,
    I64,
    U64,
    F32,
    F64,
}

impl From<AudioSampleFormat> for SampleFormat {
    fn from(value: AudioSampleFormat) -> Self {
        match value {
            AudioSampleFormat::I8 => SampleFormat::I8,
            AudioSampleFormat::U8 => SampleFormat::U8,
            AudioSampleFormat::I16 => SampleFormat::I16,
            AudioSampleFormat::U16 => SampleFormat::U16,
            AudioSampleFormat::I32 => SampleFormat::I32,
            AudioSampleFormat::U32 => SampleFormat::U32,
            AudioSampleFormat::I64 => SampleFormat::I64,
            AudioSampleFormat::U64 => SampleFormat::U64,
            AudioSampleFormat::F32 => SampleFormat::F32,
            AudioSampleFormat::F64 => SampleFormat::F64,
        }
    }
}

impl From<SampleFormat> for AudioSampleFormat {
    fn from(value: SampleFormat) -> Self {
        match value {
            SampleFormat::I8 => AudioSampleFormat::I8,
            SampleFormat::U8 => AudioSampleFormat::U8,
            SampleFormat::I16 => AudioSampleFormat::I16,
            SampleFormat::U16 => AudioSampleFormat::U16,
            SampleFormat::I32 => AudioSampleFormat::I32,
            SampleFormat::U32 => AudioSampleFormat::U32,
            SampleFormat::I64 => AudioSampleFormat::I64,
            SampleFormat::U64 => AudioSampleFormat::U64,
            SampleFormat::F32 => AudioSampleFormat::F32,
            SampleFormat::F64 => AudioSampleFormat::F64,
            _ => panic!("unsupported sample format, this should not be called"),
        }
    }
}

impl Default for AudioSampleFormat {
    fn default() -> Self {
        AudioSampleFormat::I16
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EndPointNegotiateFinishedRequest {
    // pub selected_monitor_id: String,
    pub expected_frame_rate: u8,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EndPointVideoFrame {
    pub width: i32,
    pub height: i32,
    pub pts: i64,

    #[serde(with = "serde_bytes")]
    pub buffer: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EndPointAudioFrame {
    pub channels: u8,
    pub sample_format: AudioSampleFormat,
    pub sample_rate: u32,
    #[serde(with = "serde_bytes")]
    pub buffer: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum MouseEvent {
    Up(MouseKey, f32, f32),
    Down(MouseKey, f32, f32),
    Move(MouseKey, f32, f32),
    ScrollWheel(f32),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum KeyboardEvent {
    KeyUp(tao::keyboard::KeyCode),
    KeyDown(tao::keyboard::KeyCode),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum InputEvent {
    Mouse(MouseEvent),
    Keyboard(KeyboardEvent),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EndPointInputEvent {
    pub events: Vec<InputEvent>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EndPointVisitDirectoryRequest {
    pub path: Option<PathBuf>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EndPointVisitDirectoryReply {
    pub dir: Directory,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EndPointSendFileRequest {
    pub id: String,
    pub filename: String,
    pub path: PathBuf,
    pub size: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EndPointSendFileReply {}

#[derive(Serialize, Deserialize, Debug)]
pub struct EndPointDownloadFileRequest {
    pub id: String,
    pub path: PathBuf,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EndPointDownloadFileReply {
    pub size: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EndPointFileTransferBlock {
    pub id: String,
    #[serde(with = "serde_bytes")]
    pub data: Option<Vec<u8>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EndPointFileTransferError {
    pub id: String,
}
