use egui::Context;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

use once_cell::sync::Lazy;
use std::sync::RwLock;

use crate::input::Devices;

pub(crate) static GLOBAL_YAW_SWEEP_PIXELS: Lazy<Arc<RwLock<i32>>> =
    Lazy::new(|| Arc::new(RwLock::new(0)));
pub(crate) static GLOBAL_YAW_SWEEP_STATUS: Lazy<Arc<RwLock<bool>>> =
    Lazy::new(|| Arc::new(RwLock::new(false)));

use super::{
    input::{start_keybind_receivers, MouseTracker},
    sweep::Sweeper,
};

pub fn start(ui_context: Arc<Mutex<Context>>) -> (mpsc::Receiver<bool>, mpsc::Receiver<i32>) {
    let devices = Devices::new();

    let main_mouse = devices.get_main_mouse().unwrap();
    let main_keyboard = devices.get_main_keyboard().unwrap();

    let mouse_tracker = Arc::new(Mutex::new(MouseTracker::new(main_mouse)));

    let (start_tracking_receiver, do_360_receiver) =
        start_keybind_receivers(main_keyboard);

    let ui_context_clone = Arc::clone(&ui_context);

    thread::spawn(move || {
        while do_360_receiver.recv().is_ok() {
            let mut status = GLOBAL_YAW_SWEEP_STATUS.write().unwrap();

            *status = true;
            ui_context_clone.lock().unwrap().request_repaint();

            drop(status);

            let ui_context_clone_inner = Arc::clone(&ui_context);

            thread::spawn(move || {
                Sweeper::default()
                    .perform_horizontal_sweep(*GLOBAL_YAW_SWEEP_PIXELS.read().unwrap(), 10, 5)
                    .unwrap();

                let mut status = GLOBAL_YAW_SWEEP_STATUS.write().unwrap();

                *status = false;
                ui_context_clone_inner.lock().unwrap().request_repaint();
            })
            .join()
            .unwrap();
        }
    });

    let (total_movement_sender, total_movement_receiver) = mpsc::channel::<i32>();
    let (tracking_status_sender, tracking_status_receiver) = mpsc::channel::<bool>();

    thread::spawn(move || {
        while start_tracking_receiver.recv().is_ok() {
            let mut mouse_tracker = mouse_tracker.lock().unwrap();

            if mouse_tracker.is_active() {
                tracking_status_sender.send(false).unwrap();

                let total_yaw_movement = mouse_tracker.stop();

                total_movement_sender.send(total_yaw_movement).unwrap();
            } else {
                tracking_status_sender.send(true).unwrap();

                mouse_tracker.start();
            }
        }
    });

    (tracking_status_receiver, total_movement_receiver)
}
