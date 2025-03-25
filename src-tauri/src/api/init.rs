use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use axum::{
    routing::{any, get, post},
    Router,
};

use tokio::net::TcpListener;
use tower_http::{
    cors::CorsLayer,
    services::{ServeDir, ServeFile}
};
use vigem_client::{Client, XButtons, XGamepad};

use super::{
    handlers::{create_controllers, get_controller_ids, ws_controller_handler},
    models::appstate::AppState,
};

#[tokio::main]
pub async fn run() {
    let client = Client::connect().unwrap();
    let binary_string_input_converter: HashMap<String, u16> = HashMap::from([
        (String::from("a"), XButtons::A),
        (String::from("b"), XButtons::B),
        (String::from("x"), XButtons::X),
        (String::from("y"), XButtons::Y),
        (String::from("dpad_left"), XButtons::LEFT),
        (String::from("dpad_up"), XButtons::UP),
        (String::from("dpad_down"), XButtons::DOWN),
        (String::from("dpad_right"), XButtons::RIGHT),
        (String::from("back"), XButtons::BACK),
        (String::from("start"), XButtons::START),
        (String::from("lb"), XButtons::LB),
        (String::from("rb"), XButtons::RB),
        (String::from("ls"), XButtons::LTHUMB),
        (String::from("rs"), XButtons::RTHUMB),
    ]);

    let state = Arc::new(AppState {
        client: Arc::new(client),
        controller_ids: Mutex::new(Vec::new()),
        virtual_targets: Mutex::new(HashMap::new()),
        binary_string_input_converter: Arc::new(binary_string_input_converter),
        gamepad_off: Arc::new(XGamepad::default()),
    });

    let app = Router::new()
        .route("/controllers", get(get_controller_ids))
        .route("/controllers", post(create_controllers))
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
