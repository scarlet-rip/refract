use evdev::{Device, InputEventKind, Key, RelativeAxisType};
use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

pub struct MouseTracker {
    total_distance: i32,
    is_tracking: bool,
}

impl MouseTracker {
    pub fn new() -> Self {
        MouseTracker {
            total_distance: 0,
            is_tracking: false,
        }
    }

    pub fn start_tracking(&mut self) {
        self.total_distance = 0;
        self.is_tracking = true;
    }

    pub fn stop_tracking(&mut self) -> i32 {
        self.is_tracking = false;
        self.total_distance.abs()
    }

    pub fn update(&mut self, mouse_x_movement: i32) {
        if self.is_tracking {
            self.total_distance += mouse_x_movement;
        }
    }

    pub fn is_tracking(&self) -> bool {
        self.is_tracking
    }
}

fn mouse_tracker_updater(mouse_tracker_mutex: &Arc<Mutex<MouseTracker>>) {
    thread::spawn({
        let mouse_tracker = Arc::clone(mouse_tracker_mutex);

        move || {
            let mouse_device_path = "/dev/input/event0";
            let mut mouse_device = Device::open(mouse_device_path).unwrap();

            while let Ok(events) = mouse_device.fetch_events() {
                for event in events {
                    if let InputEventKind::RelAxis(RelativeAxisType::REL_X) = event.kind() {
                        let mut tracker = mouse_tracker.lock().unwrap();
                        let mouse_x_movement = event.value();

                        tracker.update(mouse_x_movement);
                    }
                }
            }
        }
    });
}

fn input_handler() -> (mpsc::Receiver<()>, mpsc::Receiver<()>) {
    let (start_tracking_key_sender, start_tracking_key_receiver) = mpsc::channel::<()>();
    let (do_360_sender, do_360_receiver) = mpsc::channel::<()>();

    thread::spawn(move || {
        let keyboard_device_path = "/dev/input/event4";
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

pub fn start() -> (
    mpsc::Receiver<bool>,
    mpsc::Receiver<i32>,
    mpsc::Sender<u32>,
    mpsc::Receiver<bool>,
) {
    let mouse_tracker = Arc::new(Mutex::new(MouseTracker::new()));
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

        thread::spawn(move || {
            let mut enigo = Enigo::new(&Settings::default()).unwrap();

            while do_360_receiver.recv().is_ok() {
                while do_360_receiver.recv().is_ok() {
                    do_360_pixel_status_sender.send(true).unwrap();

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
                }
            }
        });
    }

    mouse_tracker_updater(&mouse_tracker);

    let (total_movement_sender, total_movement_receiver) = mpsc::channel::<i32>();
    let (tracking_status_sender, tracking_status_receiver) = mpsc::channel::<bool>();

    thread::spawn(move || {
        while start_tracking_receiver.recv().is_ok() {
            let mut mouse_tracker = mouse_tracker.lock().unwrap();

            if mouse_tracker.is_tracking() {
                tracking_status_sender.send(false).unwrap();

                let total_yaw_movement = mouse_tracker.stop_tracking();

                total_movement_sender.send(total_yaw_movement).unwrap();
            } else {
                tracking_status_sender.send(true).unwrap();

                mouse_tracker.start_tracking();
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
