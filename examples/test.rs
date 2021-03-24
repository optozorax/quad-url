use macroquad::prelude::*;
use quad_url::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "quad-url".to_owned(),
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut parameters = get_program_parameters();
    let mut to_open = String::new();
    let mut new_tab = false;

    let mut param_name = String::new();
    let mut param_value = String::new();

    let mut to_delete = String::new();

    let mut url = path(false);
    let mut url_full = path(true);

    let mut hash = get_hash();
    let mut hash_to_set = String::new();

    loop {
        clear_background(WHITE);

        egui_macroquad::ui(|egui_ctx| {
            egui::Window::new("Test").show(egui_ctx, |ui| {
                ui.label("Url:");
                ui.monospace(&url);
                if ui.button("Update").clicked() {
                    url = path(false);
                }
                ui.separator();
                ui.label("Full url:");
                ui.monospace(&url_full);
                if ui.button("Update").clicked() {
                    url_full = path(true);
                }
                ui.separator();
                if ui.button("Update parameters").clicked() {
                    parameters = get_program_parameters();
                }
                ui.label("Parameters:");
                for param in &parameters {
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = 0.;
                        ui.label("• `");
                        ui.monospace(param);
                        ui.label("`.");
                        if let Some((name, value)) = easy_parse(param) {
                            ui.label(" Parsed: `");
                            ui.monospace(name);
                            if let Some(value) = value {
                                ui.label("` = `");
                                ui.monospace(value);
                            }
                            ui.label("`.");
                        } else {
                            ui.label(" Not a parameter.");
                        }
                    });
                }
                ui.separator();
                ui.label("Set parameter:");
                ui.text_edit_singleline(&mut param_name);
                ui.text_edit_singleline(&mut param_value);
                if ui.button("Set").clicked() {
                    set_program_parameter(&param_name, &param_value);
                }
                ui.separator();
                ui.label("Delete parameter:");
                ui.text_edit_singleline(&mut to_delete);
                if ui.button("Delete").clicked() {
                    delete_program_parameter(&to_delete);
                }
                ui.separator();
                ui.label("Hash:");
                ui.monospace(&hash);
                if ui.button("Update").clicked() {
                    hash = get_hash();
                }
                ui.separator();
                ui.label("Set hash:");
                ui.text_edit_singleline(&mut hash_to_set);
                if ui.button("Set").clicked() {
                    set_hash(&hash_to_set);
                }
                ui.separator();
                ui.label("To open:");
                ui.text_edit_singleline(&mut to_open);
                ui.checkbox(&mut new_tab, "New tab");
                if ui.button("Open").clicked() {
                    link_open(&to_open, new_tab);
                }
            });
        });

        egui_macroquad::draw();

        next_frame().await
    }
}
