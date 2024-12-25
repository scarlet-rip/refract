use bon::Builder;
use egui::{Align, Color32, Label, Layout, Response, RichText, Ui, Widget};
use lazy_static::lazy_static;

const PROJECT_TITLE_SIZE: f32 = 14.5;
const INFO_TITLE_SIZE: f32 = 9.0;

lazy_static! {
    static ref PROJECT_TITLE_COLOR: Color32 = Color32::from_hex("#6b0707").expect("Invalid HEX");
    static ref INFO_TITLE_COLOR: Color32 = Color32::from_hex("#3a3737").expect("Invalid HEX");
}

const SEPARATOR_OFFSET_BOTTOM: f32 = -8.0;
const PROJECT_TITLE_X_OFFSET: f32 = -4.0;
const ASSIGNED_ENTITY_TITLE_SPACING: f32 = -1.0;

#[derive(Builder, Default)]
pub(crate) struct MainHeader<'a> {
    project_id: &'a str,
    project_codename: &'a str,
    assigned_entity: &'a str,
}

impl Widget for MainHeader<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.horizontal(|ui| {
            ui.columns(3, |cols| {
                cols[0].with_layout(Layout::left_to_right(Align::TOP), |ui| {
                    let package_version = env!("CARGO_PKG_VERSION");

                    ui.label(
                        RichText::new(format!("v{package_version}"))
                            .size(INFO_TITLE_SIZE)
                            .color(*INFO_TITLE_COLOR),
                    )
                });

                cols[1].with_layout(Layout::top_down(Align::Center), |ui| {
                    ui.spacing_mut().item_spacing.y = ASSIGNED_ENTITY_TITLE_SPACING;

                    ui.add_space(PROJECT_TITLE_X_OFFSET);

                    ui.add(
                        Label::new(
                            RichText::new(format!(r#"Project "{}" "#, self.project_codename))
                                .size(PROJECT_TITLE_SIZE)
                                .color(*PROJECT_TITLE_COLOR),
                        )
                        .extend(),
                    );

                    ui.label(
                        RichText::new(self.assigned_entity)
                            .size(INFO_TITLE_SIZE)
                            .color(*INFO_TITLE_COLOR),
                    );
                });

                cols[2].with_layout(Layout::right_to_left(Align::TOP), |ui| {
                    ui.label(
                        RichText::new(self.project_id)
                            .size(INFO_TITLE_SIZE)
                            .color(*INFO_TITLE_COLOR),
                    )
                });
            });
        });

        ui.add_space(SEPARATOR_OFFSET_BOTTOM);

        ui.add(egui::widgets::Separator::default().horizontal())
    }
}
