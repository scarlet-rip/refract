use bytecheck::CheckBytes;
use mmap_sync::synchronizer::Synchronizer;
use named_sem::NamedSemaphore;
use once_cell::sync::Lazy;
use rkyv::{Archive, Deserialize, Serialize};
use std::{
    ffi::OsString,
    sync::atomic::{AtomicBool, Ordering},
};
use tokio::time::Duration;

static SHARED_MEMORY_FILE_PATH: Lazy<OsString> =
    Lazy::new(|| OsString::from("/dev/shm/refract-sm"));
const SEMAPHORE_NAME: &str = "/refract-sem";
static LISTENER_STARTED: AtomicBool = AtomicBool::new(false);

#[derive(Archive, Serialize, Deserialize, Debug, PartialEq)]
#[archive_attr(derive(CheckBytes))]
pub enum ComboEvent {
    Measure,
    Perform360,
}

#[derive(Archive, Serialize, Deserialize, Debug, PartialEq)]
#[archive_attr(derive(CheckBytes))]
pub enum RefractEvent {
    Combo(ComboEvent),
    RelativeMouseMovement(i32),
}

pub async fn test() {
    start_listener(|archived_refract_event| match archived_refract_event {
        ArchivedRefractEvent::Combo(combo) => match combo {
            ArchivedComboEvent::Measure => {
                println!("measure")
            }
            ArchivedComboEvent::Perform360 => {
                println!("perform 360")
            }
        },
        ArchivedRefractEvent::RelativeMouseMovement(movement) => {
            println!("mouse movement: {movement}")
        }
    });

    write(&RefractEvent::Combo(ComboEvent::Measure));
    write(&RefractEvent::Combo(ComboEvent::Perform360));

    use tokio::time::{sleep, Duration};

    sleep(Duration::from_secs(3)).await;

    write(&RefractEvent::RelativeMouseMovement(-1));
    write(&RefractEvent::RelativeMouseMovement(1));
}

fn start_listener<F>(mut handler: F)
where
    F: FnMut(&ArchivedRefractEvent) + Send + 'static,
{
    if LISTENER_STARTED
        .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
        .is_err()
    {
        panic!("Listener is already running");
    }

    tokio::task::spawn_blocking(move || {
        let mut synchronizer = Synchronizer::new(&SHARED_MEMORY_FILE_PATH);
        let mut semaphore =
            NamedSemaphore::create(SEMAPHORE_NAME, 0).expect("Failed to create semaphore");

        loop {
            semaphore.wait().expect("Failed to wait for semaphore");

            let data = unsafe { synchronizer.read::<RefractEvent>(false) }.expect("read failed");
            let archived_data = &*data;

            handler(archived_data);
        }
    });
}

fn write(event: &RefractEvent) {
    let mut synchronizer = Synchronizer::new(&SHARED_MEMORY_FILE_PATH);

    synchronizer
        .write(event, Duration::from_secs(1))
        .expect("failed to write data");

    let mut semaphore =
        NamedSemaphore::create(SEMAPHORE_NAME, 0).expect("Failed to create semaphore");

    semaphore.post().expect("Failed to post semaphore");
}
