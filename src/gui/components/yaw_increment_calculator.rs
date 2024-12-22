use super::super::widgets::NumericSettingInput;
use egui::{Align, Layout, Response, Separator, Ui};

pub(crate) struct YawIncrementCalculator {
    pub sens: f32,
    pub yaw: f32,
    pub yaw_increment: f32,

    available_width: f32,
}

impl Default for YawIncrementCalculator {
    fn default() -> Self {
        Self {
            sens: 1.8,
            yaw: 0.022,
            yaw_increment: 0.0396,

            available_width: f32::default(),
        }
    }
}

const AMOUNT_OF_FIELDS: u16 = 3;

impl YawIncrementCalculator {
    pub(crate) fn show(&mut self, ui: &mut Ui) {
        self.available_width = ui.available_width();

        let mut sens_input: Option<Response> = None;
        let mut yaw_input: Option<Response> = None;

        ui.with_layout(Layout::top_down(Align::Min), |ui| {
            ui.group(|ui| {
                ui.vertical_centered(|ui| {
                    ui.label("Yaw increment calculator");
                });

                ui.add(Separator::default());

                ui.with_layout(Layout::left_to_right(Align::Min), |ui| {
                    sens_input = Some(
                        ui.add(
                            NumericSettingInput::builder(&mut self.sens)
                                .name("Sens")
                                .separator("x")
                                .setting_box_width(self.available_width)
                                .num_total_setting_inputs(AMOUNT_OF_FIELDS)
                                .build(),
                        ),
                    );

                    yaw_input = Some(
                        ui.add(
                            NumericSettingInput::builder(&mut self.yaw)
                                .name("Yaw")
                                .separator("=")
                                .setting_box_width(self.available_width)
                                .num_total_setting_inputs(AMOUNT_OF_FIELDS)
                                .build(),
                        ),
                    );

                    ui.add(
                        NumericSettingInput::builder(&mut self.yaw_increment)
                            .name("Yaw Increment")
                            .setting_box_width(self.available_width)
                            .num_total_setting_inputs(AMOUNT_OF_FIELDS)
                            .is_last(true)
                            .interactive(false)
                            .build(),
                    );

                    self.update_yaw_increment_if_needed(
                        &[sens_input.as_ref(), yaw_input.as_ref()],
                        self.sens,
                        self.yaw,
                    );
                });
            });
        });
    }

    fn update_yaw_increment_if_needed(
        &mut self,
        inputs: &[Option<&Response>],
        sens: f32,
        yaw: f32,
    ) {
        for input in inputs.iter().flatten() {
            if input.changed() && sens > 0.0 && yaw > 0.0 {
                self.yaw_increment = calculate_yaw_increment(sens, yaw);
            }
        }
    }
}

fn calculate_yaw_increment(sens: f32, yaw: f32) -> f32 {
    sens * yaw
}
