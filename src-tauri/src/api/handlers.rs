use std::{net::SocketAddr, sync::Arc, thread::sleep, time::Duration};

use axum::{
    extract::{ws::WebSocket, ConnectInfo, State, WebSocketUpgrade},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use itertools::Itertools;
use nanoid::nanoid;
use tokio::spawn;
use vigem_client::{TargetId, XGamepad, Xbox360Wired};

use crate::api::utils::ClampAdd;

use super::models::{
    appstate::AppState,
    error::Error,
    requests::{CreateControllersRequest, HandleActionRequest},
    virtual_target::VirtualTarget,
};

pub async fn create_controllers(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateControllersRequest>,
) -> Result<Response, Error> {
    let mut virtual_targets = state.virtual_targets.lock()?;
    virtual_targets.clear();

    let mut controller_ids: Vec<String> = Vec::with_capacity(payload.number_of_controllers.into());
    for _ in 0..payload.number_of_controllers {
        let controller_id = nanoid!(6);
        let new_target = Xbox360Wired::new(state.client.clone(), TargetId::XBOX360_WIRED);

        controller_ids.push(controller_id.clone());
        virtual_targets.insert(
            controller_id,
            VirtualTarget {
                controller: new_target,
                state: XGamepad::default(),
            },
        );
    }

    for target in virtual_targets.values_mut() {
        if let Err(e) = target.controller.plugin() {
            virtual_targets.clear();
            return Err(Error::from(e));
        };
    }

    Ok((StatusCode::OK, Json(controller_ids)).into_response())
}

pub async fn get_controller_ids(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<String>>, Error> {
    let controller_ids = state.controller_ids.lock()?;
    Ok(Json(controller_ids.clone()))
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
                    continue;
                }
            };

            let request = match serde_json::from_str::<HandleActionRequest>(&text) {
                Ok(r) => r,
                _ =>  {
                    println!("unable to deserialize message");
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
    state: Arc<AppState>,
    request: HandleActionRequest,
) -> Result<(), Error> {
    println!(
        "{} :: [{}]",
        request.controller_id,
        request.action_ids.join(" | ").to_ascii_lowercase()
    );

    let mut virtual_targets = state.virtual_targets.lock()?;

    let current_target =
        virtual_targets
            .get_mut(&request.controller_id)
            .ok_or(Error::OptionRetrieveError(
                "failed to get current_target from virtual_targets".into(),
            ))?;

    for action_id in request.action_ids.clone() {
        if let Some(button) = state.binary_string_input_converter.get(&action_id) {
            current_target.state.buttons.raw |= button;
        }
    }

    current_target
        .state
        .left_trigger
        .clamp_add(request.triggers.left);
    current_target
        .state
        .right_trigger
        .clamp_add(request.triggers.right);
    current_target
        .state
        .thumb_lx
        .clamp_add(request.left_stick.x);
    current_target
        .state
        .thumb_ly
        .clamp_add(request.left_stick.y);
    current_target
        .state
        .thumb_rx
        .clamp_add(request.right_stick.x);
    current_target
        .state
        .thumb_ry
        .clamp_add(request.right_stick.y);

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
