use evdev::{Device, InputEventKind, Key};
use std::collections::HashSet;
use std::sync::mpsc;
use std::thread;

pub(crate) fn start_keybind_receivers(
    mut device: Device,
) -> (mpsc::Receiver<()>, mpsc::Receiver<()>) {
    let (start_tracking_key_sender, start_tracking_key_receiver) = mpsc::channel::<()>();
    let (do_360_sender, do_360_receiver) = mpsc::channel::<()>();

    thread::spawn(move || {
        let mut keys_down: HashSet<Key> = HashSet::new();

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
                        let _ = start_tracking_key_sender.send(());
                    }

                    // Alt + ]
                    if keys_down.contains(&Key::KEY_LEFTALT)
                        && keys_down.contains(&Key::KEY_RIGHTBRACE)
                    {
                        let _ = do_360_sender.send(());
                    }
                }
            }
        }
    });

    (start_tracking_key_receiver, do_360_receiver)
}
