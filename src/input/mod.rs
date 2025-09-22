mod devices;
mod keyboard_tracker;
mod mouse_tracker;
pub mod shared_memory;
mod sweep;

pub use keyboard_tracker::KeyboardTracker;
pub use mouse_tracker::MouseTracker;
pub use sweep::Sweeper;

use crate::input::shared_memory::{ArchivedComboEvent, ArchivedRefractEvent, SharedMemoryReader};
use devices::Devices;
use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};
use std::sync::Arc;
use tokio::sync::Mutex;

pub static GLOBAL_YAW_SWEEP_PIXELS: AtomicI32 = AtomicI32::new(0);
pub static GLOBAL_YAW_SWEEP_STATUS: AtomicBool = AtomicBool::new(false);
pub static GLOBAL_TOTAL_MOVEMENT: AtomicI32 = AtomicI32::new(0);
pub static GLOBAL_TRACKING_STATUS: AtomicBool = AtomicBool::new(false);

pub async fn run_backend() {
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

pub fn run_frontend() {
    let mouse_tracker = Arc::new(Mutex::new(MouseTracker::default()));

    SharedMemoryReader::default().start_reader(move |event| match event {
        ArchivedRefractEvent::Combo(combo) => match combo {
            ArchivedComboEvent::Measure => {
                let mut mouse_tracker = mouse_tracker.try_lock().unwrap();

                if mouse_tracker.is_active() {
                    GLOBAL_TRACKING_STATUS.store(false, Ordering::Release);

                    let total_yaw_movement = mouse_tracker.stop();

                    GLOBAL_TOTAL_MOVEMENT.store(total_yaw_movement, Ordering::Release);
                } else {
                    GLOBAL_TRACKING_STATUS.store(true, Ordering::Release);

                    mouse_tracker.start();
                }
            }

            ArchivedComboEvent::Perform360 => {
                GLOBAL_YAW_SWEEP_STATUS.store(true, Ordering::Release);

                Sweeper::default()
                    .sweep(GLOBAL_YAW_SWEEP_PIXELS.load(Ordering::Acquire), 10, 5)
                    .unwrap();

                GLOBAL_YAW_SWEEP_STATUS.store(false, Ordering::Release);
            }
        },
        ArchivedRefractEvent::RelativeMouseMovement(movement) => {
            let mut mouse_tracker = mouse_tracker.try_lock().unwrap();

            if mouse_tracker.is_active() {
                mouse_tracker.register_movement(*movement);
            }
        }
    });
}
