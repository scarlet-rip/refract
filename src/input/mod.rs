pub mod combo;
pub mod devices;
pub mod relative_mouse_movement;
pub mod shared_memory;

use crate::input::{
    combo::combo_watcher, devices::Devices,
    relative_mouse_movement::relative_mouse_movement_watcher,
};

pub async fn start() {
    let devices = Devices::new();
    let main_keyboard_future = devices.get_main_keyboard();
    let main_mouse = devices
        .get_main_mouse().await
        .expect("Failed to find the main mouse");

    println!("mouse {:#?}", main_mouse.name().unwrap());

    relative_mouse_movement_watcher(main_mouse);

    let main_keyboard = main_keyboard_future
        .await
        .expect("Failed to find the main keyboard");

    println!("keybord {:#?}", main_keyboard.name().unwrap());

    combo_watcher(main_keyboard);
}
