use std::sync::Arc;

use crate::{shared_models::shared_state::SharedState, ui::actions};

pub struct PartyPlaysApp {
    pub shared_state: Arc<SharedState>,
    pub controller_count: usize,
    pub error_value: String,
}

impl PartyPlaysApp {
    pub fn new(shared_state: Arc<SharedState>) -> Self {
        Self {
            shared_state,
            controller_count: 1,
            error_value: String::from(""),
        }
    }
}

impl eframe::App for PartyPlaysApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                let controller_id_label = ui.label("Controller Ids:");
                ui.text_edit_singleline(
                    &mut self.shared_state.controller_ids.read().unwrap().join(" | "),
                )
                .labelled_by(controller_id_label.id);
            });
            ui.add(
                egui::Slider::new(&mut self.controller_count, 1..=16).text("Number of Controllers"),
            );
            if ui.button("Create Controllers").clicked() {
                if let Err(e) =
                    actions::create_controllers(self.shared_state.clone(), self.controller_count)
                {
                    self.error_value = format!("{}", e);
                }
            }
        });
    }
}
