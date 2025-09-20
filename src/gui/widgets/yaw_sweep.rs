use super::{
    KeybindActionLabel, StatusLabel, ASSETS_DIRECTORY, FRAME_TINT, GROUP_HEADER_COLOR,
    GROUP_HEADER_SIZE, INFO_LABEL_SIZE, PARTITION_HEADER_COLOR, PARTITION_HEADER_SIZE,
    PARTITION_INNER_LABEL_SIZE,
};
use crate::mouse::{
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
use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};

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
                                    GLOBAL_TOTAL_MOVEMENT.load(Ordering::Acquire)
                                ));

                                ui.columns(2, |cols| {
                                    cols[0].with_layout(Layout::right_to_left(Align::Min), |ui| {
                                        ui.add(
                                            KeybindActionLabel::builder("ALT + [", "to measure")
                                                .build(),
                                        );
                                    });

                                    cols[1].add(
                                        StatusLabel::builder(
                                            GLOBAL_TRACKING_STATUS.load(Ordering::Acquire),
                                        )
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
                                        /* atomic_i32_input(
                                            ui,
                                            "sweep‑execution‑distance",
                                            &GLOBAL_YAW_SWEEP_PIXELS,
                                        ); */

                                        let mut tmp =
                                            GLOBAL_YAW_SWEEP_PIXELS.load(Ordering::Acquire);

                                        // 2) pass a &mut i32 to your existing widget
                                        let resp =
                                            NumericInput::new("sweep‑execution‑distance", &mut tmp)
                                                .show(ui);

                                        // 3) if user edited & it parsed, write back to atomic
                                        if resp.response.changed() && resp.is_text_buffer_valid {
                                            GLOBAL_YAW_SWEEP_PIXELS.store(tmp, Ordering::Release);
                                        }

                                        let yaw_sweep_status =
                                            GLOBAL_YAW_SWEEP_STATUS.load(Ordering::Acquire);

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

/* fn atomic_i32_input<'a>(ui: &mut Ui, id: &'a str, cell: &'a AtomicI32) -> NumericInputResponse {
    // 1) snapshot value from atomic
    let mut tmp = cell.load(Ordering::Acquire);

    // 2) pass a &mut i32 to your existing widget
    let resp = NumericInput::new(id, &mut tmp).show(ui);

    // 3) if user edited & it parsed, write back to atomic
    if resp.response.changed() && resp.is_text_buffer_valid {
        cell.store(tmp, Ordering::Release);
    }

    resp
} */
