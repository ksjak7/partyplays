use egui::Ui;

pub fn button_group(ui: &mut Ui, current_buttons_pressed: u16, buttons: Vec<(u16, &str)>) -> u16 {
    let mut result = 0;
    for (target_button, display_text) in buttons {
        result += controller_button(ui, current_buttons_pressed, target_button, display_text);
    }
    ui.add_space(ui.spacing().indent);

    result
}
pub fn controller_button(
    ui: &mut Ui,
    current_buttons_pressed: u16,
    target_button: u16,
    display_text: &str,
) -> u16 {
    let mut result = 0;
    ui.label(
        if (current_buttons_pressed & target_button) == target_button {
            result += target_button;
            egui::RichText::new(display_text).color(egui::Color32::RED)
        } else {
            egui::RichText::new(display_text)
        },
    );
    result
}
