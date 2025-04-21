use bytecheck::CheckBytes;
use mmap_sync::synchronizer::Synchronizer;
use named_sem::NamedSemaphore;
use once_cell::sync::Lazy;
use rkyv::{Archive, Deserialize, Serialize};
use std::ffi::OsString;
use tokio::time::Duration;
use tokio_util::sync::CancellationToken;

static SHARED_MEMORY_FILE_PATH: Lazy<OsString> =
    Lazy::new(|| OsString::from("/dev/shm/refract-sm"));
const SEMAPHORE_NAME: &str = "/refract-sem";

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
    let listener_cancellation_token = CancellationToken::new();

    start_listener(
        |archived_refract_event| match archived_refract_event {
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
        },
        listener_cancellation_token.clone(),
    );

    write(&RefractEvent::Combo(ComboEvent::Measure));
    write(&RefractEvent::Combo(ComboEvent::Perform360));

    use tokio::time::{sleep, Duration};

    sleep(Duration::from_secs(10)).await;

    listener_cancellation_token.cancel();

    write(&RefractEvent::RelativeMouseMovement(-1));
    write(&RefractEvent::RelativeMouseMovement(1));
}

fn start_listener<F>(mut handler: F, cancellation_token: CancellationToken)
where
    F: FnMut(&ArchivedRefractEvent) + Send + 'static,
{
    tokio::task::spawn_blocking(move || {
        let mut synchronizer = Synchronizer::new(&SHARED_MEMORY_FILE_PATH);
        let mut semaphore =
            NamedSemaphore::create(SEMAPHORE_NAME, 0).expect("Failed to create semaphore");

        loop {
            if cancellation_token.is_cancelled() {
                break;
            }

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
