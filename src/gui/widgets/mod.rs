mod main_header;
pub(crate) use main_header::MainHeader;

mod main_panel;
pub(super) use main_panel::MainPanel;

mod d360_measurement_demo;
pub(crate) use d360_measurement_demo::D360MeasurementDemo;

mod sensitivity_conversion_demo;
pub(crate) use sensitivity_conversion_demo::SensitivityConversionDemo;

use egui::{Id, Ui};
use std::marker::{Send, Sync};

#[allow(dead_code)]
pub(crate) trait WidgetState: Clone + Sync + Send + 'static {
    fn try_load(ui: &Ui, id_salt: &str) -> Option<Self> {
        let id = generate_temp_id(ui, id_salt);

        ui.ctx().memory(|memory| memory.data.get_temp::<Self>(id))
    }

    fn load_or_default(ui: &Ui, id_salt: &str) -> Self
    where
        Self: Default,
    {
        Self::try_load(ui, id_salt).unwrap_or_default()
    }

    fn save_state(self, ui: &Ui, id_salt: &str) {
        let id = generate_temp_id(ui, id_salt);

        ui.ctx()
            .memory_mut(|memory| memory.data.insert_temp::<Self>(id, self))
    }
}

fn generate_temp_id(ui: &Ui, id_salt: &str) -> Id {
    ui.id().with(id_salt)
}
