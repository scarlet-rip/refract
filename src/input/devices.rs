use evdev::{Device, Key, RelativeAxisType};
use std::sync::Arc;
use tokio::sync::Mutex;
use udev::MonitorBuilder;

pub(crate) struct Devices {
    device_event_paths: Vec<String>,
}

impl Devices {
    pub fn new() -> Arc<Mutex<Self>> {
        let mut devices = Self {
            device_event_paths: Vec::new(),
        };

        devices.update_device_event_paths();

        let devices_mutex = Arc::new(Mutex::new(devices));
        let devices_mutex_clone = devices_mutex.clone();

        tokio::spawn(async move {
            Devices::start_auto_update(devices_mutex_clone).await;
        });

        devices_mutex
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

    async fn start_auto_update(devices: Arc<Mutex<Self>>) {
        let monitor = MonitorBuilder::new()
            .unwrap()
            .match_subsystem("input")
            .unwrap()
            .listen()
            .unwrap();

        for event in monitor.iter() {
            if let Some(action) = event.action() {
                if action == "add" || action == "remove" {
                    let devices_clone = devices.clone();

                    tokio::spawn(async move {
                        let mut locked = devices_clone.lock().await;

                        locked.update_device_event_paths();
                    });
                }
            }
        }
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
