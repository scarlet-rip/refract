pub mod combo;
pub mod devices;
pub mod relative_mouse_movement;
pub mod shared_memory;

use crate::input::{
    combo::combo_watcher, devices::Devices,
    relative_mouse_movement::relative_mouse_movement_watcher,
};

pub fn start() {
    let devices = Devices::new();
    let main_keyboard = devices
        .get_main_keyboard()
        .expect("Failed to find the main keyboard");
    let main_mouse = devices
        .get_main_mouse()
        .expect("Failed to find the main mouse");

    combo_watcher(main_keyboard);
    relative_mouse_movement_watcher(main_mouse);
}
