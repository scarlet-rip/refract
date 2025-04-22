use super::shared_memory::{RefractEvent, SharedMemoryBackend};
use evdev::{Device, InputEventKind, RelativeAxisType};
use std::sync::atomic::{AtomicBool, Ordering};

static WATCHER_STARTED: AtomicBool = AtomicBool::new(false);

pub fn relative_mouse_movement_watcher(mut device: Device) {
    if WATCHER_STARTED
        .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
        .is_err()
    {
        panic!("relative_mouse_movement_watcher is already started");
    }

    tokio::task::spawn_blocking(move || {
        let mut shared_memory_backend = SharedMemoryBackend::default();

        while let Ok(events) = device.fetch_events() {
            for event in events {
                if let InputEventKind::RelAxis(RelativeAxisType::REL_X) = event.kind() {
                    let movement = event.value();

                    shared_memory_backend.write(&RefractEvent::RelativeMouseMovement(movement));
                }
            }
        }
    });
}
