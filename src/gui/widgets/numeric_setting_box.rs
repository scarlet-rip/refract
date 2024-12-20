use super::{Num, NumericInput, WidgetState};
use egui::{Align, Color32, Context, Layout, Response, TextStyle, Ui, Widget};

impl WidgetState for NumericSettingInputState {}
#[derive(Default, Clone)]
struct NumericSettingInputState {
    width: f32,
    cached_input_setting_box_width: f32,
}

pub(crate) struct NumericSettingInput<'b, N: Num> {
    input_setting_box_width: f32,
    input_setting_box_amount_of_fields: u16,
    value: &'b mut N,
    separator: Option<String>,
    name: String,
    is_last: bool,
    is_interactive: bool,
}

impl<N: Num> Widget for NumericSettingInput<'_, N> {
    fn ui(self, ui: &mut Ui) -> Response {
        let mut state = NumericSettingInputState::load_or_default(ui, &self.name);

        if state.cached_input_setting_box_width != self.input_setting_box_width {
            self.calculate_input_field_width(ui, &mut state);
        }

        let response = ui
            .with_layout(Layout::top_down(Align::Min), |ui| {
                ui.label(&self.name);

                ui.horizontal(|ui| {
                    let input = ui.add(
                        NumericInput::new(&self.name, self.value)
                            .desired_width(state.width)
                            .interactive(self.is_interactive),
                    );

                    if let Some(separator) = &self.separator {
                        ui.label(separator);
                    }

                    input
                })
            })
            .inner
            .inner;

        state.save_state(ui, &self.name);

        response
    }
}

impl<'b, N: Num> NumericSettingInput<'b, N> {
    pub fn new(
        name: String,
        separator: Option<String>,
        input_setting_box_width: f32,
        input_setting_box_amount_of_fields: u16,
        is_last: bool,
        value: &'b mut N,
    ) -> Self {
        Self {
            input_setting_box_width,
            input_setting_box_amount_of_fields,
            value,
            separator,
            name,
            is_last,
            is_interactive: true,
        }
    }

    pub fn interactive(mut self, is_interactive: bool) -> Self {
        self.is_interactive = is_interactive;
        self
    }

    fn calculate_input_field_width(&self, ui: &mut Ui, state: &mut NumericSettingInputState) {
        let base_input_field_width =
            self.input_setting_box_width / self.input_setting_box_amount_of_fields as f32;

        if !self.is_last {
            let mut separator_width: f32 = 0.0;

            if let Some(separator) = &self.separator {
                separator_width = calculate_text_width(ui.ctx(), separator, TextStyle::Body);
            }

            let label_width = calculate_text_width(ui.ctx(), &self.name, TextStyle::Body);

            state.width = base_input_field_width - separator_width - label_width;
        } else {
            state.width = base_input_field_width;
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
