use super::models::party_plays_app::PartyPlaysApp;
use crate::shared_models::shared_state::SharedState;
use eframe::egui;
use std::sync::Arc;

pub async fn start(state: Arc<SharedState>) -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_maximize_button(false)
            .with_resizable(false)
            .with_inner_size([520.0, 420.0]),
        ..Default::default()
    };

    eframe::run_native(
        "PartyPlays",
        options,
        Box::new(|cc| Ok(Box::new(PartyPlaysApp::new(cc, state)))),
    )
}
