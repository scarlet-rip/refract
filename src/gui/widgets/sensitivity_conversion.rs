use egui::{Align, Color32, Layout, RichText, TextEdit, Ui};
use lazy_static::lazy_static;
use scarlet_frame::egui::Group;

const GROUP_HEADER_SIZE: f32 = 14.0;
const PARTITION_HEADER_SIZE: f32 = 14.0;
const PARTITION_INNER_LABEL_SIZE: f32 = 12.5;

lazy_static! {
    static ref GROUP_HEADER_SIZE_COLOR: Color32 =
        Color32::from_hex("#6b0707").expect("Invalid HEX");
    static ref PARTITION_HEADER_COLOR: Color32 = Color32::from_hex("#6b0707").expect("Invalid HEX");
    static ref HIGHLIGHT_COLOR: Color32 = Color32::from_hex("#6b0707").expect("Invalid HEX");
}

#[derive(Default)]
pub(crate) struct SensitivityConversion {
    original_in_game_sensitivity: String,
    original_pixel_360: String,
    target_pixel_360: String,
    converted_sensitivity: String,
}

impl SensitivityConversion {
    pub(crate) fn show(&mut self, ui: &mut Ui) {
        Group::new("sensitivity-conversion", "9slice-test.png").show(ui, |ui| {
            ui.with_layout(Layout::top_down(Align::Center), |ui| {
                ui.label(
                    RichText::new("Sensitivity Conversion")
                        .size(GROUP_HEADER_SIZE)
                        .color(*GROUP_HEADER_SIZE_COLOR),
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
                                            TextEdit::singleline(
                                                &mut self.original_in_game_sensitivity,
                                            )
                                            .desired_width(ui.available_width() / 1.4),
                                        );

                                        ui.add(
                                            TextEdit::singleline(&mut self.converted_sensitivity)
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
                                        ui.add(TextEdit::singleline(&mut self.target_pixel_360));
                                        ui.add(TextEdit::singleline(&mut self.original_pixel_360));
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
