use super::shared_memory::{RefractEvent, SharedMemoryBackend};
use evdev::{Device, InputEventKind, RelativeAxisType};

pub fn relative_mouse_movement_watcher(mut device: Device) {
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
