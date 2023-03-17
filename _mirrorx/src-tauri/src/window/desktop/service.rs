use super::state::StateCommand;
use mirrorx_core::service::endpoint::{self, EndPointID};
use scopeguard::defer;
use std::sync::Arc;
use tokio::{
    select,
    sync::mpsc::{channel, Receiver, Sender},
};

pub enum Command {
    UpdateEGUIContext(tauri_egui::egui::Context),
    Negotiate,
    SwitchScreen(String),
}

pub fn spawn_window_service_task(
    id: EndPointID,
    service: Arc<endpoint::Service>,
    state_command_tx: Sender<StateCommand>,
) -> Sender<Command> {
    let (command_tx, command_rx) = channel(180);

    tokio::spawn(window_service_task(
        id,
        service,
        state_command_tx,
        command_rx,
    ));

    command_tx
}

async fn window_service_task(
    id: EndPointID,
    service: Arc<endpoint::Service>,
    state_command_tx: Sender<StateCommand>,
    mut rx: Receiver<Command>,
) {
    tracing::info!(?id, "start command process task");
    defer! {
        tracing::info!(?id, "stop command process task");
    }

    if let Err(err) = service.spawn_audio_play_task().await {
        let _ = state_command_tx
            .send(StateCommand::ErrorHappened(err))
            .await;
        return;
    }

    let mut video_frame_rx = match service.spawn_video_decode_task().await {
        Ok(rx) => rx,
        Err(err) => {
            let _ = state_command_tx
                .send(StateCommand::ErrorHappened(err))
                .await;
            return;
        }
    };

    let mut egui_context: Option<tauri_egui::egui::Context> = None;

    loop {
        select! {
            frame = video_frame_rx.recv() => match frame {
                Some(frame) => {
                    let has_error = state_command_tx.send(StateCommand::UpdateVideoFrame(frame)).await.is_err();
                    if let Some(ref ctx) = egui_context {
                        ctx.request_repaint();
                    }

                    if has_error {
                        tracing::error!("state command channel closed");
                        break;
                    }
                },
                None => break,
            },
            command = rx.recv() => match command {
                Some(command) => match command {
                    Command::UpdateEGUIContext(ctx) => {
                        egui_context = Some(ctx);
                    }
                    Command::Negotiate => {
                        handle_negotiate(service.clone(), state_command_tx.clone()).await;
                    },
                    Command::SwitchScreen(screen_id) => {
                        handle_switch_screen(screen_id, service.clone(), state_command_tx.clone()).await;
                    }
                },
                None => break,
            }
        }
    }
}

async fn handle_negotiate(service: Arc<endpoint::Service>, state_command_tx: Sender<StateCommand>) {
    let state_command = match service.call_negotiate().await {
        Ok(reply) => StateCommand::NegotiateFinished(reply),
        Err(err) => StateCommand::ErrorHappened(err),
    };

    let _ = state_command_tx.send(state_command).await;
}

async fn handle_switch_screen(
    screen_id: String,
    service: Arc<endpoint::Service>,
    state_command_tx: Sender<StateCommand>,
) {
    let command = match service.call_switch_screen(screen_id).await {
        Ok(reply) => StateCommand::SwitchDisplay(reply.display_id),
        Err(err) => StateCommand::ErrorHappened(err),
    };

    let _ = state_command_tx.send(command).await;
}
