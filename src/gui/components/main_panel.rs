use super::{Pixel360Measurement, SensConversion};
use eframe::Frame;
use egui::{CentralPanel, Context};

#[derive(Default)]
pub(crate) struct MainPanel {
    pixel_360_measurement: Pixel360Measurement,
    sens_conversion: SensConversion,
}

impl eframe::App for MainPanel {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {
            self.pixel_360_measurement.show(ui);
            self.sens_conversion.show(ui);
        });
    }
}
