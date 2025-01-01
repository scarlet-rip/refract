use crate::start;
use egui::{
    text::LayoutJob, Align, Color32, Layout, RichText, TextEdit, TextFormat, TextStyle, Ui,
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
    static ref MEASUREMENT_STATUS_HIGHLIGHT_COLOR_ACTIVE: Color32 =
        Color32::from_hex("#076A19").expect("Invalid HEX");
    static ref MEASUREMENT_STATUS_HIGHLIGHT_COLOR_INACTIVE: Color32 =
        Color32::from_hex("#821E1E").expect("Invalid HEX");
}

pub(crate) struct D360MeasurementDemo {
    pixel_360_distance: i32,
    tracking_status: bool,
    tracking_status_receiver: Receiver<bool>,
    total_movement_receiver: Receiver<i32>,
    do_360_pixel_amount_sender: Sender<u32>,
    do_360_pixels: String,

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

            is_measurement_active: false,
        }
    }
}

impl D360MeasurementDemo {
    pub(crate) fn show(&mut self, ui: &mut Ui) {
        if let Ok(pixel_360_distance) = self.total_movement_receiver.try_recv() {
            self.pixel_360_distance = pixel_360_distance;
        }

        if let Ok(tracking_status) = self.tracking_status_receiver.try_recv() {
            self.tracking_status = tracking_status;
        }

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

                        ui.toggle_value(
                            &mut self.tracking_status,
                            format!("Measured distance: {} px", self.pixel_360_distance),
                        );
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
                                let mut font_id =
                                    ui.ctx().style().text_styles[&TextStyle::Body].to_owned();
                                font_id.size = INFO_LABEL_SIZE;

                                let mut job = LayoutJob::default();

                                job.append(
                                    "ALT + M",
                                    f32::default(),
                                    TextFormat {
                                        color: *KEYBIND_HIGHLIGHT_COLOR,
                                        font_id: font_id.clone(),

                                        ..Default::default()
                                    },
                                );

                                job.append(
                                    " to perform a sweep",
                                    f32::default(),
                                    TextFormat {
                                        font_id: font_id.clone(),

                                        ..Default::default()
                                    },
                                );

                                ui.label(
                                    RichText::new("Sweep distance (px):")
                                        .size(PARTITION_INNER_LABEL_SIZE),
                                );

                                ui.label(job);
                            });

                            ui.with_layout(Layout::top_down(Align::Center), |ui| {
                                let mut font_id =
                                    ui.ctx().style().text_styles[&TextStyle::Body].to_owned();
                                font_id.size = INFO_LABEL_SIZE;

                                let mut job = LayoutJob::default();

                                job.append(
                                    "Status: ",
                                    f32::default(),
                                    TextFormat {
                                        font_id: font_id.clone(),

                                        ..Default::default()
                                    },
                                );

                                let status_color = if self.is_measurement_active {
                                    *MEASUREMENT_STATUS_HIGHLIGHT_COLOR_ACTIVE
                                } else {
                                    *MEASUREMENT_STATUS_HIGHLIGHT_COLOR_INACTIVE
                                };

                                let status_text = if self.is_measurement_active {
                                    "Active"
                                } else {
                                    "Inactive"
                                };

                                job.append(
                                    status_text,
                                    f32::default(),
                                    TextFormat {
                                        color: status_color,
                                        font_id,

                                        ..Default::default()
                                    },
                                );

                                let do_360_pixels_text_edit =
                                    ui.add(TextEdit::singleline(&mut self.do_360_pixels));

                                if do_360_pixels_text_edit.changed() {
                                    if let Ok(do_360_pixels) = self.do_360_pixels.parse::<u32>() {
                                        self.do_360_pixel_amount_sender
                                            .send(do_360_pixels)
                                            .unwrap();
                                    }
                                }

                                ui.label(job);
                            });
                        });
                    });
                });
            });
        });

        ui.separator();
    }
}
