#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod handlers;
mod init;
mod models;

use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use eframe::egui;
use models::apistate::ApiState;
use tokio::task;
use vigem_client::{Client, XButtons};

#[tokio::main]
async fn main() {
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

    let state = Arc::new(ApiState {
        client: Arc::new(client),
        controller_ids: RwLock::new(Vec::new()),
        virtual_targets: RwLock::new(HashMap::new()),
        binary_string_input_converter: Arc::new(binary_string_input_converter),
    });

    let api_task = task::spawn(init::run(state.clone()));

    tokio::select! {
        _ = api_task => println!("api finished"),
        _ = ui(state.clone()) => println!("ui finished")
    }
}

async fn ui(state: Arc<ApiState>) -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };

    eframe::run_native(
        "PartyPlays",
        options,
        Box::new(|_| Ok(Box::new(PartyPlaysApp::new(state)))),
    )
}

struct PartyPlaysApp {
    api_state: Arc<ApiState>,
    controller_count: i32,
}

impl PartyPlaysApp {
    fn new(api_state: Arc<ApiState>) -> Self {
        Self {
            api_state,
            controller_count: 1,
        }
    }
}

impl eframe::App for PartyPlaysApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                let controller_id_label = ui.label("Controller Ids:");
                ui.text_edit_singleline(
                    &mut self.api_state.controller_ids.read().unwrap().join(" | "),
                )
                .labelled_by(controller_id_label.id);
            });
            ui.add(
                egui::Slider::new(&mut self.controller_count, 1..=16).text("Number of Controllers"),
            );
            if ui.button("Create Controllers").clicked() {
                let _ = handlers::create_controllers(
                    self.api_state.clone(),
                    self.controller_count.try_into().unwrap(),
                );
            }
        });
    }
}
