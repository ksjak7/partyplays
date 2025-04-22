use egui::style::HandleShape;
use std::sync::Arc;
use std::time::Duration;
use vigem_client::XButtons;

use crate::ui::components::controller_button::button_group;
use crate::{
    shared_models::shared_state::SharedState,
    ui::actions::{clear_targets_and_controllers, create_controllers},
    ui::styles::update_text_styles,
};

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

                ui.add_space(ui.spacing().menu_spacing);

                if ui.button("Create").clicked() {
                    if let Err(e) =
                        create_controllers(self.shared_state.clone(), self.controller_count)
                    {
                        self.error_value = format!("{}", e);
                    } else {
                        self.error_value.clear();
                    }
                }

                if ui.button("Clear").clicked() {
                    if let Err(e) = clear_targets_and_controllers(self.shared_state.clone()) {
                        self.error_value = format!("{}", e);
                    } else {
                        self.error_value.clear();
                    }
                }
            });
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            let controller_ids = self.shared_state.controller_ids.read().unwrap();
            let mut virtual_targets = self.shared_state.virtual_targets.write().unwrap();
            ui.vertical(|ui| {
                egui::Grid::new("controller_ids")
                    .min_col_width(450.0)
                    .show(ui, |ui| {
                        for (index, id) in controller_ids.iter().enumerate() {
                            let virtual_target = virtual_targets.get_mut(id).unwrap();
                            ui.vertical(|ui| {
                                ui.label(format!("{} :: {}", id, index));

                                ui.horizontal(|ui| {
                                    virtual_target.ui_buttons_pressed -= button_group(
                                        ui,
                                        virtual_target.ui_buttons_pressed,
                                        [
                                            (XButtons::A, "A"),
                                            (XButtons::B, "B"),
                                            (XButtons::X, "X"),
                                            (XButtons::Y, "Y"),
                                        ]
                                        .into(),
                                    );
                                    virtual_target.ui_buttons_pressed -= button_group(
                                        ui,
                                        virtual_target.ui_buttons_pressed,
                                        [
                                            (XButtons::UP, "Up"),
                                            (XButtons::DOWN, "Down"),
                                            (XButtons::LEFT, "Left"),
                                            (XButtons::RIGHT, "Right"),
                                        ]
                                        .into(),
                                    );
                                    virtual_target.ui_buttons_pressed -= button_group(
                                        ui,
                                        virtual_target.ui_buttons_pressed,
                                        [(XButtons::LB, "LB"), (XButtons::RB, "RB")].into(),
                                    );
                                    virtual_target.ui_buttons_pressed -= button_group(
                                        ui,
                                        virtual_target.ui_buttons_pressed,
                                        [(XButtons::LTHUMB, "LS"), (XButtons::RTHUMB, "RS")].into(),
                                    );
                                });
                                ui.horizontal(|ui| {
                                    ui.add_space(ui.spacing().indent);
                                    ui.vertical(|ui| {
                                        ui.label("Left Stick:");
                                        ui.add(
                                            egui::Slider::new(
                                                &mut virtual_target.state.thumb_lx,
                                                i16::MIN..=i16::MAX,
                                            )
                                            .handle_shape(HandleShape::Rect { aspect_ratio: 0.5 })
                                            .show_value(false),
                                        );
                                        ui.add(
                                            egui::Slider::new(
                                                &mut virtual_target.state.thumb_ly,
                                                i16::MIN..=i16::MAX,
                                            )
                                            .handle_shape(HandleShape::Rect { aspect_ratio: 0.5 })
                                            .show_value(false),
                                        );
                                    });
                                    ui.vertical(|ui| {
                                        ui.label("Right Stick:");
                                        ui.add(
                                            egui::Slider::new(
                                                &mut virtual_target.state.thumb_rx,
                                                i16::MIN..=i16::MAX,
                                            )
                                            .handle_shape(HandleShape::Rect { aspect_ratio: 0.5 })
                                            .show_value(false),
                                        );
                                        ui.add(
                                            egui::Slider::new(
                                                &mut virtual_target.state.thumb_ry,
                                                i16::MIN..=i16::MAX,
                                            )
                                            .handle_shape(HandleShape::Rect { aspect_ratio: 0.5 })
                                            .show_value(false),
                                        );
                                    });
                                    ui.vertical(|ui| {
                                        ui.label("Triggers:");
                                        ui.add(
                                            egui::Slider::new(
                                                &mut virtual_target.state.left_trigger,
                                                u8::MIN..=u8::MAX,
                                            )
                                            .handle_shape(HandleShape::Rect { aspect_ratio: 0.5 })
                                            .show_value(false),
                                        );
                                        ui.add(
                                            egui::Slider::new(
                                                &mut virtual_target.state.right_trigger,
                                                u8::MIN..=u8::MAX,
                                            )
                                            .handle_shape(HandleShape::Rect { aspect_ratio: 0.5 })
                                            .show_value(false),
                                        );
                                    });
                                });
                            });
                            if index % 2 == 1 {
                                ui.end_row();
                            }
                        }
                    });
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
                if !self.error_value.is_empty() {
                    ui.add(egui::Label::new(
                        egui::RichText::new(format!("Error: {}", self.error_value))
                            .color(egui::Color32::RED),
                    ));
                }
            });
        });
        ctx.request_repaint_after(Duration::from_millis(50));
    }
}
