use super::models::party_plays_app::PartyPlaysApp;
use crate::shared_models::shared_state::SharedState;
use eframe::egui;
use std::sync::Arc;

pub async fn start(state: Arc<SharedState>) -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([932.0, 320.0]),
        ..Default::default()
    };

    eframe::run_native(
        "PartyPlays",
        options,
        Box::new(|cc| Ok(Box::new(PartyPlaysApp::new(cc, state)))),
    )
}
