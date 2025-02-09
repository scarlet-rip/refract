use evdev::{Device, InputEventKind, RelativeAxisType};
use std::{
    sync::atomic::{AtomicBool, Ordering},
    sync::{mpsc, Arc, Mutex},
    thread,
    time::Duration,
};

pub struct MouseTracker {
    device: Arc<Mutex<Device>>,
    tracked_distance: Arc<Mutex<i32>>,
    tracking_active: Arc<AtomicBool>,
    stop_signal_sender: Option<mpsc::Sender<()>>,
}

impl MouseTracker {
    pub fn new(device: Device) -> Self {
        MouseTracker {
            device: Arc::new(Mutex::new(device)),
            tracked_distance: Arc::new(Mutex::new(0)),
            tracking_active: Arc::new(AtomicBool::new(false)),
            stop_signal_sender: None,
        }
    }

    pub fn is_active(&self) -> bool {
        self.tracking_active.load(Ordering::Relaxed)
    }

    pub fn start(&mut self) {
        if self.tracking_active.swap(true, Ordering::Relaxed) {
            return; // Prevent starting tracking if already active
        }

        let (stop_signal_sender, stop_signal_receiver) = mpsc::channel();
        self.stop_signal_sender = Some(stop_signal_sender);

        let tracked_distance = Arc::clone(&self.tracked_distance);
        let tracking_active = Arc::clone(&self.tracking_active);
        let device = Arc::clone(&self.device);

        thread::spawn(move || {
            while tracking_active.load(Ordering::Relaxed) {
                if let Ok(events) = device.lock().unwrap().fetch_events() {
                    for event in events {
                        if let InputEventKind::RelAxis(RelativeAxisType::REL_X) = event.kind() {
                            let movement = event.value();
                            let mut total = tracked_distance.lock().unwrap();
                            *total += movement.abs();
                        }
                    }
                }

                if stop_signal_receiver.try_recv().is_ok() {
                    break;
                }

                thread::sleep(Duration::from_millis(10));
            }
        });
    }

    pub fn stop(&mut self) -> i32 {
        if !self.tracking_active.swap(false, Ordering::Relaxed) {
            return 0; // Not actively tracking, return early
        }

        if let Some(sender) = &self.stop_signal_sender {
            let _ = sender.send(());
        }

        let mut total_distance = self.tracked_distance.lock().unwrap();
        let total = *total_distance;

        *total_distance = 0;

        total
    }
}
