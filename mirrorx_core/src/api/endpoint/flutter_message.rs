use crate::component::frame::DesktopDecodeFrame;
use flutter_rust_bridge::ZeroCopyBuffer;

pub enum FlutterMediaMessage {
    Video(ZeroCopyBuffer<Vec<u8>>),
    Audio(i64, i64, ZeroCopyBuffer<Vec<u8>>),
}
