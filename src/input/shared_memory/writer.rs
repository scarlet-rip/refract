use super::{
    setup_file_permissions_for_front_backend, RefractEvent, SemSyncError, SharedMemoryError,
    SEMAPHORE, SHARED_MEMORY_FILE_PATH, SHARED_MEMORY_FILE_PATH_OS_STR,
};
use mmap_sync::synchronizer::Synchronizer;
use sem_safe::SemaphoreRef;
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
    pub fn write(&mut self, event: &RefractEvent) -> Result<(), SharedMemoryError> {
        self.synchronizer
            .write(event, Duration::from_secs(1))
            .map_err(SharedMemoryError::Write)?;

        static INIT: Once = Once::new();

        INIT.call_once(|| {
            setup_file_permissions_for_front_backend(
                "SharedMemory|> File Permissions Check",
                vec![
                    &(SHARED_MEMORY_FILE_PATH.to_owned() + "_data_0"),
                    &(SHARED_MEMORY_FILE_PATH.to_owned() + "_data_1"),
                    &(SHARED_MEMORY_FILE_PATH.to_owned() + "_state"),
                ],
            )
        });

        self.semaphore
            .post()
            .map_err(|_| SharedMemoryError::SemSync(SemSyncError::Post))
    }
}
