use std::sync::{Arc, Mutex};

use axum::{
    routing::{get, post},
    Router,
};
use tower_http::cors::{Any, CorsLayer};

use super::{
    handlers::{create_controllers, get_controller_ids, handle_action},
    models::appstate::AppState,
};

#[tokio::main]
pub async fn run() {
    let state = Arc::new(AppState {
        controller_ids: Mutex::new(Vec::new()),
        virtual_targets: Mutex::new(Vec::new()),
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
