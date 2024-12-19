use super::GameSettings;
use eframe::egui::{CentralPanel, Context};

#[derive(Default)]
pub(crate) struct MainPanel {
    game_settings: GameSettings,
}

impl eframe::App for MainPanel {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            self.game_settings.show(ui);
        });
    }
}
