use super::shared_memory::{RefractEvent, SharedMemoryWriter};
use evdev::{Device, InputEventKind, RelativeAxisType};
use std::sync::{
    atomic::{AtomicBool, AtomicI32, Ordering},
    Arc,
};
use tokio::sync::mpsc;
use tracing::{error, instrument};

static RUNNING: AtomicBool = AtomicBool::new(false);

impl Default for MouseTracker {
    fn default() -> Self {
        MouseTracker {
            tracked_distance: Arc::new(AtomicI32::new(0)),
            tracking_active: Arc::new(AtomicBool::new(false)),
            stop_signal_sender: None,
        }
    }
}

pub struct MouseTracker {
    tracked_distance: Arc<AtomicI32>,
    tracking_active: Arc<AtomicBool>,
    stop_signal_sender: Option<mpsc::Sender<()>>,
}

impl MouseTracker {
    pub fn is_active(&self) -> bool {
        self.tracking_active.load(Ordering::Relaxed)
    }

    pub fn register_movement(&mut self, movement: i32) {
        if self.is_active() {
            self.tracked_distance.fetch_add(movement, Ordering::Relaxed);
        }
    }

    pub fn start(&mut self) {
        self.tracking_active.swap(true, Ordering::Relaxed);
    }

    pub fn stop(&mut self) -> i32 {
        let was_active = self.tracking_active.swap(false, Ordering::Relaxed);

        if !was_active {
            return 0;
        }

        if let Some(sender) = &self.stop_signal_sender {
            sender.try_send(()).unwrap();
        }

        let total_distance = self.tracked_distance.load(Ordering::Relaxed).abs();

        self.tracked_distance.store(0, Ordering::Relaxed);

        total_distance
    }

    #[instrument(skip(device), fields(device_name = ?device.name()))]
    pub fn start_watcher(mut device: Device) {
        if RUNNING.swap(true, Ordering::SeqCst) {
            panic!("MouseTracker::start_watcher called more than once!");
        }

        tokio::task::spawn_blocking(move || {
            let mut shared_memory_backend = SharedMemoryWriter::default();

            while let Ok(events) = device.fetch_events() {
                for event in events {
                    if let InputEventKind::RelAxis(RelativeAxisType::REL_X) = event.kind() {
                        let movement = event.value();

                        let result = shared_memory_backend
                            .write(&RefractEvent::RelativeMouseMovement(movement));

                        if let Err(err) = result {
                            error!(error = ?err, "Failed to write mouse movement");
                        }
                    }
                }
            }
        });
    }
}
