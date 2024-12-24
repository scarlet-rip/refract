use egui::{TextEdit, Ui};

#[derive(Default)]
pub(crate) struct SensConversion {
    original_in_game_sensitivity: String,
    original_pixel_360: String,
    target_pixel_360: String,
    converted_sensitivity: f64,
}

impl SensConversion {
    pub(crate) fn show(&mut self, ui: &mut Ui) {
        ui.vertical(|ui| {
            ui.heading("Sensitivity Conversion");

            ui.label("in-game sensitivity");
            ui.add(TextEdit::singleline(&mut self.original_in_game_sensitivity));

            ui.label("original pixel 360");
            ui.add(TextEdit::singleline(&mut self.original_pixel_360));

            ui.label("target pixel 360");
            ui.add(TextEdit::singleline(&mut self.target_pixel_360));

            if ui.button("Convert").clicked() {
                let original_in_game_sensitivity =
                    self.original_in_game_sensitivity.parse::<f64>().unwrap();
                let original_pixel_360 = self.original_pixel_360.parse::<f64>().unwrap();
                let target_pixel_360 = self.target_pixel_360.parse::<f64>().unwrap();

                let d360_difference = target_pixel_360 / original_pixel_360;

                self.converted_sensitivity = original_in_game_sensitivity * d360_difference;
            }

            ui.label(format!("{}", self.converted_sensitivity));
        });
    }
}
