use eframe::egui::{TextEdit, Ui};

fn add_input_field(
    ui: &mut Ui,
    text_buffer: &mut String,
    value_buffer: &mut f32,
    desired_width: f32,
) {
    let text_buffer_clone = text_buffer.clone();
    let input_box = ui.add(
        TextEdit::singleline(text_buffer)
            .desired_width(desired_width)
            .text_color(if text_buffer_clone.parse::<f32>().is_err() {
                ui.style().visuals.error_fg_color
            } else {
                ui.style().visuals.noninteractive().text_color()
            }),
    );

    if input_box.changed() {
        if let Ok(value) = text_buffer.parse::<f32>() {
            *value_buffer = value;
        }
    }
}

pub(crate) struct GameSettings {
    pub sens: f32,
    sens_text_buffer: String,

    pub yaw: f32,
    yaw_text_buffer: String,

    pub increment: f32,
    increment_text_buffer: String,

    seperator_widths: Vec<f32>,
    label_widths: Vec<f32>,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            sens: 1.8,
            sens_text_buffer: String::from("1.8"),

            yaw: 0.022,
            yaw_text_buffer: String::from("0.022"),

            increment: 0.0396,
            increment_text_buffer: String::from("0.0396"),

            seperator_widths: vec![],
            label_widths: vec![],
        }
    }
}

impl GameSettings {
    pub(crate) fn show(&mut self, ui: &mut Ui) {
        let max_width = ui.available_width();
        let input_field_width = max_width / 3.0; // 3 options

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

            self.label_widths.push(sens_width);
            self.label_widths.push(yaw_width);
        }

        ui.group(|ui| {
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.label("Sens");
                    ui.horizontal(|ui| {
                        let seperator_width = self.seperator_widths.first().unwrap_or(&0.0);
                        let label_width = self.label_widths.first().unwrap_or(&0.0);

                        add_input_field(
                            ui,
                            &mut self.sens_text_buffer,
                            &mut self.sens,
                            input_field_width - seperator_width - label_width,
                        );

                        ui.label("x");
                    });
                });

                ui.vertical(|ui| {
                    ui.label("Yaw");

                    ui.horizontal(|ui| {
                        let seperator_width = self.seperator_widths.get(1).unwrap_or(&0.0);
                        let label_width = self.label_widths.get(1).unwrap_or(&0.0);

                        add_input_field(
                            ui,
                            &mut self.yaw_text_buffer,
                            &mut self.yaw,
                            input_field_width - seperator_width - label_width,
                        );

                        ui.label("=");
                    });
                });

                ui.vertical(|ui| {
                    ui.label("Increment");
                    add_input_field(
                        ui,
                        &mut self.increment_text_buffer,
                        &mut self.increment,
                        input_field_width,
                    );
                });
            });
        });

        self.increment = self.sens * self.yaw;

        self.increment_text_buffer.clear();
        self.increment_text_buffer.push_str(&self.increment.to_string());
    }
}

// Works
// Stuff to do/fix (that I'm aware of, most likely there are more):
// Disable input in increment field
// Dont update increment if one of inputs have invalid values visible
// Center the labels & inputs possibly idk how it'd look like but give it a try
// REFACTOR PLEASE THE CODE SUCKS RN
// ALSO if possible, find a way to adjust the width of input field without workarounds
// by that I mean finding the width of label & separator without creating new labels
// lastly, I hardcoded the amount of options/fields here let input_field_width = max_width / 3.0;
// get rid of that
// add a label above the group that says "Game settings" or sum like dat
