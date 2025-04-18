use evdev::{Device, InputEventKind, Key};
use std::sync::mpsc;
use std::thread;

pub(crate) fn start_keybind_receivers(
    mut device: Device,
) -> (mpsc::Receiver<()>, mpsc::Receiver<()>) {
    let (start_tracking_key_sender, start_tracking_key_receiver) = mpsc::channel::<()>();
    let (do_360_sender, do_360_receiver) = mpsc::channel::<()>();

    thread::spawn(move || {
        let mut is_alt_down = false;

        while let Ok(events) = device.fetch_events() {
            for event in events {
                if let InputEventKind::Key(Key::KEY_LEFTALT) = event.kind() {
                    is_alt_down = event.value() == 1;
                }

                if let InputEventKind::Key(Key::KEY_LEFTBRACE) = event.kind() {
                    if event.value() == 1 && is_alt_down {
                        start_tracking_key_sender.send(()).unwrap();
                    }
                }

                if let InputEventKind::Key(Key::KEY_RIGHTBRACE) = event.kind() {
                    if event.value() == 1 && is_alt_down {
                        do_360_sender.send(()).unwrap();
                    }
                }
            }
        }
    });

    (start_tracking_key_receiver, do_360_receiver)
}
