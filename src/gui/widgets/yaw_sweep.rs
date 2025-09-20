use super::{
    KeybindActionLabel, StatusLabel, ASSETS_DIRECTORY, FRAME_TINT, GROUP_HEADER_COLOR,
    GROUP_HEADER_SIZE, INFO_LABEL_SIZE, PARTITION_HEADER_COLOR, PARTITION_HEADER_SIZE,
    PARTITION_INNER_LABEL_SIZE,
};
use egui::{
    load::TexturePoll, Align, Layout, Margin, Response, RichText, SizeHint, TextureFilter,
    TextureOptions, Ui, Widget,
};
use scarlet_egui::{
    frame::{Frame, FrameDecoration, FrameDecorationNineSlice},
    input_field::NumericInput,
    widget_state::{WidgetState, WidgetStateType},
};

#[derive(Clone)]
struct YawSweepState {
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
                &format!("file://{}/nine_slice.png", ASSETS_DIRECTORY.as_str()),
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
                let state = YawSweepState::load_or_new(ui, None, WidgetStateType::Runtime, || {
                    YawSweepState {
                        measurement_status: false,
                        measured_sweep_distance: i32::default(),
                    }
                });

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
                                        // TODO:
                                        // So currently it's a placeholder
                                    
                                        let mut pixels = i32::default();

                                        NumericInput::new("sweep‑execution‑distance", &mut pixels)
                                            .show(ui);

                                        let yaw_sweep_status = bool::default();

                                        ui.add(
                                            StatusLabel::builder(yaw_sweep_status)
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
