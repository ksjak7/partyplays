use std::net::SocketAddr;
use std::sync::Arc;

use axum::{routing::any, Router};

use tokio::net::TcpListener;
use tower_http::{
    cors::CorsLayer,
    services::{ServeDir, ServeFile},
};

use crate::models::apistate::ApiState;

use super::handlers::ws_controller_handler;

pub async fn run(state: Arc<ApiState>) {
    println!("initializing router");

    let app = Router::new()
        .route("/ws", any(ws_controller_handler))
        .route_service("/", ServeFile::new("public/index.html"))
        .nest_service("/public", ServeDir::new("public"))
        .layer(CorsLayer::very_permissive())
        .with_state(state);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}
