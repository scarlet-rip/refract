use evdev::{Device, Key, RelativeAxisType};

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

    pub fn get_main_keyboard(&self) -> Option<Device> {
        for device_path in &self.device_event_paths {
            if let Ok(device) = Device::open(device_path) {
                if let Some(keys) = device.supported_keys() {
                    if keys.contains(Key::KEY_LEFTALT) {
                        return Some(device);
                    }
                }
            }
        }

        None
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
