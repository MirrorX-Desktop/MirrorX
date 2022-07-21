use crate::{
    error::MirrorXError,
    service::endpoint::message::{
        GetDisplayInfoRequest, GetDisplayInfoResponse, Input, InputEvent, MouseEvent,
        StartMediaTransmissionRequest, StartMediaTransmissionResponse,
    },
    utility::nonce_value::NonceValue,
};
use ring::aead::{OpeningKey, SealingKey};

pub async fn connect(
    is_active_side: bool,
    local_device_id: String,
    remote_device_id: String,
    sealing_key: SealingKey<NonceValue>,
    opening_key: OpeningKey<NonceValue>,
) -> Result<(), MirrorXError> {
    crate::service::endpoint::connect(
        "192.168.0.101:28001",
        is_active_side,
        local_device_id,
        remote_device_id.clone(),
        opening_key,
        sealing_key,
    )
    .await
}

pub async fn get_display_info(
    remote_device_id: String,
) -> Result<GetDisplayInfoResponse, MirrorXError> {
    let endpoint = match crate::service::endpoint::ENDPOINTS.get(&remote_device_id) {
        Some(pair) => pair,
        None => return Err(MirrorXError::EndPointNotFound(remote_device_id)),
    };

    endpoint.get_display_info(GetDisplayInfoRequest {}).await
}

pub async fn start_media_transmission(
    remote_device_id: String,
    expect_fps: u8,
    expect_display_id: String,
    texture_id: i64,
    video_texture_ptr: i64,
    update_frame_callback_ptr: i64,
) -> Result<StartMediaTransmissionResponse, MirrorXError> {
    let endpoint = match crate::service::endpoint::ENDPOINTS.get(&remote_device_id) {
        Some(pair) => pair,
        None => return Err(MirrorXError::EndPointNotFound(remote_device_id)),
    };

    let resp = endpoint
        .start_media_transmission(StartMediaTransmissionRequest {
            expect_fps,
            expect_display_id,
        })
        .await?;

    endpoint
        .start_video_render(
            resp.screen_width as i32,
            resp.screen_height as i32,
            expect_fps as i32,
            texture_id,
            video_texture_ptr,
            update_frame_callback_ptr,
        )
        .await?;

    // endpoint.start_audio_play().await?;

    Ok(resp)
}

pub async fn input(remote_device_id: String, event: InputEvent) -> Result<(), MirrorXError> {
    let endpoint = match crate::service::endpoint::ENDPOINTS.get(&remote_device_id) {
        Some(pair) => pair,
        None => return Err(MirrorXError::EndPointNotFound(remote_device_id)),
    };

    endpoint.trigger_input(Input { event }).await
}

pub fn register_close_notificaton(
    remote_device_id: String,
) -> Result<crossbeam::channel::Receiver<()>, MirrorXError> {
    let endpoint = match crate::service::endpoint::ENDPOINTS.get(&remote_device_id) {
        Some(pair) => pair,
        None => return Err(MirrorXError::EndPointNotFound(remote_device_id)),
    };

    Ok(endpoint.subscribe_exit())
}
