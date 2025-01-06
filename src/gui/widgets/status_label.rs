use bon::Builder;
use egui::{text::LayoutJob, Color32, Response, TextFormat, TextStyle, Ui, Widget};
use lazy_static::lazy_static;

lazy_static! {
    static ref STATUS_HIGHLIGHT_COLOR_ACTIVE: Color32 =
        Color32::from_hex("#076A19").expect("Invalid HEX");
    static ref STATUS_HIGHLIGHT_COLOR_INACTIVE: Color32 =
        Color32::from_hex("#821E1E").expect("Invalid HEX");
}

#[derive(Builder)]
pub(crate) struct StatusLabel {
    #[builder(start_fn)]
    status: bool,

    #[builder(default = 9.0)]
    size: f32,
}

impl Widget for StatusLabel {
    fn ui(self, ui: &mut Ui) -> Response {
        let mut font_id = ui.ctx().style().text_styles[&TextStyle::Body].to_owned();
        font_id.size = self.size;

        let mut layout_job = LayoutJob::default();

        layout_job.append(
            "Status: ",
            f32::default(),
            TextFormat {
                font_id: font_id.clone(),

                ..Default::default()
            },
        );

        let status_color = if self.status {
            *STATUS_HIGHLIGHT_COLOR_ACTIVE
        } else {
            *STATUS_HIGHLIGHT_COLOR_INACTIVE
        };

        let status_text = if self.status { "Active" } else { "Inactive" };

        layout_job.append(
            status_text,
            f32::default(),
            TextFormat {
                color: status_color,
                font_id,

                ..Default::default()
            },
        );

        ui.label(layout_job)
    }
}
