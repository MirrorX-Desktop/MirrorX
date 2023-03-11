use super::state::StateCommand;
use mirrorx_core::service::endpoint::{self, EndPointID};
use scopeguard::defer;
use std::sync::Arc;
use tokio::sync::mpsc::{channel, Receiver, Sender};

pub enum Command {
    Negotiate,
    SwitchScreen(String),
}

pub fn spawn_command_process_task(
    id: EndPointID,
    service: Arc<endpoint::Service>,
    state_command_tx: Sender<StateCommand>,
) -> Sender<Command> {
    let (command_tx, command_rx) = channel(180);

    tokio::spawn(command_process_task(
        id,
        service,
        state_command_tx,
        command_rx,
    ));

    command_tx
}

async fn command_process_task(
    id: EndPointID,
    service: Arc<endpoint::Service>,
    state_command_tx: Sender<StateCommand>,
    mut rx: Receiver<Command>,
) {
    tracing::info!(?id, "start command process task");
    defer! {
        tracing::info!(?id, "stop command process task");
    }

    loop {
        let Some(command) = rx.recv().await else {
            break;
        };

        match command {
            Command::Negotiate => handle_negotiate(service.clone(), state_command_tx.clone()).await,
            Command::SwitchScreen(screen_id) => {
                handle_switch_screen(screen_id, service.clone(), state_command_tx.clone()).await;
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
