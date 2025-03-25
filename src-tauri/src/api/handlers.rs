use std::{net::SocketAddr, sync::Arc, thread::sleep, time::Duration};

use axum::{
    extract::{ws::{Message, WebSocket}, ConnectInfo, State, WebSocketUpgrade},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use nanoid::nanoid;
use tokio::spawn;
use vigem_client::{TargetId, XButtons, XGamepad, Xbox360Wired};

use super::models::{appstate::AppState, requests::CreateControllersRequest};

pub async fn create_controllers(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateControllersRequest>,
) -> Response {
    let mut virtual_targets = match state.virtual_targets.lock() {
        Ok(targets) => targets,
        _ => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "failed to reset existing controllers",
            )
                .into_response();
        }
    };

    virtual_targets.clear();

    let mut controller_ids: Vec<String> = Vec::with_capacity(payload.number_of_controllers.into());
    for _ in 0..payload.number_of_controllers {
        let controller_id = nanoid!(6);
        let new_target = Xbox360Wired::new(state.client.clone(), TargetId::XBOX360_WIRED);

        controller_ids.push(controller_id.clone());
        virtual_targets.insert(controller_id, new_target);
    }

    for controller in virtual_targets.values_mut() {
        match controller.plugin() {
            Ok(_) => {}
            _ => {
                virtual_targets.clear();
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "failed to initialize controllers",
                )
                    .into_response();
            }
        };
    }

    (StatusCode::OK, Json(controller_ids)).into_response()
}

pub async fn get_controller_ids(State(state): State<Arc<AppState>>) -> Json<Vec<String>> {
    match state.controller_ids.lock() {
        Ok(result) => Json(result.clone()),
        _ => Json(Vec::new()),
    }
}

pub async fn ws_controller_handler(
    State(state): State<Arc<AppState>>,
    ws: WebSocketUpgrade,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    println!("Connected at {addr}");

    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

pub async fn handle_socket(mut socket: WebSocket, state: Arc<AppState>) {
    spawn(async move {
        while let Some(Ok(msg)) = socket.recv().await {
            let text = match msg.into_text() {
                Ok(t) => t,
                _ => {
                    println!("unable to convert message to text");
                    return;
                }
            };

            let mut msg_parts = text.split("::");

            let controller_id = match msg_parts.next() {
                Some(t) => String::from(t),
                _ => {
                    println!("unable to find controller_id");
                    return;
                }
            };

            let action_id = match msg_parts.next() {
                Some(t) => String::from(t),
                _ => {
                    println!("unable to find action_id");
                    return;
                }
            };

            handle_action(state.clone(), controller_id, action_id).await;
        }
    });
}

pub async fn handle_action(state: Arc<AppState>, controller_id: String, action_id: String) {
    println!("{} :: {}", controller_id, action_id);

    let action_id = action_id.to_ascii_lowercase();

    let button = match state.binary_string_input_converter.get(&action_id) {
        Some(button) => button,
        _ => {
            println!("invalid input type");
            return;
        }
    };

    let mut virtual_targets = match state.virtual_targets.lock() {
        Ok(targets) => targets,
        _ => {
            println!("failed to lock controllers");
            return;
        }
    };

    let current_target = match virtual_targets.get_mut(&controller_id) {
        Some(target) => target,
        _ => {
            println!("invalid controller id");
            return;
        }
    };

    let gamepad = XGamepad {
        buttons: XButtons(button.clone()),
        thumb_lx: 32767,
        ..Default::default()
    };

    match current_target.update(&gamepad) {
        Ok(_) => {}
        _ => {
            println!("unable to update controller");
            return;
        }
    }

    sleep(Duration::from_millis(50));

    match current_target.update(&state.gamepad_off) {
        Ok(_) => {}
        _ => {
            println!("unable to update controller");
            return;
        }
    }
}