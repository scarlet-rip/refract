use crossterm::event::{self, KeyCode};
use crossterm::{
    event::KeyEvent,
    terminal::{disable_raw_mode, enable_raw_mode},
};
use hyprland::{
    data::CursorPosition,
    shared::{HyprData, HyprError},
};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub(crate) fn get_mouse_location() -> Result<(i64, i64), HyprError> {
    let position = CursorPosition::get()?;

    Ok((position.x, position.y))
}

pub struct MouseTracker {
    last_position: Option<(i64, i64)>,
    total_distance: i64,
}

impl MouseTracker {
    pub fn new() -> Self {
        MouseTracker {
            last_position: None,
            total_distance: 0,
        }
    }

    pub fn start_tracking(&mut self, position: (i64, i64)) {
        self.last_position = Some(position);
        self.total_distance = 0;
    }

    pub fn stop_tracking(&mut self, position: (i64, i64)) {
        if let Some(last_pos) = self.last_position {
            let dx = position.0 - last_pos.0;

            self.total_distance += dx.abs();
        }
    }

    pub fn update_tracking(&mut self, position: (i64, i64)) {
        if let Some(last_pos) = self.last_position {
            let dx = position.0 - last_pos.0;

            self.total_distance += dx.abs();
        }

        self.last_position = Some(position);
    }

    pub fn get_total_distance(&self) -> i64 {
        self.total_distance
    }

    pub fn reset(&mut self) {
        self.last_position = Some((0, 0));
        self.total_distance = 0;
    }
}

fn start_tracking(tracker: &mut MouseTracker) {
    tracker.reset();

    let start_position = get_mouse_location().unwrap();

    tracker.start_tracking(start_position);
}

fn stop_tracking(tracker: &mut MouseTracker) -> i64 {
    let end_position = get_mouse_location().unwrap();

    tracker.stop_tracking(end_position);

    tracker.get_total_distance()
}


fn while_tracking(tracker: Arc<Mutex<MouseTracker>>) {
    loop {
        let position = get_mouse_location().unwrap();
        let mut tracker = tracker.lock().unwrap();

        tracker.update_tracking(position);

        thread::sleep(Duration::from_millis(10));
    }
}

pub fn start() {
    enable_raw_mode().unwrap();

    let mouse_tracker = Arc::new(Mutex::new(MouseTracker::new()));

    let tracker_clone = Arc::clone(&mouse_tracker);

    thread::spawn(move || {
        while_tracking(tracker_clone);
    });

    loop {
        if event::poll(Duration::from_millis(100)).unwrap() {
            if let event::Event::Key(KeyEvent { code, .. }) = event::read().unwrap() {
                match code {
                    KeyCode::Char('r') => {
                        println!("Started");

                        let mut tracker = mouse_tracker.lock().unwrap();

                        start_tracking(&mut tracker);
                    }
                    KeyCode::Char('s') => {
                        let mut tracker = mouse_tracker.lock().unwrap();
                        let total_yaw_movement = stop_tracking(&mut tracker);

                        println!("{} pixels", total_yaw_movement);
                    }
                    KeyCode::Esc => {
                        disable_raw_mode().unwrap();

                        break;
                    }
                    _ => {}
                }
            }
        }

        thread::sleep(Duration::from_millis(10));
    }
}
