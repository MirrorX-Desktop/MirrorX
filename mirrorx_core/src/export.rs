use flutter_rust_bridge::StreamSink;

use crate::utility::runtime::TOKIO_RUNTIME;

macro_rules! async_block_on {
    ($future:expr) => {{
        let (tx, rx) = crossbeam::channel::bounded(1);

        TOKIO_RUNTIME.spawn(async move { tx.try_send($future.await) });

        let message = rx
            .recv()
            .map_err(|err| anyhow::anyhow!("receive call result failed ({})", err))??;

        Ok(message)
    }};
}

pub fn logger_init() -> anyhow::Result<()> {
    let _ = tracing_subscriber::fmt::try_init();
    Ok(())
}

pub fn config_read(
    path: String,
    key: String,
) -> anyhow::Result<Option<crate::api::config::ConfigProperties>> {
    let model = crate::api::config::read(&path, &key)?;
    Ok(model)
}

pub fn config_save(
    path: String,
    key: String,
    properties: crate::api::config::ConfigProperties,
) -> anyhow::Result<()> {
    crate::api::config::save(&path, &key, &properties)?;
    Ok(())
}

pub fn signaling_dial(req: crate::api::signaling::dial::DialRequest) -> anyhow::Result<()> {
    async_block_on! {
        crate::api::signaling::dial::dial(req)
    }
}

pub fn signaling_register(
    req: crate::api::signaling::register::RegisterRequest,
) -> anyhow::Result<crate::api::signaling::register::RegisterResponse> {
    async_block_on! {
        crate::api::signaling::register::register(req)
    }
}

pub fn signaling_subscribe(
    req: crate::api::signaling::subscribe::SubscribeRequest,
    stream: StreamSink<crate::api::signaling::subscribe::PublishMessage>,
) -> anyhow::Result<()> {
    async_block_on! {
        crate::api::signaling::subscribe::subscribe(req, stream)
    }
}

pub fn signaling_heartbeat(
    req: crate::api::signaling::heartbeat::HeartbeatRequest,
) -> anyhow::Result<crate::api::signaling::heartbeat::HeartbeatResponse> {
    async_block_on! {
        crate::api::signaling::heartbeat::heartbeat(req)
    }
}

pub fn signaling_visit(
    req: crate::api::signaling::visit::VisitRequest,
) -> anyhow::Result<crate::api::signaling::visit::VisitResponse> {
    async_block_on! {
        crate::api::signaling::visit::visit(req)
    }
}

pub fn signaling_key_exchange(
    req: crate::api::signaling::key_exchange::KeyExchangeRequest,
) -> anyhow::Result<crate::api::signaling::key_exchange::KeyExchangeResponse> {
    async_block_on! {
        crate::api::signaling::key_exchange::key_exchange(req)
    }
}

// pub fn endpoint_get_display_info(
//     remote_device_id: String,
// ) -> anyhow::Result<GetDisplayInfoResponse> {
//     async_block_on! {
//         api::endpoint::get_display_info(
//             remote_device_id
//         )
//     }
// }

// pub fn endpoint_start_media_transmission(
//     remote_device_id: String,
//     expect_fps: u8,
//     expect_display_id: String,
//     texture_id: i64,
//     video_texture_ptr: i64,
//     update_frame_callback_ptr: i64,
// ) -> anyhow::Result<StartMediaTransmissionResponse> {
//     async_block_on! {
//         api::endpoint::start_media_transmission(
//             remote_device_id,
//             expect_fps,
//             expect_display_id,
//             texture_id,
//             video_texture_ptr,
//             update_frame_callback_ptr,
//         )
//     }
// }

// pub fn endpoint_input(remote_device_id: String, event: InputEvent) -> anyhow::Result<()> {
//     async_block_on! {
//         api::endpoint::input(remote_device_id, event)
//     }
// }

// pub fn endpoint_manually_close(remote_device_id: String) -> anyhow::Result<()> {
//     api::endpoint::manually_close(remote_device_id).map_err(|err| anyhow::anyhow!(err))
// }

// pub fn endpoint_close_notify(
//     remote_device_id: String,
//     sink: flutter_rust_bridge::StreamSink<()>,
// ) -> anyhow::Result<()> {
//     let mut rx = api::endpoint::register_close_notificaton(remote_device_id)
//         .map_err(|err| anyhow::anyhow!(err))?;

//     TOKIO_RUNTIME.block_on(async move {
//         let _ = rx.recv().await;
//     });

//     sink.add(());
//     Ok(())
// }
