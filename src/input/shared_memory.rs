use bytecheck::CheckBytes;
use mmap_sync::synchronizer::Synchronizer;
use rkyv::{Archive, Deserialize, Serialize};
use sem_safe::{
    named::{OpenFlags, Semaphore},
    SemaphoreRef,
};
use std::{
    ffi::{CString, OsString},
    fs::{set_permissions, Permissions},
    os::unix::fs::PermissionsExt,
    sync::atomic::{AtomicBool, Ordering},
};
use tokio::time::Duration;

// TODO: properly handle the file permission stuff

const SHARED_MEMORY_FILE_PATH: &str = "/dev/shm/refract-sm";

lazy_static::lazy_static! {
    pub static ref SHARED_MEMORY_FILE_PATH_OS_STR: OsString = OsString::from(SHARED_MEMORY_FILE_PATH.to_string());

    static ref SEMAPHORE: Semaphore = {
        let semaphore_path = "/dev/shm/sem.refract-sem";

        if std::fs::File::open(semaphore_path).is_ok() {
            return Semaphore::open(
                &CString::new("/refract-sem").expect("Failed to name semaphore"),
                OpenFlags::Create {
                    exclusive: false,
                    value: 0,
                    mode: 0o660,
                },
            ).expect("Failed to open semaphore")
        } else {
            let semaphore = Semaphore::open(
                &CString::new("/refract-sem").expect("Failed to name semaphore"),
                OpenFlags::Create {
                    exclusive: false,
                    value: 0,
                    mode: 0o660,
                },
            ).expect("Failed to open semaphore");

            set_permissions(semaphore_path, Permissions::from_mode(0o660)).expect("Failed to set semaphore file permissions");

            return semaphore
        }
    };
}

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
            let mut synchronizer = Synchronizer::new(&SHARED_MEMORY_FILE_PATH_OS_STR);

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
            synchronizer: Synchronizer::new(&SHARED_MEMORY_FILE_PATH_OS_STR),
            semaphore: SEMAPHORE.sem_ref(),
        }
    }
}

impl SharedMemoryBackend<'_> {
    pub fn write(&mut self, event: &RefractEvent) {
        self.synchronizer
            .write(event, Duration::from_secs(1))
            .expect("failed to write data");

        use std::fs;

        let desired_mode = 0o660;
        let perms = fs::Permissions::from_mode(desired_mode);

        for suffix in ["_data_0", "_data_1", "_state"] {
            let path = format!("{SHARED_MEMORY_FILE_PATH}{suffix}");

            if let Ok(metadata) = fs::metadata(&path) {
                let current_perms = metadata.permissions().mode() & 0o777; // Only permission bits
                if current_perms != desired_mode {
                    fs::set_permissions(&path, perms.clone())
                        .unwrap_or_else(|_| panic!("Failed to set permissions for {path}"));
                }
            }
        }

        self.semaphore.post().expect("Failed to post semaphore");
    }
}
