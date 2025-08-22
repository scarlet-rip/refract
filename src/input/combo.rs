use super::shared_memory::{ComboEvent, RefractEvent, SharedMemoryWriter};
use evdev::{Device, InputEventKind, Key};
use std::collections::HashSet;
use std::sync::atomic::{AtomicBool, Ordering};

static WATCHER_STARTED: AtomicBool = AtomicBool::new(false);

pub fn combo_watcher(mut device: Device) {
    if WATCHER_STARTED
        .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
        .is_err()
    {
        panic!("combo_watcher is already started");
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

                        shared_memory_backend.write(&event);
                    }

                    // Alt + ]
                    if keys_down.contains(&Key::KEY_LEFTALT)
                        && keys_down.contains(&Key::KEY_RIGHTBRACE)
                    {
                        let event = RefractEvent::Combo(ComboEvent::Perform360);

                        shared_memory_backend.write(&event);
                    }
                }
            }
        }
    });
}
