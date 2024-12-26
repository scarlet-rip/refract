use super::{MainHeader, D360MeasurementDemo, SensitivityConversionDemo};
use egui::{CentralPanel, Context};

const PROJECT_ID: &str = "GK-C-001";
const PROJECT_CODENAME: &str = "Pixel Gauge";
const ASSIGNED_ENTITY: &str = "GK | Creators";

#[derive(Default)]
pub(crate) struct MainPanel {
    d360_measurement_demo: D360MeasurementDemo,
    sensitivity_conversion_demo: SensitivityConversionDemo,
}

impl MainPanel {
    pub fn show(&mut self, ctx: &Context) {
        CentralPanel::default().show(ctx, |ui| {
            ui.add(
                MainHeader::builder()
                    .project_id(PROJECT_ID)
                    .project_codename(PROJECT_CODENAME)
                    .assigned_entity(ASSIGNED_ENTITY)
                    .build(),
            );

            self.d360_measurement_demo.show(ui);
            self.sensitivity_conversion_demo.show(ui);
        });
    }
}
