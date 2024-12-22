use crossterm::event::{self, KeyCode, KeyModifiers};
use crossterm::{
    event::KeyEvent,
    terminal::{disable_raw_mode, enable_raw_mode},
};
use crossterm::{queue, style::Print};
use evdev::{Device, InputEventKind, RelativeAxisType};
use std::{
    io::{stdout, Write},
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

pub struct MouseTracker {
    total_distance: i32,
}

impl MouseTracker {
    pub fn new() -> Self {
        MouseTracker { total_distance: 0 }
    }

    pub fn start_tracking(&mut self) {
        self.total_distance = 0;
    }

    pub fn stop_tracking(&mut self) -> i32 {
        self.total_distance.abs()
    }

    pub fn update_tracking(&mut self, distance: i32) {
        self.total_distance += distance;
    }
}

pub fn start() {
    enable_raw_mode().unwrap();

    let mouse_tracker = Arc::new(Mutex::new(MouseTracker::new()));
    let tracker_clone = Arc::clone(&mouse_tracker);

    let device_path = "/dev/input/event0";
    let mut device = Device::open(device_path).unwrap();

    thread::spawn(move || loop {
        let events = device.fetch_events().unwrap();

        events.into_iter().for_each(|event| {
            if let InputEventKind::RelAxis(RelativeAxisType::REL_X) = event.kind() {
                tracker_clone.lock().unwrap().update_tracking(event.value());
            }
        });
    });

    let mut tracking_active = false;
    let mut stdout = stdout();

    loop {
        if event::poll(Duration::from_millis(10)).unwrap() {
            if let event::Event::Key(KeyEvent {
                code, modifiers, ..
            }) = event::read().unwrap()
            {
                if modifiers.contains(KeyModifiers::ALT) && code == KeyCode::Char('r') {
                    if tracking_active {
                        let mut tracker = mouse_tracker.lock().unwrap();
                        let total_yaw_movement = tracker.stop_tracking();

                        queue!(stdout, Print(format!("\r\n{} pixels", total_yaw_movement)))
                            .unwrap();

                        stdout.flush().unwrap();
                    } else {
                        let mut tracker = mouse_tracker.lock().unwrap();

                        tracker.start_tracking();
                    }

                    tracking_active = !tracking_active;
                }

                if code == KeyCode::Esc {
                    disable_raw_mode().unwrap();
                    break;
                }
            }
        }
    }
}
