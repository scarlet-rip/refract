use crate::ui::widgets::{NumericInput, NumericInputOutput};
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
    seperator_widths: Vec<f32>,
    label_widths: Vec<f32>,
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
            seperator_widths: vec![],
            label_widths: vec![],
        }
    }
}

const AMOUNT_OF_FIELDS: u16 = 3;

impl GameSettings {
    fn calculate_base_seperator_and_label_widths(&mut self, ui: &mut Ui) {
        if self.seperator_widths.is_empty() {
            let x_label = ui.label("x");
            let x_width = x_label.rect.width();

            let equals_label = ui.label("=");
            let equals_width = equals_label.rect.width();

            self.seperator_widths.push(x_width);
            self.seperator_widths.push(equals_width);
        }

        if self.label_widths.is_empty() {
            let sens_label = ui.label("Sens");
            let sens_width = sens_label.rect.width();

            let yaw_label = ui.label("Yaw");
            let yaw_width = yaw_label.rect.width();

            let increment_label = ui.label("Increment");
            let increment_width = increment_label.rect.width();

            self.label_widths.push(sens_width);
            self.label_widths.push(yaw_width);
            self.label_widths.push(increment_width);
        }
    }

    fn calculate_input_field_width(
        &self,
        amount_of_fields: u16,
        field_num: usize,
        is_last: bool,
    ) -> f32 {
        let field_num = field_num - 1;
        let base_input_field_width = self.available_width / amount_of_fields as f32;

        if !is_last {
            let seperator_width = self.seperator_widths.get(field_num).unwrap_or(&0.0);
            let label_width = self.label_widths.get(field_num).unwrap_or(&0.0);

            base_input_field_width - seperator_width - label_width
        } else {
            base_input_field_width
        }
    }

    pub(crate) fn show(&mut self, ui: &mut Ui) {
        self.available_width = ui.available_width();
        self.calculate_base_seperator_and_label_widths(ui);

        let mut sens_input: Option<NumericInputOutput> = None;
        let mut yaw_input: Option<NumericInputOutput> = None;

        ui.with_layout(Layout::top_down(Align::Min), |ui| {
            ui.group(|ui| {
                ui.vertical_centered(|ui| {
                    ui.label("Game settings");
                });

                ui.add(Separator::default());

                ui.with_layout(Layout::left_to_right(Align::Min), |ui| {
                    ui.with_layout(Layout::top_down(Align::Min), |ui| {
                        ui.label("Sens");

                        ui.horizontal(|ui| {
                            let input_field_width =
                                self.calculate_input_field_width(AMOUNT_OF_FIELDS, 1, false);

                            sens_input = Some(
                                NumericInput::new(&mut self.sens, &mut self.sens_buffer)
                                    .desired_width(input_field_width)
                                    .show(ui),
                            );

                            ui.label("x");
                        });
                    });

                    ui.with_layout(Layout::top_down(Align::Min), |ui| {
                        ui.label("Yaw");

                        ui.horizontal(|ui| {
                            let input_field_width =
                                self.calculate_input_field_width(AMOUNT_OF_FIELDS, 2, false);

                            yaw_input = Some(
                                NumericInput::new(&mut self.yaw, &mut self.yaw_buffer)
                                    .desired_width(input_field_width)
                                    .show(ui),
                            );

                            ui.label("=");
                        });
                    });

                    ui.with_layout(Layout::top_down(Align::Min), |ui| {
                        ui.label("Increment");

                        let input_field_width =
                            self.calculate_input_field_width(AMOUNT_OF_FIELDS, 3, true);

                        NumericInput::new(&mut self.increment, &mut self.increment_buffer)
                            .desired_width(input_field_width)
                            .interactive(false)
                            .show(ui)
                    });

                    if let Some(sens_input) = sens_input {
                        self.is_sens_input_valid = sens_input.is_buffer_valid;
                        if let Some(yaw_input) = yaw_input {
                            self.is_yaw_input_valid = yaw_input.is_buffer_valid;
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

// Works
// Stuff to do/fix (that I'm aware of, most likely there are more):
// Disable input in increment field
// Center the labels & inputs possibly idk how it'd look like but give it a try
// REFACTOR PLEASE THE CODE SUCKS RN
// ALSO if possible, find a way to adjust the width of input field without workarounds
// by that I mean finding the width of label & separator without creating new labels
