use super::YawIncrementCalculator;
use eframe::Frame;
use egui::{CentralPanel, Context};

#[derive(Default)]
pub(crate) struct MainPanel {
    yaw_increment_calculator: YawIncrementCalculator,
}

impl eframe::App for MainPanel {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {
            self.yaw_increment_calculator.show(ui);
        });
    }
}
