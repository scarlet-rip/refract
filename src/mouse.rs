use egui::Context;
use evdev::{Device, InputEventKind, Key};
use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

fn input_handler() -> (mpsc::Receiver<()>, mpsc::Receiver<()>) {
    let (start_tracking_key_sender, start_tracking_key_receiver) = mpsc::channel::<()>();
    let (do_360_sender, do_360_receiver) = mpsc::channel::<()>();

    thread::spawn(move || {
        let keyboard_device_path = "/dev/input/event2";
        let mut keyboard_device = Device::open(keyboard_device_path).unwrap();

        let mut is_alt_down = false;

        while let Ok(events) = keyboard_device.fetch_events() {
            for event in events {
                if let InputEventKind::Key(Key::KEY_LEFTALT) = event.kind() {
                    is_alt_down = event.value() == 1;
                }

                if let InputEventKind::Key(Key::KEY_M) = event.kind() {
                    if event.value() == 1 && is_alt_down {
                        start_tracking_key_sender.send(()).unwrap();
                    }
                }

                if let InputEventKind::Key(Key::KEY_X) = event.kind() {
                    if event.value() == 1 && is_alt_down {
                        do_360_sender.send(()).unwrap();
                    }
                }
            }
        }
    });

    (start_tracking_key_receiver, do_360_receiver)
}

use super::input::MouseTracker;

pub fn start(
    ui_context: &Context,
) -> (
    mpsc::Receiver<bool>,
    mpsc::Receiver<i32>,
    mpsc::Sender<u32>,
    mpsc::Receiver<bool>,
) {
    let mouse_tracker = Arc::new(Mutex::new(MouseTracker::new(Device::open("/dev/input/event0").unwrap())));
    let (start_tracking_receiver, do_360_receiver) = input_handler();

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
        use enigo::{Coordinate, Enigo, Mouse, Settings};
        use std::thread;
        use std::time::Duration;

        let shared_do_360_pixel_amount = Arc::clone(&do_360_pixel_amount);
        let ui_context_clone = ui_context.clone();

        thread::spawn(move || {
            let mut enigo = Enigo::new(&Settings::default()).unwrap();

            while do_360_receiver.recv().is_ok() {
                do_360_pixel_status_sender.send(true).unwrap();

                ui_context_clone.request_repaint();

                let pixels = *shared_do_360_pixel_amount.lock().unwrap() as i32;
                let chunk_size = 10;
                let delay = 5;

                for _ in 0..(pixels / chunk_size) {
                    enigo.move_mouse(chunk_size, 0, Coordinate::Rel).unwrap();
                    thread::sleep(Duration::from_millis(delay));
                }

                let remaining_pixels = pixels % chunk_size;
                if remaining_pixels > 0 {
                    enigo
                        .move_mouse(remaining_pixels, 0, Coordinate::Rel)
                        .unwrap();
                }

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
