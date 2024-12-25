use super::MainHeader;
use egui::{CentralPanel, Context};

const PROJECT_ID: &str = "GK-C-001";
const PROJECT_CODENAME: &str = "Pixel Gauge";
const ASSIGNED_ENTITY: &str = "GK | Creators";

#[derive(Default)]
pub(crate) struct MainPanel {}

impl MainPanel {
    pub fn show(self, ctx: &Context) {
        CentralPanel::default().show(ctx, |ui| {
            ui.add(
                MainHeader::builder()
                    .project_id(PROJECT_ID)
                    .project_codename(PROJECT_CODENAME)
                    .assigned_entity(ASSIGNED_ENTITY)
                    .build(),
            );
        });
    }
}
