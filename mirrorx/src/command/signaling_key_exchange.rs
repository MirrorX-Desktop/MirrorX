use super::UIState;
use crate::window::create_desktop_window;
use mirrorx_core::{core_error, error::CoreResult};
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

    tracing::info!(?local_device_id, ?remote_device_id, "key exchange success");

    if let Err(err) = egui_plugin.create_window(
        window_label.clone(),
        Box::new(move |cc| {
            if let Some(gl_context) = cc.gl.as_ref() {
                Box::new(create_desktop_window(
                    cc,
                    gl_context.clone(),
                    local_device_id,
                    remote_device_id,
                    resp.opening_key_bytes,
                    resp.opening_nonce_bytes,
                    resp.sealing_key_bytes,
                    resp.sealing_nonce_bytes,
                    resp.visit_credentials,
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
