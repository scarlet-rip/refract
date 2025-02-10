use egui::Context;
use enigo::{Coordinate, Enigo, Mouse, Settings};
use evdev::Device;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

use super::{
    input::{start_keybind_receivers, MouseTracker},
    sweep::Sweeper,
};

pub fn start(
    ui_context: &Context,
) -> (
    mpsc::Receiver<bool>,
    mpsc::Receiver<i32>,
    mpsc::Sender<u32>,
    mpsc::Receiver<bool>,
) {
    let mouse_tracker = Arc::new(Mutex::new(MouseTracker::new(
        Device::open("/dev/input/event0").unwrap(),
    )));

    let (start_tracking_receiver, do_360_receiver) =
        start_keybind_receivers(Device::open("/dev/input/event2").unwrap());

    let (do_360_pixel_amount_sender, do_360_pixel_amount_receiver) = mpsc::channel::<u32>();
    let (do_360_pixel_status_sender, do_360_pixel_status_receiver) = mpsc::channel::<bool>();

    let do_360_pixel_amount = Arc::new(Mutex::new(0));

    {
        let shared_do_360_pixel_amount = Arc::clone(&do_360_pixel_amount);

        thread::spawn(move || {
            while let Ok(pixel_amount) = do_360_pixel_amount_receiver.recv() {
                *shared_do_360_pixel_amount.lock().unwrap() = pixel_amount;
            }
        });
    }

    {
        let shared_do_360_pixel_amount = Arc::clone(&do_360_pixel_amount);
        let ui_context_clone = ui_context.clone();

        thread::spawn(move || {
            let mut sweeper = Sweeper::default();

            while do_360_receiver.recv().is_ok() {
                do_360_pixel_status_sender.send(true).unwrap();
                ui_context_clone.request_repaint();

                let pixels = *shared_do_360_pixel_amount.lock().unwrap() as i32;

                sweeper.perform_horizontal_sweep(pixels, 10, 5).unwrap();

                do_360_pixel_status_sender.send(false).unwrap();
                ui_context_clone.request_repaint();
            }
        });
    }

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

    (
        tracking_status_receiver,
        total_movement_receiver,
        do_360_pixel_amount_sender,
        do_360_pixel_status_receiver,
    )
}
