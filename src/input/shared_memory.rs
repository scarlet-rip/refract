use bytecheck::CheckBytes;
use mmap_sync::synchronizer::Synchronizer;
use once_cell::sync::Lazy;
use rkyv::{Archive, Deserialize, Serialize};
use sem_safe::{
    named::{OpenFlags, Semaphore},
    SemaphoreRef,
};
use std::{
    ffi::{CString, OsString},
    sync::atomic::{AtomicBool, Ordering},
};
use tokio::time::Duration;

static SHARED_MEMORY_FILE_PATH: Lazy<OsString> =
    Lazy::new(|| OsString::from("/dev/shm/refract-sm"));
static LISTENER_STARTED: AtomicBool = AtomicBool::new(false);
static SEMAPHORE: Lazy<Semaphore> = Lazy::new(|| {
    Semaphore::open(
        &CString::new("/refract-sem").expect("Failed to name semaphore"),
        OpenFlags::Create {
            exclusive: false,
            value: 0,
            mode: 0o660,
        },
    )
    .expect("Failed to open semaphore")
});

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

pub struct SharedMemoryFrontend {}

impl SharedMemoryFrontend {
    pub fn start_listener<F>(mut handler: F)
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

            loop {
                SEMAPHORE
                    .sem_ref()
                    .wait()
                    .expect("Failed to wait for semaphore");

                let data =
                    unsafe { synchronizer.read::<RefractEvent>(false) }.expect("read failed");
                let archived_data = &*data;

                handler(archived_data);
            }
        });
    }
}

pub struct SharedMemoryBackend<'a> {
    synchronizer: Synchronizer,
    semaphore: SemaphoreRef<'a>,
}

impl Default for SharedMemoryBackend<'_> {
    fn default() -> Self {
        Self {
            synchronizer: Synchronizer::new(&SHARED_MEMORY_FILE_PATH),
            semaphore: SEMAPHORE.sem_ref(),
        }
    }
}

impl SharedMemoryBackend<'_> {
    pub fn write(&mut self, event: &RefractEvent) {
        self.synchronizer
            .write(event, Duration::from_secs(1))
            .expect("failed to write data");

        self.semaphore.post().expect("Failed to post semaphore");
    }
}
