use std::sync::Arc;

use axum::{extract::State, Json};
use nanoid::nanoid;

use super::models::{
    appstate::AppState,
    requests::{CreateControllersRequest, HandleActionRequest},
};

pub async fn create_controllers(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateControllersRequest>,
) -> Json<Vec<String>> {
    let mut new_controller_ids: Vec<String> = Vec::new();
    for _ in 0..payload.number_of_controllers {
        new_controller_ids.push(nanoid!(6));
    }

    println!("{}", new_controller_ids.join(" | "));

    match state.controller_ids.lock() {
        Ok(mut controller_ids) => {
            controller_ids.clear();
            controller_ids.append(&mut new_controller_ids);
            Json(controller_ids.clone())
        }
        _ => Json(Vec::new()),
    }
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
) {
    println!("{} :: {}", payload.controller_id, payload.action_id);
}
