use std::net::SocketAddr;

use super::UIState;
use crate::window::create_desktop_window;
use mirrorx_core::{
    api::endpoint::id::EndPointID, core_error, error::CoreResult, utility::nonce_value::NonceValue,
};
use ring::aead::BoundKey;
use tauri_egui::EguiPluginHandle;

#[tauri::command]
#[tracing::instrument(skip(state, egui_plugin))]
pub async fn signaling_key_exchange(
    addr: String,
    local_device_id: String,
    remote_device_id: String,
    password: String,
    state: tauri::State<'_, UIState>,
    egui_plugin: tauri::State<'_, EguiPluginHandle>,
) -> CoreResult<()> {
    let addr: SocketAddr = addr
        .parse()
        .map_err(|err| core_error!("parse SocketAddr from str failed ({})", err))?;
    let local_device_id = local_device_id.replace('-', "").parse()?;
    let remote_device_id: i64 = remote_device_id.replace('-', "").parse()?;
    let window_label = format!("MirrorX {}", remote_device_id);

    let signaling_provider = state.signaling_client.lock().await;
    let signaling_provider = signaling_provider
        .as_ref()
        .ok_or_else(|| core_error!("current signaling provider is empty"))?;

    let resp = signaling_provider
        .key_exchange(mirrorx_core::api::signaling::KeyExchangeRequest {
            local_device_id,
            remote_device_id,
            password,
        })
        .await?;

    let unbound_sealing_key =
        ring::aead::UnboundKey::new(&ring::aead::AES_256_GCM, &resp.sealing_key_bytes)?;

    let mut nonce = [0u8; 12];
    nonce.copy_from_slice(&resp.sealing_nonce_bytes);
    let sealing_key = ring::aead::SealingKey::new(unbound_sealing_key, NonceValue::new(nonce));

    let unbound_opening_key =
        ring::aead::UnboundKey::new(&ring::aead::AES_256_GCM, &resp.opening_key_bytes)?;

    let mut nonce = [0u8; 12];
    nonce.copy_from_slice(&resp.opening_nonce_bytes);
    let opening_key = ring::aead::OpeningKey::new(unbound_opening_key, NonceValue::new(nonce));

    tracing::info!(?local_device_id, ?remote_device_id, "key exchange success");

    if let Err(err) = egui_plugin.create_window(
        window_label.clone(),
        Box::new(move |cc| {
            if let Some(gl_context) = cc.gl.as_ref() {
                Box::new(create_desktop_window(
                    cc,
                    gl_context.clone(),
                    EndPointID::DeviceID {
                        local_device_id: local_device_id,
                        remote_device_id: remote_device_id,
                    },
                    Some((opening_key, sealing_key)),
                    Some(resp.visit_credentials),
                    addr,
                ))
            } else {
                panic!("get gl context failed");
            }
        }),
        window_label,
        tauri_egui::eframe::NativeOptions {
            // hardware_acceleration: HardwareAcceleration::Required,
            ..Default::default()
        },
    ) {
        tracing::error!(?err, "create desktop window failed");
        return Err(core_error!("create remote desktop window failed"));
    }

    Ok(())
}
