use super::{Num, NumericInput, WidgetState};
use bon::Builder;
use egui::{Align, Color32, Context, Layout, Response, TextStyle, Ui, Widget, WidgetText};

impl WidgetState for NumericSettingInputState {}
#[derive(Default, Clone)]
struct NumericSettingInputState {
    width: f32,
    cached_input_setting_box_width: f32,
}

#[derive(Builder)]
pub(crate) struct NumericSettingInput<'b, N: Num> {
    #[builder(start_fn)]
    value: &'b mut N,

    #[builder(into)]
    separator: Option<WidgetText>,

    #[builder(into)]
    name: WidgetText,

    #[builder(name = setting_box_width)]
    input_setting_box_width: f32,

    #[builder(name = num_total_setting_inputs)]
    input_setting_box_amount_of_fields: u16,

    #[builder(default = false)]
    #[builder(name = is_last)]
    is_last: bool,

    #[builder(default = true)]
    #[builder(name = interactive)]
    is_interactive: bool,
}

impl<N: Num> Widget for NumericSettingInput<'_, N> {
    fn ui(self, ui: &mut Ui) -> Response {
        let owned_name = self.name.to_owned();
        let id_salt = owned_name.text();

        let mut state = NumericSettingInputState::load_or_default(ui, id_salt);

        if state.cached_input_setting_box_width != self.input_setting_box_width {
            self.calculate_input_field_width(ui, &mut state);
        }

        let response = ui
            .with_layout(Layout::top_down(Align::Min), |ui| {
                ui.label(self.name);

                ui.horizontal(|ui| {
                    let input = ui.add(
                        NumericInput::builder(self.value)
                            .interactive(self.is_interactive)
                            .desired_width(state.width)
                            .id_salt(id_salt)
                            .build(),
                    );

                    if let Some(separator) = self.separator {
                        ui.label(separator);
                    }

                    input
                })
            })
            .inner
            .inner;

        state.save_state(ui, id_salt);

        response
    }
}

impl<N: Num> NumericSettingInput<'_, N> {
    fn calculate_input_field_width(&self, ui: &mut Ui, state: &mut NumericSettingInputState) {
        let base_input_field_width =
            self.input_setting_box_width / self.input_setting_box_amount_of_fields as f32;

        if !self.is_last {
            let mut separator_width: f32 = 0.0;

            if let Some(separator) = &self.separator {
                separator_width = calculate_text_width(ui.ctx(), separator.text(), TextStyle::Body);
            }

            let label_width = calculate_text_width(ui.ctx(), self.name.text(), TextStyle::Body);

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
