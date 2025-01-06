use super::{KeybindActionLabel, StatusLabel, WidgetState};
use crate::start;
use egui::{Align, Color32, Layout, Response, RichText, TextEdit, Ui, Widget};
use lazy_static::lazy_static;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex};

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

#[derive(Clone)]
struct YawSweepState {
    measurement_status_receiver: Arc<Mutex<Receiver<bool>>>,
    sweep_execution_status_receiver: Arc<Mutex<Receiver<bool>>>,
    sweep_distance_measurement_receiver: Arc<Mutex<Receiver<i32>>>,
    sweep_execution_distance_sender: Arc<Mutex<Sender<u32>>>,

    measurement_status: bool,
    measured_sweep_distance: i32,

    sweep_execution_distance_buffer: String,
    sweep_execution_status: bool,
}

impl Default for YawSweepState {
    fn default() -> Self {
        let (
            measurement_status_receiver,
            sweep_distance_measurement_receiver,
            sweep_execution_distance_sender,
            sweep_execution_status_receiver,
        ) = start();

        YawSweepState {
            measurement_status_receiver: Arc::new(Mutex::new(measurement_status_receiver)),
            sweep_distance_measurement_receiver: Arc::new(Mutex::new(
                sweep_distance_measurement_receiver,
            )),
            sweep_execution_distance_sender: Arc::new(Mutex::new(sweep_execution_distance_sender)),
            sweep_execution_status_receiver: Arc::new(Mutex::new(sweep_execution_status_receiver)),

            measurement_status: false,
            measured_sweep_distance: 0,

            sweep_execution_distance_buffer: String::default(),
            sweep_execution_status: false,
        }
    }
}

impl WidgetState for YawSweepState {}

#[derive(Default)]
pub(crate) struct YawSweep {}

impl Widget for YawSweep {
    fn ui(self, ui: &mut Ui) -> Response {
        let mut state = YawSweepState::load_or_default(ui, "");

        let (
            measurement_status_receiver,
            sweep_distance_measurement_receiver,
            sweep_execution_distance_sender,
            sweep_execution_status_receiver,
        ) = (
            state.measurement_status_receiver.lock().unwrap(),
            state.sweep_distance_measurement_receiver.lock().unwrap(),
            state.sweep_execution_distance_sender.lock().unwrap(),
            state.sweep_execution_status_receiver.lock().unwrap(),
        );

        if let Ok(measurement_status) = measurement_status_receiver.try_recv() {
            state.measurement_status = measurement_status;
        }

        if let Ok(sweep_execution_status) = sweep_execution_status_receiver.try_recv() {
            state.sweep_execution_status = sweep_execution_status;
        }

        if let Ok(sweep_distance_measurement) = sweep_distance_measurement_receiver.try_recv() {
            state.measured_sweep_distance = sweep_distance_measurement;
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

                        ui.label(format!(
                            "Measured distance: {} px",
                            state.measured_sweep_distance
                        ));

                        ui.columns(2, |cols| {
                            cols[0].with_layout(Layout::right_to_left(Align::Min), |ui| {
                                ui.add(
                                    KeybindActionLabel::builder("ALT + M", "to measure").build(),
                                );
                            });

                            cols[1].add(
                                StatusLabel::builder(state.measurement_status)
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

                                ui.add(
                                    KeybindActionLabel::builder("ALT + X", "to perform a sweep")
                                        .build(),
                                );
                            });

                            ui.with_layout(Layout::top_down(Align::Center), |ui| {
                                let do_360_pixels_text_edit = ui.add(TextEdit::singleline(
                                    &mut state.sweep_execution_distance_buffer,
                                ));

                                if do_360_pixels_text_edit.changed() {
                                    if let Ok(do_360_pixels) =
                                        state.sweep_execution_distance_buffer.parse::<u32>()
                                    {
                                        sweep_execution_distance_sender
                                            .send(do_360_pixels)
                                            .unwrap();
                                    }
                                }

                                ui.add(
                                    StatusLabel::builder(state.sweep_execution_status)
                                        .size(INFO_LABEL_SIZE)
                                        .build(),
                                );
                            });
                        });
                    });
                });
            });
        });

        state.clone().save_state(ui, "");

        ui.separator()
    }
}
