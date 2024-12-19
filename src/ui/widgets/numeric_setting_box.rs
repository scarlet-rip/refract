use crate::ui::widgets::{Num, NumericInput};
use eframe::egui::{Align, Color32, Context, Layout, Response, TextStyle, Ui, Widget};

// TODO:
// use states, some fields should be set every render
// the displayed decimal precision might not be accurate, take a look at that
// refactor

#[derive(Debug)]
pub(crate) struct NumericSettingInput<'a, N: Num> {
    input_setting_box_width: f32,
    input_setting_box_amount_of_fields: u16,
    width: f32,
    value: &'a mut N,
    text_buffer: &'a mut String,
    separator: Option<String>,
    name: String,
    is_last: bool,
    is_interactive: bool,
}

pub(crate) struct NumericSettingInputOutput {
    pub response: Response,
    pub is_text_buffer_value_valid: bool,
}

impl<N: Num> Widget for NumericSettingInput<'_, N> {
    fn ui(self, ui: &mut Ui) -> Response {
        self.show(ui).response
    }
}

impl<'t, N: Num> NumericSettingInput<'t, N> {
    pub fn new(
        name: String,
        separator: Option<String>,
        input_setting_box_width: f32,
        input_setting_box_amount_of_fields: u16,
        is_last: bool,
        text_buffer: &'t mut String,
        value: &'t mut N,
    ) -> Self {
        Self {
            input_setting_box_width,
            input_setting_box_amount_of_fields,
            width: f32::default(),
            value,
            text_buffer,
            separator,
            name,
            is_last,
            is_interactive: true,
        }
    }

    pub fn show(mut self, ui: &mut Ui) -> NumericSettingInputOutput {
        self.calculate_input_field_width(ui);

        let mut is_text_buffer_value_valid = false;

        let response = ui
            .with_layout(Layout::top_down(Align::Min), |ui| {
                ui.label(self.name);

                ui.horizontal(|ui| {
                    let input = NumericInput::new(self.value, self.text_buffer)
                        .desired_width(self.width)
                        .interactive(self.is_interactive)
                        .show(ui);

                    if let Some(separator) = self.separator {
                        ui.label(separator);
                    }

                    is_text_buffer_value_valid = input.is_buffer_valid;

                    input
                })
            })
            .response;

        NumericSettingInputOutput {
            response,
            is_text_buffer_value_valid,
        }
    }

    pub fn interactive(mut self, is_interactive: bool) -> Self {
        self.is_interactive = is_interactive;

        self
    }

    fn calculate_input_field_width(&mut self, ui: &mut Ui) {
        let base_input_field_width =
            self.input_setting_box_width / self.input_setting_box_amount_of_fields as f32;

        if !self.is_last {
            let mut separator_width: f32 = 0.0;

            if let Some(separator) = &self.separator {
                separator_width = calculate_text_width(ui.ctx(), separator, TextStyle::Body);
            }

            let label_width = calculate_text_width(ui.ctx(), &self.name, TextStyle::Body);

            self.width = base_input_field_width - separator_width - label_width;
        } else {
            self.width = base_input_field_width;
        }
    }
}

fn calculate_text_width(ctx: &Context, text: &str, text_style: TextStyle) -> f32 {
    let font_id = text_style.resolve(&ctx.style());

    ctx.fonts(|fonts| {
        let galley = fonts.layout_no_wrap(text.into(), font_id, Color32::default());

        galley.rect.width()
    })
}
