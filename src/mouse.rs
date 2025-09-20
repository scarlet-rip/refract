use crate::input::{
    shared_memory::{ArchivedComboEvent, ArchivedRefractEvent, SharedMemoryReader},
    MouseTracker, Sweeper,
};
use std::sync::Arc;
use tokio::sync::Mutex;

use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};

pub static GLOBAL_YAW_SWEEP_PIXELS: AtomicI32 = AtomicI32::new(0);
pub static GLOBAL_YAW_SWEEP_STATUS: AtomicBool = AtomicBool::new(false);

pub static GLOBAL_TOTAL_MOVEMENT: AtomicI32 = AtomicI32::new(0);
pub static GLOBAL_TRACKING_STATUS: AtomicBool = AtomicBool::new(false);

pub fn start() {
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

                GLOBAL_YAW_SWEEP_STATUS.store(true, Ordering::Release);
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
