use crate::input::{
    Sweeper,
    MouseTracker,
    shared_memory::{ArchivedComboEvent, ArchivedRefractEvent, SharedMemoryReader},
};
use egui::Context;
use once_cell::sync::Lazy;
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex, RwLock};

pub static GLOBAL_YAW_SWEEP_PIXELS: Lazy<Arc<RwLock<i32>>> = Lazy::new(|| Arc::new(RwLock::new(0)));
pub static GLOBAL_YAW_SWEEP_STATUS: Lazy<Arc<RwLock<bool>>> =
    Lazy::new(|| Arc::new(RwLock::new(false)));

pub fn start(ui_context: Arc<Mutex<Context>>) -> (mpsc::Receiver<bool>, mpsc::Receiver<i32>) {
    let mouse_tracker = Arc::new(Mutex::new(MouseTracker::default()));
    let ui_context_clone = Arc::clone(&ui_context);

    let (tracking_status_sender, tracking_status_receiver) = mpsc::channel::<bool>(1);
    let (total_movement_sender, total_movement_receiver) = mpsc::channel::<i32>(1);

    SharedMemoryReader::default().start_reader(move |event| match event {
        ArchivedRefractEvent::Combo(combo) => match combo {
            ArchivedComboEvent::Measure => {
                let mut mouse_tracker = mouse_tracker.try_lock().unwrap();

                if mouse_tracker.is_active() {
                    tracking_status_sender.try_send(false).unwrap();

                    let total_yaw_movement = mouse_tracker.stop();

                    total_movement_sender.try_send(total_yaw_movement).unwrap();
                } else {
                    tracking_status_sender.try_send(true).unwrap();

                    mouse_tracker.start();
                }
            }

            ArchivedComboEvent::Perform360 => {
                let mut status = GLOBAL_YAW_SWEEP_STATUS.try_write().unwrap();

                *status = true;
                ui_context_clone.try_lock().unwrap().request_repaint();

                drop(status);

                let ui_context_clone_inner = Arc::clone(&ui_context);

                Sweeper::default()
                    .sweep(*GLOBAL_YAW_SWEEP_PIXELS.try_read().unwrap(), 10, 5)
                    .unwrap();

                let mut status = GLOBAL_YAW_SWEEP_STATUS.try_write().unwrap();

                *status = false;
                ui_context_clone_inner.try_lock().unwrap().request_repaint();
            }
        },
        ArchivedRefractEvent::RelativeMouseMovement(movement) => {
            let mut mouse_tracker = mouse_tracker.try_lock().unwrap();

            if mouse_tracker.is_active() {
                mouse_tracker.register_movement(*movement);
            }
        }
    });

    (tracking_status_receiver, total_movement_receiver)
}
