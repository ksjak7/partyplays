use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use axum::{
    routing::{get, post},
    Router,
};
use tower_http::cors::{Any, CorsLayer};
use vigem_client::Client;

use super::{
    handlers::{create_controllers, get_controller_ids, handle_action},
    models::appstate::AppState,
};

#[tokio::main]
pub async fn run() {
    let client = Client::connect().unwrap();
    let state = Arc::new(AppState {
        client: Arc::new(client),
        controller_ids: Mutex::new(Vec::new()),
        virtual_targets: Mutex::new(HashMap::new()),
    });

    let cors: CorsLayer = CorsLayer::new().allow_origin(Any).allow_headers(Any);

    let app = Router::new()
        .route("/controllers", get(get_controller_ids))
        .route("/controllers", post(create_controllers))
        .route("/controllers/input", post(handle_action))
        .layer(cors)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
