use crate::api::models::requests::HandleActionRequest;
use crate::shared_models::{error::Error, shared_state::SharedState};
use axum::http::StatusCode;
use axum::{
    extract::{ws::WebSocket, ConnectInfo, State, WebSocketUpgrade},
    response::IntoResponse,
};
use std::{net::SocketAddr, sync::Arc, thread::sleep, time::Duration};
use tokio::spawn;

pub async fn health_handler() -> impl IntoResponse {
    (StatusCode::OK, "Healthy").into_response()
}
pub async fn ws_controller_handler(
    State(state): State<Arc<SharedState>>,
    ws: WebSocketUpgrade,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    println!("Connected at {addr}");

    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

pub async fn handle_socket(mut socket: WebSocket, state: Arc<SharedState>) {
    spawn(async move {
        while let Some(Ok(msg)) = socket.recv().await {
            let text = match msg.into_text() {
                Ok(t) => t,
                _ => {
                    println!("unable to convert message to text");
                    continue;
                }
            };

            let request = match serde_json::from_str::<HandleActionRequest>(&text) {
                Ok(r) => r,
                Err(e) => {
                    println!("unable to deserialize message :: {e}");
                    continue;
                }
            };

            if let Some(e) = handle_action(state.clone(), request).await.err() {
                println!("{e}");
            };
        }
    });
}

pub async fn handle_action(
    state: Arc<SharedState>,
    request: HandleActionRequest,
) -> Result<(), Error> {
    let mut virtual_targets = state.virtual_targets.write()?;

    let current_target =
        virtual_targets
            .get_mut(&request.controller_id)
            .ok_or(Error::OptionRetrieve(
                "failed to get current_target from virtual_targets".into(),
            ))?;

    for action_id in request.action_ids.clone() {
        if let Some(button) = state.binary_string_input_converter.get(&action_id) {
            current_target.state.buttons.raw |= button;
            current_target.ui_buttons_pressed |= button;
        }
    }

    current_target.state.thumb_lx = request.left_stick.x.clamp(-100, 100) * 300;
    current_target.state.thumb_ly = request.left_stick.y.clamp(-100, 100) * 300;
    current_target.state.thumb_rx = request.right_stick.x.clamp(-100, 100) * 300;
    current_target.state.thumb_ry = request.right_stick.y.clamp(-100, 100) * 300;
    current_target.state.left_trigger =
        (f32::from(request.triggers.left.clamp(0, 100)) * 2.55).ceil() as u8;
    current_target.state.right_trigger =
        (f32::from(request.triggers.right.clamp(0, 100)) * 2.55).ceil() as u8;

    current_target.controller.update(&current_target.state)?;

    sleep(Duration::from_millis(50));

    for action_id in request.action_ids {
        if let Some(button) = state.binary_string_input_converter.get(&action_id) {
            current_target.state.buttons.raw -= button;
        }
    }

    current_target.controller.update(&current_target.state)?;
    Ok(())
}
