use super::{
    KeybindActionLabel, StatusLabel, ASSETS_DIRECTORY, FRAME_TINT, GROUP_HEADER_COLOR,
    GROUP_HEADER_SIZE, INFO_LABEL_SIZE, PARTITION_HEADER_COLOR, PARTITION_HEADER_SIZE,
    PARTITION_INNER_LABEL_SIZE,
};
use crate::input::{
    GLOBAL_TOTAL_MOVEMENT, GLOBAL_TRACKING_STATUS, GLOBAL_YAW_SWEEP_PIXELS, GLOBAL_YAW_SWEEP_STATUS,
};
use egui::{
    load::TexturePoll, Align, Layout, Margin, Response, RichText, SizeHint, TextureFilter,
    TextureOptions, Ui, Widget,
};
use scarlet_egui::{
    frame::{Frame, FrameDecoration, FrameDecorationNineSlice},
    input_field::NumericInput,
};
use std::sync::atomic::Ordering;

#[derive(Default)]
pub(crate) struct YawSweep {}

impl Widget for YawSweep {
    fn ui(self, ui: &mut Ui) -> Response {
        let total_movement = GLOBAL_TOTAL_MOVEMENT.load(Ordering::Acquire);
        let tracking_status = GLOBAL_TRACKING_STATUS.load(Ordering::Acquire);
        let yaw_sweep_status = GLOBAL_YAW_SWEEP_STATUS.load(Ordering::Acquire);

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

                                ui.label(format!("Measured distance: {} px", total_movement,));

                                ui.columns(2, |cols| {
                                    cols[0].with_layout(Layout::right_to_left(Align::Min), |ui| {
                                        ui.add(
                                            KeybindActionLabel::builder("ALT + [", "to measure")
                                                .build(),
                                        );
                                    });

                                    cols[1].add(
                                        StatusLabel::builder(tracking_status)
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
                                        // TODO (for scarlet-egui): Allow atomics
                                        let mut yaw_sweep_pixels =
                                            GLOBAL_YAW_SWEEP_PIXELS.load(Ordering::Acquire);

                                        let resp = NumericInput::new(
                                            "sweep‑execution‑distance",
                                            &mut yaw_sweep_pixels,
                                        )
                                        .show(ui);

                                        // if user edited & its valid, write back to atomic
                                        if resp.response.changed() && resp.is_text_buffer_valid {
                                            GLOBAL_YAW_SWEEP_PIXELS
                                                .store(yaw_sweep_pixels, Ordering::Release);
                                        }

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
            }),
            _ => ui.response(),
        }
    }
}
