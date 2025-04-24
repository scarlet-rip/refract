mod main_header;
pub(crate) use main_header::MainHeader;

mod main_panel;
pub(super) use main_panel::MainPanel;

mod yaw_sweep;
pub(crate) use yaw_sweep::YawSweep;

mod sensitivity_conversion;
pub(crate) use sensitivity_conversion::SensitivityConversion;

mod status_label;
pub(crate) use status_label::StatusLabel;

mod keybind_action_label;
pub(crate) use keybind_action_label::KeybindActionLabel;

lazy_static::lazy_static! {
    pub static ref ASSETS_DIRECTORY: String = std::env::var("REFRACT_ASSETS_DIRECTORY").unwrap_or(String::from("assets"));
}
