use std::collections::BTreeMap;
use std::sync::Arc;

use egui::FontFamily::Proportional;
use egui::FontId;
use egui::TextStyle::*;

use crate::{shared_models::shared_state::SharedState, ui::actions};

pub struct PartyPlaysApp {
    pub shared_state: Arc<SharedState>,
    pub controller_count: usize,
    pub error_value: String,
    pub show_ip_address: bool,
    pub ip_address: String,
    pub port: String,
}

impl PartyPlaysApp {
    pub fn new(ctx: &eframe::CreationContext<'_>, shared_state: Arc<SharedState>) -> Self {
        let state_ip_address = shared_state.local_ip_address.clone();
        let mut address_parts = state_ip_address.split(":");
        let ip_address = address_parts.next().unwrap().into();
        let port = address_parts.next().unwrap().into();

        update_text_styles(&ctx.egui_ctx);

        Self {
            shared_state,
            controller_count: 1,
            error_value: String::from(""),
            show_ip_address: false,
            ip_address,
            port,
        }
    }
}

impl eframe::App for PartyPlaysApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top").show(ctx, |ui| {
            ui.horizontal(|ui| {
                let number_of_controllers_label = ui.label("Number of Controllers:");
                ui.add(egui::Slider::new(&mut self.controller_count, 1..=16))
                    .labelled_by(number_of_controllers_label.id);

                ui.add_space(40.0);

                if ui.button("Create Controllers").clicked() {
                    if let Err(e) = actions::create_controllers(
                        self.shared_state.clone(),
                        self.controller_count,
                    ) {
                        self.error_value = format!("{}", e);
                    }
                }
            });
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                let controller_ids = self.shared_state.controller_ids.read().unwrap();

                for (index, id) in controller_ids.iter().enumerate() {
                    ui.add(egui::Label::new(format!(
                        "Controller {}: {}",
                        index + 1,
                        id
                    )));
                }
            });
        });
        egui::TopBottomPanel::bottom("bottom").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.add(egui::Label::new(format!(
                    "{}:{}",
                    if self.show_ip_address {
                        self.ip_address.clone()
                    } else {
                        "xxx.xxx.xxx.xxx".into()
                    },
                    self.port
                )));
                if ui
                    .button(if self.show_ip_address { "Hide" } else { "Show" })
                    .clicked()
                {
                    self.show_ip_address = !self.show_ip_address;
                }
            });
        });
    }
}

pub fn update_text_styles(ctx: &egui::Context) {
    let text_styles: BTreeMap<_, _> = [
        (Body, FontId::new(16.0, Proportional)),
        (Button, FontId::new(16.0, Proportional)),
    ]
    .into();
    ctx.all_styles_mut(move |style| style.text_styles = text_styles.clone());
}
