#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod api;
mod shared_models;
mod ui;

use shared_models::shared_state::SharedState;
use tokio::task;

#[tokio::main]
async fn main() {
    let shared_state = SharedState::new_arc();
    let api_task = task::spawn(api::start(shared_state.clone()));

    tokio::select! {
        _ = api_task => println!("api finished"),
        _ = ui::start(shared_state.clone()) => println!("ui finished")
    }
}
