pub mod combo;
pub mod devices;
pub mod relative_mouse_movement;
pub mod shared_memory;

use crate::input_backend::{
    combo::combo_watcher, devices::Devices,
    relative_mouse_movement::relative_mouse_movement_watcher,
};
use shared_memory::{ArchivedComboEvent, ArchivedRefractEvent, SharedMemoryFrontend};

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

    SharedMemoryFrontend::start_listener(|archived_refract_event| match archived_refract_event {
        ArchivedRefractEvent::Combo(combo) => match combo {
            ArchivedComboEvent::Measure => {
                println!("measure")
            }
            ArchivedComboEvent::Perform360 => {
                println!("perform 360")
            }
        },
        ArchivedRefractEvent::RelativeMouseMovement(movement) => {
            println!("mouse movement: {movement}")
        }
    });
}
