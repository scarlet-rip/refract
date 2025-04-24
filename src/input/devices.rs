use evdev::{Device, EventType, Key, RelativeAxisType};

pub(crate) struct Devices {
    device_event_paths: Vec<String>,
}

impl Devices {
    pub fn new() -> Self {
        let mut devices = Self {
            device_event_paths: Vec::new(),
        };

        devices.update_device_event_paths();

        devices
    }

    fn update_device_event_paths(&mut self) {
        let Ok(input_events) = std::fs::read_dir("/dev/input") else {
            return;
        };

        let input_event_paths = input_events
            .filter_map(Result::ok)
            .map(|entry| entry.path())
            .filter(|path| {
                path.file_name()
                    .and_then(|n| n.to_str())
                    .map_or(false, |name| name.starts_with("event"))
            })
            .filter_map(|path| path.to_str().map(|s| s.to_string()))
            .collect();

        self.device_event_paths = input_event_paths;
    }

    pub async fn get_main_keyboard(&self) -> Option<Device> {
        use tokio::{
            sync::mpsc,
            task,
            time::{timeout, Duration},
        };
        let (tx, mut rx) = mpsc::channel(1);
        let paths = self.device_event_paths.clone();

        for device_path in paths {
            let device_path_clone = device_path.clone();
            let tx_clone = tx.clone();

            task::spawn_blocking(move || {
                if let Ok(mut device) = Device::open(device_path_clone) {
                    if let Some(keys) = device.supported_keys() {
                        if keys.contains(Key::KEY_LEFTALT)
                            && keys.contains(Key::KEY_LEFTBRACE)
                            && keys.contains(Key::KEY_RIGHTBRACE)
                        {
                            // Workarounds mainly for logitech bluetooth devices
                            // as they show up as a mouse and a keyboard
                            let has_key_event = device
                                .fetch_events()
                                .map(|events| {
                                    events.into_iter().any(|e| e.event_type() == EventType::KEY)
                                })
                                .unwrap_or(false);

                            if has_key_event {
                                let _ = tx_clone.blocking_send(device);
                            }
                        }
                    }
                }
            });
        }

        (timeout(Duration::from_secs(120), rx.recv()).await).unwrap_or_default()
    }

    pub fn get_main_mouse(&self) -> Option<Device> {
        for device_path in &self.device_event_paths {
            if let Ok(device) = Device::open(device_path) {
                if let Some(axes) = device.supported_relative_axes() {
                    if axes.contains(RelativeAxisType::REL_X) {
                        return Some(device);
                    }
                }
            }
        }

        None
    }
}
