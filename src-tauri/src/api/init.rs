use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use axum::{
    routing::{get, post},
    Router,
};
use tower_http::cors::CorsLayer;
use vigem_client::{Client, XButtons, XGamepad};

use super::{
    handlers::{create_controllers, get_controller_ids, handle_action},
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
        .route("/controllers/input", post(handle_action))
        .layer(CorsLayer::very_permissive())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
