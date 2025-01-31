use super::widgets::MainPanel;
use eframe::Frame;
use eframe::NativeOptions;
use egui::{Context, Vec2, ViewportBuilder, WindowLevel};

const PINNED_WINDOW_SIZE: Vec2 = Vec2::new(450.0, 250.0);
const APP_ID: &str = "rip.scarlet.pixelgauge";
const WINDOW_TITLE: &str = "Pixel Gauge";

#[derive(Default)]
struct EframeApp {
    main_panel: MainPanel,
}

impl eframe::App for EframeApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        egui_extras::install_image_loaders(ctx);

        self.main_panel.show(ctx);
    }
}

pub fn run_eframe() -> eframe::Result {
    let options = NativeOptions {
        centered: true,
        viewport: ViewportBuilder::default()
            .with_window_level(WindowLevel::AlwaysOnTop)
            .with_resizable(false)
            .with_app_id(APP_ID)
            .with_title(WINDOW_TITLE)
            .with_always_on_top()
            .with_max_inner_size(PINNED_WINDOW_SIZE)
            .with_min_inner_size(PINNED_WINDOW_SIZE),
        ..Default::default()
    };

    eframe::run_native(
        APP_ID,
        options,
        Box::new(|_cc| Ok(Box::<EframeApp>::default())),
    )
}
