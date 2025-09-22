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

use egui::Color32;
use once_cell::sync::Lazy;

static ASSETS_DIRECTORY: Lazy<String> =
    Lazy::new(|| std::env::var("REFRACT_ASSETS_DIRECTORY").unwrap_or(String::from("assets")));

const GROUP_HEADER_SIZE: f32 = 14.0;
const PARTITION_HEADER_SIZE: f32 = 14.0;
const PARTITION_INNER_LABEL_SIZE: f32 = 12.5;
const INFO_LABEL_SIZE: f32 = 9.0;

static FRAME_TINT: Lazy<Color32> =
    Lazy::new(|| Color32::from_hex("#3a3737").expect("Invalid HEX"));

static GROUP_HEADER_COLOR: Lazy<Color32> =
    Lazy::new(|| Color32::from_hex("#6b0707").expect("Invalid HEX"));

static PARTITION_HEADER_COLOR: Lazy<Color32> =
    Lazy::new(|| Color32::from_hex("#6b0707").expect("Invalid HEX"));

static HIGHLIGHT_COLOR: Lazy<Color32> =
    Lazy::new(|| Color32::from_hex("#6b0707").expect("Invalid HEX"));

static KEYBIND_HIGHLIGHT_COLOR: Lazy<Color32> =
    Lazy::new(|| Color32::from_hex("#821E1E").expect("Invalid HEX"));

static STATUS_HIGHLIGHT_COLOR_ACTIVE: Lazy<Color32> =
    Lazy::new(|| Color32::from_hex("#076A19").expect("Invalid HEX"));

static STATUS_HIGHLIGHT_COLOR_INACTIVE: Lazy<Color32> =
    Lazy::new(|| Color32::from_hex("#821E1E").expect("Invalid HEX"));
