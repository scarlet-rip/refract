use super::shared_memory::{ComboEvent, RefractEvent, SharedMemoryWriter};
use evdev::{Device, InputEventKind, Key};
use std::{
    collections::HashSet,
    sync::atomic::{AtomicBool, Ordering},
};
use tracing::{error, instrument};

static RUNNING: AtomicBool = AtomicBool::new(false);

#[derive(Default)]
pub struct KeyboardTracker {}

impl KeyboardTracker {
    #[instrument(skip(device), fields(device_name = ?device.name()))]
    pub fn start_watcher(mut device: Device) {
        if RUNNING.swap(true, Ordering::SeqCst) {
            panic!("KeyboardTracker::start_watcher called more than once!");
        }

        tokio::task::spawn_blocking(move || {
            let mut keys_down: HashSet<Key> = HashSet::new();
            let mut shared_memory_backend = SharedMemoryWriter::default();

            while let Ok(events) = device.fetch_events() {
                for event in events {
                    if let InputEventKind::Key(key) = event.kind() {
                        match event.value() {
                            1 => {
                                keys_down.insert(key);
                            }
                            0 => {
                                keys_down.remove(&key);
                            }
                            _ => {}
                        }

                        // Alt + [
                        if keys_down.contains(&Key::KEY_LEFTALT)
                            && keys_down.contains(&Key::KEY_LEFTBRACE)
                        {
                            let event = RefractEvent::Combo(ComboEvent::Measure);
                            let result = shared_memory_backend.write(&event);

                            if let Err(err) = result {
                                error!(error = ?err, "Failed to write keyboard combo Alt + [");
                            }
                        }

                        // Alt + ]
                        if keys_down.contains(&Key::KEY_LEFTALT)
                            && keys_down.contains(&Key::KEY_RIGHTBRACE)
                        {
                            let event = RefractEvent::Combo(ComboEvent::Perform360);
                            let result = shared_memory_backend.write(&event);

                            if let Err(err) = result {
                                error!(error = ?err, "Failed to write keyboard combo Alt + ]");
                            }
                        }
                    }
                }
            }
        });
    }
}
