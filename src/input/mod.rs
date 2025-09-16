pub mod devices;
pub mod keyboard_tracker;
pub mod mouse_tracker;
pub mod shared_memory;

use crate::input::{
    devices::Devices, keyboard_tracker::KeyboardTracker, mouse_tracker::MouseTracker,
};

pub async fn start_trackers() {
    let devices = Devices::new();
    let main_keyboard_future = devices.get_main_keyboard();
    let main_mouse = devices
        .get_main_mouse()
        .await
        .expect("Failed to find the main mouse");

    MouseTracker::start_watcher(main_mouse);

    let main_keyboard = main_keyboard_future
        .await
        .expect("Failed to find the main keyboard");

    KeyboardTracker::start_watcher(main_keyboard);
}
