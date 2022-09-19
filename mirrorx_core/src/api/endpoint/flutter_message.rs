use flutter_rust_bridge::ZeroCopyBuffer;

#[derive(Clone)]
pub enum FlutterMediaMessage {
    Video(i64, i64, ZeroCopyBuffer<Vec<u8>>),
    Audio(i64, i64, ZeroCopyBuffer<Vec<u8>>),
}
