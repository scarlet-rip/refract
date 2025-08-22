use super::{RefractEvent, SEMAPHORE, SHARED_MEMORY_FILE_PATH, SHARED_MEMORY_FILE_PATH_OS_STR};
use file_owner::PathExt;
use mmap_sync::synchronizer::Synchronizer;
use sem_safe::SemaphoreRef;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::sync::Once;
use tokio::time::Duration;

pub struct SharedMemoryWriter<'a> {
    synchronizer: Synchronizer,
    semaphore: SemaphoreRef<'a>,
}

impl Default for SharedMemoryWriter<'_> {
    fn default() -> Self {
        Self {
            synchronizer: Synchronizer::new(&SHARED_MEMORY_FILE_PATH_OS_STR),
            semaphore: SEMAPHORE.sem_ref(),
        }
    }
}

impl SharedMemoryWriter<'_> {
    pub fn write(&mut self, event: &RefractEvent) {
        self.synchronizer
            .write(event, Duration::from_secs(1))
            .expect("failed to write data");

        static INIT: Once = Once::new();

        INIT.call_once(|| {
            let desired_mode = 0o660;
            let perms = fs::Permissions::from_mode(desired_mode);

            for suffix in ["_data_0", "_data_1", "_state"] {
                let path = format!("{SHARED_MEMORY_FILE_PATH}{suffix}");

                if let Ok(metadata) = fs::metadata(&path) {
                    let current_perms = metadata.permissions().mode() & 0o777;

                    if current_perms != desired_mode {
                        fs::set_permissions(&path, perms.clone())
                            .unwrap_or_else(|_| panic!("Failed to set permissions for {path}"));

                        path.set_group("refract").expect("failed to set group");
                    }
                }
            }
        });

        self.semaphore.post().expect("Failed to post semaphore");
    }
}
