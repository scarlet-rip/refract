use super::WidgetState;
use egui::{Response, TextEdit, Ui, Widget};
use std::{fmt::Display, str::FromStr};

pub(crate) trait Num: FromStr + Display + Default {}

impl Num for i8 {}
impl Num for i16 {}
impl Num for i32 {}
impl Num for i64 {}
impl Num for i128 {}
impl Num for isize {}
impl Num for u8 {}
impl Num for u16 {}
impl Num for u32 {}
impl Num for u64 {}
impl Num for u128 {}
impl Num for usize {}
impl Num for f32 {}
impl Num for f64 {}

impl WidgetState for NumericInputState {}

#[derive(Default, Clone)]
pub(crate) struct NumericInputState {
    text_buffer: String,
    is_text_buffer_valid: bool,
}

pub(crate) struct NumericInput<'b, N: Num> {
    id_salt: &'b str,
    desired_width: f32,
    value_buffer: &'b mut N,
    is_interactive: bool,
}

impl<N: Num> Widget for NumericInput<'_, N> {
    fn ui(self, ui: &mut Ui) -> Response {
        let mut state = NumericInputState::load_or_default(ui, self.id_salt);

        if state.text_buffer.is_empty() || !self.is_interactive {
            state.text_buffer = self.value_buffer.to_string();
            state.is_text_buffer_valid = true;
        }

        let text_color = if state.is_text_buffer_valid {
            ui.style().visuals.noninteractive().text_color()
        } else {
            ui.style().visuals.error_fg_color
        };

        let text_edit = ui.add(
            TextEdit::singleline(&mut state.text_buffer)
                .desired_width(self.desired_width)
                .text_color(text_color)
                .interactive(self.is_interactive),
        );

        if text_edit.changed() {
            if state.text_buffer.is_empty() {
                 state.is_text_buffer_valid = true;

                return text_edit
            };

            let parsed_possible_text = state.text_buffer.as_str().parse::<N>();

            state.is_text_buffer_valid = parsed_possible_text.is_ok();

            if let Ok(parsed_text) = parsed_possible_text {
                *self.value_buffer = parsed_text;
            } else {
                *self.value_buffer = N::default();
            }
        }

        state.save_state(ui, self.id_salt);

        text_edit
    }
}

impl<'b, N: Num> NumericInput<'b, N> {
    pub fn new(id_salt: &'b str, value_buffer: &'b mut N) -> Self {
        Self {
            id_salt,
            value_buffer,
            desired_width: f32::MAX,
            is_interactive: true,
        }
    }

    pub fn desired_width(mut self, width: f32) -> Self {
        self.desired_width = width;

        self
    }

    #[allow(unused)]
    pub fn interactive(mut self, is_interactive: bool) -> Self {
        self.is_interactive = is_interactive;

        self
    }
}
