mod numeric_input;
pub(crate) use numeric_input::{Num, NumericInput};

mod numeric_setting_box;
pub(crate) use numeric_setting_box::NumericSettingInput;

use eframe::egui::Ui;
use std::marker::{Send, Sync};

pub(crate) trait WidgetTempState: Clone + Default + Sync + Send + 'static {}

pub(crate) trait WidgetWithTempState {
    fn id_salt(&self) -> &str;

    fn try_load_state<'a, S: WidgetTempState + 'a>(&'a self, ui: &'a Ui) -> Option<S> {
        let id = generate_widget_id(ui, self.id_salt());

        ui.ctx()
            .memory_mut(|memory| memory.data.remove_temp::<S>(id))
    }

    fn load_state_or_default<'a, S: WidgetTempState + 'a>(&'a self, ui: &'a Ui) -> S {
        self.try_load_state::<S>(ui).unwrap_or_default()
    }

    fn update_state<'a, S: WidgetTempState + 'a>(&'a self, ui: &'a Ui, state: S) {
        let id = generate_widget_id(ui, self.id_salt());

        ui.ctx()
            .memory_mut(|memory| memory.data.insert_temp::<S>(id, state))
    }
}

fn generate_widget_id(ui: &Ui, id_salt: &str) -> eframe::egui::Id {
    ui.id().with(id_salt)
}
