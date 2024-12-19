use eframe::egui::{Response, TextBuffer, TextEdit, Ui, Widget};
use std::{fmt::Display, str::FromStr};

pub(crate) trait Num: FromStr + Display + Copy {}

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

pub(crate) struct NumericInput<'t, T: Num> {
    desired_width: f32,
    text_buffer: &'t mut dyn TextBuffer,
    value_buffer: &'t mut T,
    is_interactive: bool,
}

pub(crate) struct NumericInputOutput {
    pub response: Response,
    pub is_buffer_valid: bool,
}

impl<T: Num> NumericInput<'_, T> {
    pub fn show(self, ui: &mut Ui) -> NumericInputOutput {
        let is_buffer_valid = self.text_buffer.as_str().parse::<T>().is_ok();
        let text_color = if is_buffer_valid {
            ui.style().visuals.noninteractive().text_color()
        } else {
            ui.style().visuals.error_fg_color
        };

        let text_edit = ui.add(
            TextEdit::singleline(self.text_buffer)
                .desired_width(self.desired_width)
                .text_color(text_color)
                .interactive(self.is_interactive),
        );

        if text_edit.changed() {
            if let Ok(value) = self.text_buffer.as_str().parse::<T>() {
                *self.value_buffer = value;
            }
        }

        NumericInputOutput {
            response: text_edit,
            is_buffer_valid,
        }
    }
}

impl<T: Num> Widget for NumericInput<'_, T> {
    fn ui(self, ui: &mut Ui) -> Response {
        self.show(ui).response
    }
}

impl<'t, T: Num> NumericInput<'t, T> {
    pub fn new(value_buffer: &'t mut T, text_buffer: &'t mut String) -> Self {
        Self {
            text_buffer,
            value_buffer,
            desired_width: f32::default(),
            is_interactive: true,
        }
    }

    pub fn desired_width(mut self, width: f32) -> Self {
        self.desired_width = width;

        self
    }

    pub fn interactive(mut self, is_interactive: bool) -> Self {
        self.is_interactive = is_interactive;

        self
    }
}
