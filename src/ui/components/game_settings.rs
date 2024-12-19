use crate::ui::widgets::{NumericSettingInput, NumericSettingInputOutput};
use eframe::egui::{Align, Layout, Separator, Ui};

pub(crate) struct GameSettings {
    pub sens: f32,
    pub yaw: f32,
    pub increment: f32,

    sens_buffer: String,
    yaw_buffer: String,
    increment_buffer: String,

    is_sens_input_valid: bool,
    is_yaw_input_valid: bool,

    available_width: f32,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            sens: 1.8,
            yaw: 0.022,
            increment: 0.0396,

            sens_buffer: String::from("1.8"),
            yaw_buffer: String::from("0.022"),
            increment_buffer: String::from("0.0396"),

            is_sens_input_valid: bool::default(),
            is_yaw_input_valid: bool::default(),

            available_width: f32::default(),
        }
    }
}

const AMOUNT_OF_FIELDS: u16 = 3;

impl GameSettings {
    pub(crate) fn show(&mut self, ui: &mut Ui) {
        self.available_width = ui.available_width();

        let mut sens_input: Option<NumericSettingInputOutput> = None;
        let mut yaw_input: Option<NumericSettingInputOutput> = None;

        ui.with_layout(Layout::top_down(Align::Min), |ui| {
            ui.group(|ui| {
                ui.vertical_centered(|ui| {
                    ui.label("Game settings");
                });

                ui.add(Separator::default());

                ui.with_layout(Layout::left_to_right(Align::Min), |ui| {
                    sens_input = Some(
                        NumericSettingInput::<f32>::new(
                            "Sens".into(),
                            Some("x".into()),
                            self.available_width,
                            AMOUNT_OF_FIELDS,
                            false,
                            &mut self.sens_buffer,
                            &mut self.yaw,
                        )
                        .show(ui),
                    );

                    yaw_input = Some(
                        NumericSettingInput::<f32>::new(
                            "Yaw".into(),
                            Some("=".into()),
                            self.available_width,
                            AMOUNT_OF_FIELDS,
                            false,
                            &mut self.yaw_buffer,
                            &mut self.yaw,
                        )
                        .show(ui),
                    );

                    NumericSettingInput::<f32>::new(
                        "Increment".into(),
                        None,
                        self.available_width,
                        AMOUNT_OF_FIELDS,
                        true,
                        &mut self.increment_buffer,
                        &mut self.increment,
                    )
                    .interactive(false)
                    .show(ui);

                    if let Some(sens_input) = &sens_input {
                        self.is_sens_input_valid = sens_input.is_text_buffer_value_valid;
                        if let Some(yaw_input) = &yaw_input {
                            self.is_yaw_input_valid = yaw_input.is_text_buffer_value_valid;
                        }
                    }

                    if self.is_sens_input_valid && self.is_yaw_input_valid {
                        self.increment = self.sens * self.yaw;
                        self.increment_buffer = self.increment.to_string();
                    }
                });
            });
        });
    }
}
