use egui::{TextEdit, Ui};
use scarlet_frame::egui::Group;

#[derive(Default)]
pub(crate) struct SensitivityConversionDemo {
    original_in_game_sensitivity: String,
    original_pixel_360: String,
    target_pixel_360: String,
    converted_sensitivity: f64,
}

impl SensitivityConversionDemo {
    pub(crate) fn show(&mut self, ui: &mut Ui) {
        Group::new("sensitivity-conversion", "9slice-test.png").show(ui, |ui| {
            ui.vertical(|ui| {
                ui.heading("Sensitivity Conversion");

                ui.label("in-game sensitivity");
                ui.add(TextEdit::singleline(&mut self.original_in_game_sensitivity));

                ui.label("original pixel 360");
                ui.add(TextEdit::singleline(&mut self.original_pixel_360));

                ui.label("target pixel 360");
                ui.add(TextEdit::singleline(&mut self.target_pixel_360));

                if ui.button("Convert").clicked() {
                    if let Ok(original_in_game_sensitivity) =
                        self.original_in_game_sensitivity.parse::<f64>()
                    {
                        if let Ok(original_pixel_360) = self.original_pixel_360.parse::<f64>() {
                            if let Ok(target_pixel_360) = self.target_pixel_360.parse::<f64>() {
                                let d360_difference = target_pixel_360 / original_pixel_360;

                                self.converted_sensitivity =
                                    original_in_game_sensitivity * d360_difference;
                            }
                        }
                    }
                }

                ui.label(self.converted_sensitivity.to_string());

                ui.separator();
            });
        });
    }
}
