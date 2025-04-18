use egui::Context;
use evdev::{Device, Key, RelativeAxisType};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

use once_cell::sync::Lazy;
use std::sync::RwLock;

pub(crate) static GLOBAL_YAW_SWEEP_PIXELS: Lazy<Arc<RwLock<i32>>> =
    Lazy::new(|| Arc::new(RwLock::new(0)));
pub(crate) static GLOBAL_YAW_SWEEP_STATUS: Lazy<Arc<RwLock<bool>>> =
    Lazy::new(|| Arc::new(RwLock::new(false)));

use super::{
    input::{start_keybind_receivers, MouseTracker},
    sweep::Sweeper,
};

fn find_main_mouse() -> Option<String> {
    let Ok(input_events) = std::fs::read_dir("/dev/input") else {
        return None;
    };

    input_events
        .filter_map(Result::ok)
        .filter_map(|entry| entry.path().to_str().map(|s| s.to_string()))
        .find_map(|path| {
            if path.contains("event") {
                if let Ok(device) = Device::open(path.as_str()) {
                    if let Some(axes) = device.supported_relative_axes() {
                        if axes.contains(RelativeAxisType::REL_X) {
                            return Some(path);
                        }
                    }
                }
            }

            None
        })
}

fn find_main_keyboard() -> Option<String> {
    let Ok(input_events) = std::fs::read_dir("/dev/input") else {
        return None;
    };

    input_events
        .filter_map(Result::ok)
        .filter_map(|entry| entry.path().to_str().map(|s| s.to_string()))
        .find_map(|path| {
            if path.contains("event") {
                if let Ok(device) = Device::open(path.as_str()) {
                    if let Some(keys) = device.supported_keys() {
                        if keys.contains(Key::KEY_LEFTALT) {
                            return Some(path);
                        }
                    }
                }
            }

            None
        })
}

pub fn start(ui_context: Arc<Mutex<Context>>) -> (mpsc::Receiver<bool>, mpsc::Receiver<i32>) {
    let main_mouse_path = find_main_mouse().unwrap();

    let mouse_tracker = Arc::new(Mutex::new(MouseTracker::new(
        Device::open(main_mouse_path).unwrap(),
    )));

    let main_keyboard_path = find_main_keyboard().unwrap();

    find_main_mouse().unwrap();

    let (start_tracking_receiver, do_360_receiver) =
        start_keybind_receivers(Device::open(main_keyboard_path).unwrap());

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
