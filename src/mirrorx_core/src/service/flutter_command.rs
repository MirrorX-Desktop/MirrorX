use flutter_rust_bridge::StreamSink;
use lazy_static::lazy_static;

static mut INNER_FLUTTER_COMMAND_STREAM_SINK: Option<StreamSink<FlutterCommand>> = None;

lazy_static! {
    pub static ref FLUTTER_COMMAND_STREAM_SINK: &'static StreamSink<FlutterCommand> =
        unsafe { INNER_FLUTTER_COMMAND_STREAM_SINK.as_ref().unwrap() };
}

#[derive(Debug, Clone)]
pub enum FlutterCommand {
    PopupDesktopConnectInputPasswordDialog,
}

pub fn init_flutter_command_stream_sink(flutter_command_stream_sink: StreamSink<FlutterCommand>) {
    unsafe {
        INNER_FLUTTER_COMMAND_STREAM_SINK = Some(flutter_command_stream_sink);
    }
}
