use super::handlers::ws_controller_handler;
use crate::shared_models::shared_state::SharedState;
use axum::{routing::any, Router};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::{
    cors::CorsLayer,
    services::{ServeDir, ServeFile},
};

pub async fn start(state: Arc<SharedState>) {
    println!("initializing router");

    let app = Router::new()
        .route("/ws", any(ws_controller_handler))
        .route_service("/", ServeFile::new("public/index.html"))
        .nest_service("/public", ServeDir::new("public"))
        .layer(CorsLayer::very_permissive())
        .with_state(state);

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}
