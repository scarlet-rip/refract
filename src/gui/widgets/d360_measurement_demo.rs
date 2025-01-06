use super::StatusLabel;
use crate::start;
use egui::{
    text::LayoutJob, Align, Color32, Label, Layout, RichText, TextEdit, TextFormat, TextStyle, Ui,
};
use lazy_static::lazy_static;
use std::sync::mpsc::{Receiver, Sender};

const GROUP_HEADER_SIZE: f32 = 14.0;
const PARTITION_HEADER_SIZE: f32 = 14.0;
const PARTITION_INNER_LABEL_SIZE: f32 = 12.5;

const INFO_LABEL_SIZE: f32 = 9.0;

lazy_static! {
    static ref GROUP_HEADER_SIZE_COLOR: Color32 =
        Color32::from_hex("#6b0707").expect("Invalid HEX");
    static ref PARTITION_HEADER_COLOR: Color32 = Color32::from_hex("#6b0707").expect("Invalid HEX");
    static ref KEYBIND_HIGHLIGHT_COLOR: Color32 =
        Color32::from_hex("#821E1E").expect("Invalid HEX");
    static ref STATUS_HIGHLIGHT_COLOR_ACTIVE: Color32 =
        Color32::from_hex("#076A19").expect("Invalid HEX");
    static ref STATUS_HIGHLIGHT_COLOR_INACTIVE: Color32 =
        Color32::from_hex("#821E1E").expect("Invalid HEX");
}

pub(crate) struct D360MeasurementDemo {
    pixel_360_distance: i32,
    tracking_status: bool,
    tracking_status_receiver: Receiver<bool>,
    total_movement_receiver: Receiver<i32>,
    do_360_pixel_amount_sender: Sender<u32>,
    do_360_pixels: String,

    is_sweep_active: bool,
    is_measurement_active: bool,
}

impl Default for D360MeasurementDemo {
    fn default() -> Self {
        let (tracking_status_receiver, total_movement_receiver, do_360_pixel_amount_sender) =
            start();
        Self {
            pixel_360_distance: 0,
            tracking_status: false,
            tracking_status_receiver,
            total_movement_receiver,
            do_360_pixel_amount_sender,
            do_360_pixels: String::default(),

            is_sweep_active: false,
            is_measurement_active: false,
        }
    }
}

impl D360MeasurementDemo {
    pub(crate) fn show(&mut self, ui: &mut Ui) {
        ui.with_layout(Layout::top_down(Align::Center), |ui| {
            ui.label(
                RichText::new("Yaw Sweep")
                    .size(GROUP_HEADER_SIZE)
                    .color(*GROUP_HEADER_SIZE_COLOR),
            );

            ui.columns(2, |cols| {
                cols[0].with_layout(Layout::left_to_right(Align::Min), |ui| {
                    ui.with_layout(Layout::top_down(Align::Center), |ui| {
                        ui.label(
                            RichText::new("Measurement")
                                .size(PARTITION_HEADER_SIZE)
                                .color(*PARTITION_HEADER_COLOR),
                        );

                        ui.label(format!("Measured distance: {} px", self.pixel_360_distance));

                        ui.columns(2, |cols| {
                            cols[0].with_layout(Layout::right_to_left(Align::Min), |ui| {
                                ui.add(create_keybind_action_label(ui, "ALT + X", "to measure"));
                            });

                            cols[1].add(
                                StatusLabel::builder(self.is_measurement_active)
                                    .size(INFO_LABEL_SIZE)
                                    .build(),
                            );
                        });
                    });
                });

                cols[1].with_layout(Layout::right_to_left(Align::Min), |ui| {
                    ui.with_layout(Layout::top_down(Align::Center), |ui| {
                        ui.label(
                            RichText::new("Execution")
                                .size(PARTITION_HEADER_SIZE)
                                .color(*PARTITION_HEADER_COLOR),
                        );

                        ui.with_layout(Layout::left_to_right(Align::Min), |ui| {
                            ui.with_layout(Layout::top_down(Align::Min), |ui| {
                                ui.label(
                                    RichText::new("Sweep distance (px):")
                                        .size(PARTITION_INNER_LABEL_SIZE),
                                );

                                ui.add(create_keybind_action_label(
                                    ui,
                                    "ALT + M",
                                    "to perform a sweep",
                                ));
                            });

                            ui.with_layout(Layout::top_down(Align::Center), |ui| {
                                let do_360_pixels_text_edit =
                                    ui.add(TextEdit::singleline(&mut self.do_360_pixels));

                                if do_360_pixels_text_edit.changed() {
                                    if let Ok(do_360_pixels) = self.do_360_pixels.parse::<u32>() {
                                        self.do_360_pixel_amount_sender
                                            .send(do_360_pixels)
                                            .unwrap();
                                    }
                                }

                                ui.add(
                                    StatusLabel::builder(self.is_sweep_active)
                                        .size(INFO_LABEL_SIZE)
                                        .build(),
                                );
                            });
                        });
                    });
                });
            });
        });

        ui.separator();
    }
}

fn create_keybind_action_label(ui: &Ui, keybind_text: &str, action_text: &str) -> Label {
    let mut font_id = ui.ctx().style().text_styles[&TextStyle::Body].to_owned();
    font_id.size = INFO_LABEL_SIZE;

    let mut job = LayoutJob::default();

    job.append(
        &(keybind_text.to_owned() + "  "),
        f32::default(),
        TextFormat {
            color: *KEYBIND_HIGHLIGHT_COLOR,
            font_id: font_id.clone(),

            ..Default::default()
        },
    );

    job.append(
        action_text,
        f32::default(),
        TextFormat {
            font_id: font_id.clone(),

            ..Default::default()
        },
    );

    Label::new(job)
}
