use crate::{
    error::MirrorXError,
    socket::endpoint::{
        endpoint::{EndPoint, ENDPOINTS},
        message::{
            GetDisplayInfoRequest, GetDisplayInfoResponse, StartMediaTransmissionRequest,
            StartMediaTransmissionResponse,
        },
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
    let endpoint = EndPoint::connect(
        "192.168.0.101:28001",
        is_active_side,
        local_device_id,
        remote_device_id.clone(),
        opening_key,
        sealing_key,
    )
    .await?;

    ENDPOINTS.insert(remote_device_id, endpoint);

    Ok(())
}

pub async fn get_display_info(
    remote_device_id: String,
) -> Result<GetDisplayInfoResponse, MirrorXError> {
    let endpoint = match ENDPOINTS.get(&remote_device_id) {
        Some(pair) => pair,
        None => return Err(MirrorXError::EndPointNotFound(remote_device_id)),
    };

    endpoint.get_display_info(GetDisplayInfoRequest {}).await
}

pub async fn start_media_transmission(
    remote_device_id: String,
    display_id: String,
    texture_id: i64,
    video_texture_ptr: i64,
    update_frame_callback_ptr: i64,
) -> Result<StartMediaTransmissionResponse, MirrorXError> {
    let endpoint = match ENDPOINTS.get(&remote_device_id) {
        Some(pair) => pair,
        None => return Err(MirrorXError::EndPointNotFound(remote_device_id)),
    };

    endpoint.start_video_render_process(
        texture_id,
        video_texture_ptr,
        update_frame_callback_ptr,
    )?;
    endpoint.start_audio_play_process().await?;

    let resp = endpoint
        .start_media_transmission(StartMediaTransmissionRequest {
            expect_fps: 60,
            expect_display_id: display_id,
        })
        .await?;

    Ok(resp)
}
