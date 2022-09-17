use crate::{api::endpoint::RESERVE_STREAMS, error::CoreResult, utility::runtime::TOKIO_RUNTIME};
use std::time::Duration;
use tokio::net::TcpStream;
use tokio_util::codec::LengthDelimitedCodec;

pub struct ConnectRequest {
    pub local_device_id: i64,
    pub remote_device_id: i64,
    pub addr: String,
}

// pub struct ConnectResponse {}

pub async fn connect(req: ConnectRequest) -> CoreResult<()> {
    let stream =
        tokio::time::timeout(Duration::from_secs(10), TcpStream::connect(req.addr)).await??;

    stream.set_nodelay(true)?;

    let stream = LengthDelimitedCodec::builder()
        .little_endian()
        .max_frame_length(32 * 1024 * 1024)
        .new_framed(stream);

    RESERVE_STREAMS.insert((req.local_device_id, req.remote_device_id), stream);

    TOKIO_RUNTIME.spawn(async move {
        tokio::time::sleep(Duration::from_secs(60 * 2)).await;
        RESERVE_STREAMS.remove(&(req.local_device_id, req.remote_device_id))
    });

    Ok(())
}
