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

fn input_receiver_left_alt_t() -> mpsc::Receiver<()> {
    let (start_tracking_key_sender, start_tracking_key_receiver) = mpsc::channel::<()>();

    thread::spawn(move || {
        let keyboard_device_path = "/dev/input/event26";
        let mut keyboard_device = Device::open(keyboard_device_path).unwrap();

        let mut is_alt_down = false;

        while let Ok(events) = keyboard_device.fetch_events() {
            for event in events {
                if let InputEventKind::Key(Key::KEY_LEFTALT) = event.kind() {
                    is_alt_down = event.value() == 1;
                }

                if let InputEventKind::Key(Key::KEY_T) = event.kind() {
                    if event.value() == 1 && is_alt_down {
                        start_tracking_key_sender.send(()).unwrap();
                    }
                }
            }
        }
    });

    start_tracking_key_receiver
}

pub fn start() -> (mpsc::Receiver<bool>, mpsc::Receiver<i32>) {
    let mouse_tracker = Arc::new(Mutex::new(MouseTracker::new()));
    let start_tracking_key_receiver = input_receiver_left_alt_t();

    mouse_tracker_updater(&mouse_tracker);

    let (total_movement_sender, total_movement_receiver) = mpsc::channel::<i32>();
    let (tracking_status_sender, tracking_status_receiver) = mpsc::channel::<bool>();

    thread::spawn(move || {
        while start_tracking_key_receiver.recv().is_ok() {
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

    (tracking_status_receiver, total_movement_receiver)
}
