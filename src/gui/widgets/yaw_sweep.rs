use super::{KeybindActionLabel, StatusLabel};
use crate::mouse::{GLOBAL_YAW_SWEEP_PIXELS, GLOBAL_YAW_SWEEP_STATUS};
use crate::start;
use egui::{
    load::TexturePoll, Align, Color32, Layout, Margin, Response, RichText, SizeHint, TextureFilter,
    TextureOptions, Ui, Widget,
};
use lazy_static::lazy_static;
use scarlet_egui::{
    frame::{Frame, FrameDecoration, FrameDecorationNineSlice},
    input_field::NumericInput,
    widget_state::{WidgetState, WidgetStateType},
};
use std::sync::Arc;
use tokio::sync::{mpsc::Receiver, Mutex};

const GROUP_HEADER_SIZE: f32 = 14.0;
const PARTITION_HEADER_SIZE: f32 = 14.0;
const PARTITION_INNER_LABEL_SIZE: f32 = 12.5;

const INFO_LABEL_SIZE: f32 = 9.0;

lazy_static! {
    static ref FRAME_TINT: Color32 = Color32::from_hex("#3a3737").expect("Invalid HEX");
    static ref GROUP_HEADER_COLOR: Color32 = Color32::from_hex("#6b0707").expect("Invalid HEX");
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
    sweep_distance_measurement_receiver: Arc<Mutex<Receiver<i32>>>,

    measurement_status: bool,
    measured_sweep_distance: i32,
}

impl WidgetState for YawSweepState {}

#[derive(Default)]
pub(crate) struct YawSweep {}

impl Widget for YawSweep {
    fn ui(self, ui: &mut Ui) -> Response {
        let texture = ui
            .ctx()
            .try_load_texture(
                "file://assets/nine_slice.png",
                TextureOptions {
                    magnification: TextureFilter::Nearest,
                    minification: TextureFilter::Nearest,

                    ..Default::default()
                },
                SizeHint::Size(48, 48),
            )
            .unwrap();

        match texture {
            TexturePoll::Ready { texture } => Frame::new(
                "yaw-sweep-frame",
                FrameDecoration::NineSlice(FrameDecorationNineSlice {
                    texture,
                    tint: Some(*FRAME_TINT),
                }),
                Margin::default(),
                Margin::default(),
            )
            .show(ui, |ui| {
                let mut state =
                    YawSweepState::load_or_new(ui, None, WidgetStateType::Runtime, || {
                        let (measurement_status_receiver, sweep_distance_measurement_receiver) =
                            start(Arc::new(Mutex::new(ui.ctx().to_owned())));

                        YawSweepState {
                            measurement_status_receiver: Arc::new(Mutex::new(
                                measurement_status_receiver,
                            )),
                            sweep_distance_measurement_receiver: Arc::new(Mutex::new(
                                sweep_distance_measurement_receiver,
                            )),

                            measurement_status: false,
                            measured_sweep_distance: i32::default(),
                        }
                    });

                let (mut measurement_status_receiver, mut sweep_distance_measurement_receiver) = (
                    state.measurement_status_receiver.try_lock().unwrap(),
                    state
                        .sweep_distance_measurement_receiver
                        .try_lock()
                        .unwrap(),
                );

                if let Ok(measurement_status) = measurement_status_receiver.try_recv() {
                    state.measurement_status = measurement_status;
                }

                if let Ok(sweep_distance_measurement) =
                    sweep_distance_measurement_receiver.try_recv()
                {
                    state.measured_sweep_distance = sweep_distance_measurement;
                }

                ui.with_layout(Layout::top_down(Align::Center), |ui| {
                    ui.label(
                        RichText::new("Yaw Sweep")
                            .size(GROUP_HEADER_SIZE)
                            .color(*GROUP_HEADER_COLOR),
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
                                            KeybindActionLabel::builder("ALT + [", "to measure")
                                                .build(),
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
                                            KeybindActionLabel::builder(
                                                "ALT + ]",
                                                "to perform a sweep",
                                            )
                                            .build(),
                                        );
                                    });

                                    ui.with_layout(Layout::top_down(Align::Center), |ui| {
                                        // TODO: Fix this dirty workaround
                                        // write blows up for sum reason
                                        if let Ok(mut pixels) = GLOBAL_YAW_SWEEP_PIXELS.try_write()
                                        {
                                            NumericInput::new(
                                                "sweep‑execution‑distance",
                                                &mut *pixels,
                                            )
                                            .show(ui);
                                        } else {
                                            let current =
                                                *GLOBAL_YAW_SWEEP_PIXELS.try_read().unwrap();
                                            let mut temp = current;

                                            NumericInput::new(
                                                "sweep‑execution‑distance (locked)",
                                                &mut temp,
                                            )
                                            .show(ui);
                                        }

                                        ui.add(
                                            StatusLabel::builder(
                                                *GLOBAL_YAW_SWEEP_STATUS.try_read().unwrap(),
                                            )
                                            .size(INFO_LABEL_SIZE)
                                            .build(),
                                        );
                                    });
                                });
                            });
                        });
                    });
                });

                state.clone().save_state(ui, None, WidgetStateType::Runtime);
            }),
            _ => ui.response(),
        }
    }
}
