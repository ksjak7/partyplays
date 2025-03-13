use std::sync::Arc;

use axum::{extract::State, Json};
use nanoid::nanoid;
use vigem_client::{TargetId, Xbox360Wired};

use super::models::{
    appstate::AppState,
    requests::{CreateControllersRequest, HandleActionRequest},
};

pub async fn create_controllers(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateControllersRequest>,
) -> Json<Vec<String>> {
    let mut virtual_targets = state.virtual_targets.lock().unwrap();
    virtual_targets.clear();

    let mut controller_ids: Vec<String> = Vec::with_capacity(payload.number_of_controllers.into());
    for _ in 0..payload.number_of_controllers {
        let controller_id = nanoid!(6);
        let new_target = Xbox360Wired::new(state.client.clone(), TargetId::XBOX360_WIRED);

        controller_ids.push(controller_id.clone());
        virtual_targets.insert(controller_id, new_target);
    }

    for controller in virtual_targets.values_mut() {
        controller.plugin().unwrap();
    }

    Json(controller_ids)
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
