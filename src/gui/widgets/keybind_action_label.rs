use bon::Builder;
use egui::{text::LayoutJob, Color32, Response, TextFormat, TextStyle, Ui, Widget};
use lazy_static::lazy_static;

lazy_static! {
    static ref KEYBIND_HIGHLIGHT_COLOR: Color32 =
        Color32::from_hex("#821E1E").expect("Invalid HEX");
}

#[derive(Builder)]
pub(crate) struct KeybindActionLabel<'a> {
    #[builder(start_fn)]
    keybind_text: &'a str,

    #[builder(start_fn)]
    action_text: &'a str,

    #[builder(default = 9.0)]
    size: f32,
}

impl Widget for KeybindActionLabel<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let mut font_id = ui.ctx().style().text_styles[&TextStyle::Body].to_owned();
        font_id.size = self.size;

        let mut layout_job = LayoutJob::default();

        layout_job.append(
            &(self.keybind_text.to_owned() + "  "),
            f32::default(),
            TextFormat {
                color: *KEYBIND_HIGHLIGHT_COLOR,
                font_id: font_id.clone(),

                ..Default::default()
            },
        );

        layout_job.append(
            self.action_text,
            f32::default(),
            TextFormat {
                font_id: font_id.clone(),

                ..Default::default()
            },
        );

        ui.label(layout_job)
    }
}
