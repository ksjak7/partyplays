use std::{sync::Arc, thread::sleep, time::Duration};

use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use nanoid::nanoid;
use vigem_client::{TargetId, XButtons, XGamepad, Xbox360Wired};

use super::models::{
    appstate::AppState,
    requests::{CreateControllersRequest, HandleActionRequest},
};

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

pub async fn handle_action(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<HandleActionRequest>,
) -> Response {
    println!("{} :: {}", payload.controller_id, payload.action_id);

    let action_id = payload.action_id.to_ascii_lowercase();

    let button = match state.binary_string_input_converter.get(&action_id) {
        Some(button) => button,
        _ => {
            return (StatusCode::BAD_REQUEST, "invalid input type").into_response();
        }
    };

    let mut virtual_targets = match state.virtual_targets.lock() {
        Ok(targets) => targets,
        _ => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "failed to lock controllers",
            )
                .into_response();
        }
    };

    let current_target = match virtual_targets.get_mut(&payload.controller_id) {
        Some(target) => target,
        _ => {
            return (StatusCode::BAD_REQUEST, "invalid controller id").into_response();
        }
    };

    let gamepad = XGamepad {
        buttons: XButtons(button.clone()),
        ..Default::default()
    };

    match current_target.update(&gamepad) {
        Ok(_) => {}
        _ => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "unable to update controller",
            )
                .into_response();
        }
    }

    sleep(Duration::from_millis(50));

    match current_target.update(&state.gamepad_off) {
        Ok(_) => {}
        _ => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "unable to update controller",
            )
                .into_response();
        }
    }

    return StatusCode::NO_CONTENT.into_response();
}
