use super::NumericInput;
use egui::{Align, Color32, Layout, RichText, TextEdit, Ui};
use lazy_static::lazy_static;
use scarlet_egui::frame::Frame;

const GROUP_HEADER_SIZE: f32 = 14.0;
const PARTITION_HEADER_SIZE: f32 = 14.0;
const PARTITION_INNER_LABEL_SIZE: f32 = 12.5;

lazy_static! {
    static ref FRAME_TINT: Color32 = Color32::from_hex("#3a3737").expect("Invalid HEX");
    static ref GROUP_HEADER_COLOR: Color32 = Color32::from_hex("#6b0707").expect("Invalid HEX");
    static ref PARTITION_HEADER_COLOR: Color32 = Color32::from_hex("#6b0707").expect("Invalid HEX");
    static ref HIGHLIGHT_COLOR: Color32 = Color32::from_hex("#6b0707").expect("Invalid HEX");
}

#[derive(Default)]
pub(crate) struct SensitivityConversion {
    original_sens: f64,
    original_sweep: u16,
    target_sweep: u16,
    converted_sens: String,
}

fn convert_sensitivity(
    original_in_game_sensitivity: f64,
    original_pixels_per_360: u16,
    target_pixels_per_360: u16,
) -> f64 {
    let d360_difference = target_pixels_per_360 as f64 / original_pixels_per_360 as f64;

    original_in_game_sensitivity * d360_difference
}

impl SensitivityConversion {
    pub(crate) fn show(&mut self, ui: &mut Ui) {
        // TODO: Only convert the sensitivity if the inputs change
        self.converted_sens =
            convert_sensitivity(self.original_sens, self.original_sweep, self.target_sweep)
                .to_string();

        Frame::new("assets/nine_slice.png")
            .tint(*FRAME_TINT)
            .show(ui, |ui| {
                ui.with_layout(Layout::top_down(Align::Center), |ui| {
                    ui.label(
                        RichText::new("Sensitivity Conversion")
                            .size(GROUP_HEADER_SIZE)
                            .color(*GROUP_HEADER_COLOR),
                    );

                    ui.columns(2, |cols| {
                        cols[0].with_layout(Layout::left_to_right(Align::Min), |ui| {
                            ui.with_layout(Layout::top_down(Align::Center), |ui| {
                                ui.label(
                                    RichText::new("Sensitivity")
                                        .size(PARTITION_HEADER_SIZE)
                                        .color(*PARTITION_HEADER_COLOR),
                                );

                                ui.columns(2, |cols| {
                                    cols[0].with_layout(Layout::left_to_right(Align::Min), |ui| {
                                        ui.with_layout(Layout::top_down(Align::RIGHT), |ui| {
                                            ui.spacing_mut().item_spacing.y += 5.0;

                                            ui.label(
                                                RichText::new("Original:")
                                                    .size(PARTITION_INNER_LABEL_SIZE),
                                            );

                                            ui.label(
                                                RichText::new("Target:")
                                                    .size(PARTITION_INNER_LABEL_SIZE),
                                            );
                                        });
                                    });

                                    cols[1].with_layout(Layout::right_to_left(Align::Min), |ui| {
                                        ui.with_layout(Layout::top_down(Align::LEFT), |ui| {
                                            ui.add(
                                                NumericInput::new(
                                                    "original-sens",
                                                    &mut self.original_sens,
                                                )
                                                .desired_width(ui.available_width() / 1.4),
                                            );

                                            ui.add(
                                                TextEdit::singleline(&mut self.converted_sens)
                                                    .desired_width(ui.available_width() / 1.4)
                                                    .interactive(false)
                                                    .frame(false)
                                                    .text_color(*HIGHLIGHT_COLOR)
                                                    .horizontal_align(Align::Center),
                                            );
                                        });
                                    });
                                });
                            });
                        });

                        cols[1].with_layout(Layout::right_to_left(Align::Min), |ui| {
                            ui.with_layout(Layout::top_down(Align::Center), |ui| {
                                ui.label(
                                    RichText::new("Sweep Distance")
                                        .size(PARTITION_HEADER_SIZE)
                                        .color(*PARTITION_HEADER_COLOR),
                                );

                                ui.columns(2, |cols| {
                                    cols[0].with_layout(Layout::right_to_left(Align::Min), |ui| {
                                        ui.with_layout(Layout::top_down(Align::RIGHT), |ui| {
                                            ui.spacing_mut().item_spacing.y += 5.0;

                                            ui.label(
                                                RichText::new("Original (px):")
                                                    .size(PARTITION_INNER_LABEL_SIZE),
                                            );

                                            ui.label(
                                                RichText::new("Target (px):")
                                                    .size(PARTITION_INNER_LABEL_SIZE),
                                            );
                                        });
                                    });

                                    cols[1].with_layout(Layout::left_to_right(Align::Min), |ui| {
                                        ui.with_layout(Layout::top_down(Align::LEFT), |ui| {
                                            ui.add(NumericInput::new(
                                                "target-sweep",
                                                &mut self.target_sweep,
                                            ));

                                            ui.add(NumericInput::new(
                                                "original-sweep",
                                                &mut self.original_sweep,
                                            ));
                                        });
                                    });
                                });
                            });
                        });
                    });
                });
            });
    }
}
