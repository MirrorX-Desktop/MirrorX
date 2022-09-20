use crate::component::frame::DesktopDecodeFrame;
use flutter_rust_bridge::ZeroCopyBuffer;

#[derive(Clone)]
pub enum FlutterMediaMessage {
    Video(DesktopDecodeFrame),
    Audio(i64, i64, ZeroCopyBuffer<Vec<u8>>),
}
