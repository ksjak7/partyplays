use egui::FontFamily::Proportional;
use egui::FontId;
use egui::TextStyle::*;
use std::collections::BTreeMap;

pub fn update_text_styles(ctx: &egui::Context) {
    let text_styles: BTreeMap<_, _> = [
        (Heading, FontId::new(16.0, Proportional)),
        (Body, FontId::new(16.0, Proportional)),
        (Button, FontId::new(16.0, Proportional)),
    ]
    .into();
    ctx.all_styles_mut(move |style| style.text_styles = text_styles.clone());
}
