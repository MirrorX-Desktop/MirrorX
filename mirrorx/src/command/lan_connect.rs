use crate::window::create_desktop_window;
use mirrorx_core::{
    api::endpoint::id::EndPointID, core_error, error::CoreResult, utility::lan_ip::get_lan_ip,
};
use std::net::{IpAddr, SocketAddr};
use tauri_egui::EguiPluginHandle;

#[tauri::command]
pub async fn lan_connect(
    addr: String,
    egui_plugin: tauri::State<'_, EguiPluginHandle>,
) -> CoreResult<()> {
    let local_ip = get_lan_ip().await?;
    let remote_ip: IpAddr = addr
        .parse()
        .map_err(|_| core_error!("parse addr to IpAddr failed"))?;
    let remote_addr = SocketAddr::new(remote_ip, 30000);
    let window_label = format!("MirrorX {}", remote_ip);

    if let Err(err) = egui_plugin.create_window(
        window_label.clone(),
        Box::new(move |cc| {
            if let Some(gl_context) = cc.gl.as_ref() {
                Box::new(create_desktop_window(
                    cc,
                    gl_context.clone(),
                    EndPointID::LANID {
                        local_ip,
                        remote_ip,
                    },
                    None,
                    None,
                    remote_addr,
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
