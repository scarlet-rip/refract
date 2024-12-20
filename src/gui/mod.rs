pub(crate) mod components;
pub(crate) mod widgets;

use components::MainPanel;
use eframe::{egui::ViewportBuilder, NativeOptions};

pub fn run_gui() -> eframe::Result {
    let options = NativeOptions {
        viewport: ViewportBuilder::default()
            .with_resizable(false)
            .with_app_id("sens-matcher-linux")
            .with_title("sens-matcher-linux")
            .with_always_on_top()
            .with_max_inner_size([450.0, 350.0])
            .with_min_inner_size([450.0, 350.0]),
        ..Default::default()
    };

    eframe::run_native(
        "sens-matcher-linux",
        options,
        Box::new(|_cc| Ok(Box::<MainPanel>::default())),
    )
}
