use super::{MainHeader, SensitivityConversion, YawSweep};
use egui::{CentralPanel, Context};

const PROJECT_ID: &str = "SC-001";
const PROJECT_CODENAME: &str = "Refract";
const ASSIGNED_ENTITY: &str = "Scarlet R.I.P.";

#[derive(Default)]
pub(crate) struct MainPanel {
    sensitivity_conversion: SensitivityConversion,
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

            ui.add(YawSweep::default());

            self.sensitivity_conversion.show(ui);
        });
    }
}
