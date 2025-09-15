use super::{
    ensure_file_permissions_for_front_backend_communication, RefractEvent, SemSyncError,
    SharedMemoryError, SHARED_MEMORY_FILE_PATH, SHARED_MEMORY_FILE_PATH_OS_STR,
    SHARED_MEMORY_SYNC_SEMAPHORE,
};
use mmap_sync::synchronizer::Synchronizer;
use sem_safe::SemaphoreRef;
use std::{sync::Once, time::Duration};
use tracing::{debug, instrument};

pub struct SharedMemoryWriter<'a> {
    synchronizer: Synchronizer,
    semaphore: SemaphoreRef<'a>,
}

impl Default for SharedMemoryWriter<'_> {
    fn default() -> Self {
        Self {
            synchronizer: Synchronizer::new(&SHARED_MEMORY_FILE_PATH_OS_STR),
            semaphore: SHARED_MEMORY_SYNC_SEMAPHORE.sem_ref(),
        }
    }
}

impl SharedMemoryWriter<'_> {
    #[instrument(skip(self, event), fields(shm_path = %SHARED_MEMORY_FILE_PATH_OS_STR.to_string_lossy()))]
    pub fn write(&mut self, event: &RefractEvent) -> Result<(), SharedMemoryError> {
        self.synchronizer
            .write(event, Duration::from_secs(1))
            .map_err(SharedMemoryError::Write)?;

        static INIT: Once = Once::new();

        INIT.call_once(|| {
            ensure_file_permissions_for_front_backend_communication(
                "SharedMemory|> File Permissions Check",
                vec![
                    &format!("{SHARED_MEMORY_FILE_PATH}_data_0"),
                    &format!("{SHARED_MEMORY_FILE_PATH}_data_1"),
                    &format!("{SHARED_MEMORY_FILE_PATH}_state"),
                ],
            );

            debug!("Permissions ensured for shared-memory files");
        });

        self.semaphore
            .post()
            .map_err(|_| SharedMemoryError::SemSync(SemSyncError::Post))
    }
}
