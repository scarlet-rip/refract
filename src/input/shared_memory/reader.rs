use super::{
    sync::open_shm_sync_sem, ArchivedRefractEvent, RefractEvent, SHARED_MEMORY_FILE_PATH_OS_STR,
};
use super::{SemSyncError, SharedMemoryError};
use mmap_sync::synchronizer::Synchronizer;
use sem_safe::named::Semaphore;
use std::sync::atomic::{AtomicBool, Ordering};
use tracing::{error, info, instrument};

static RUNNING: AtomicBool = AtomicBool::new(false);

pub struct SharedMemoryReader {
    synchronizer: Synchronizer,
    semaphore: Semaphore,
}

impl Default for SharedMemoryReader {
    fn default() -> Self {
        Self {
            synchronizer: Synchronizer::new(&SHARED_MEMORY_FILE_PATH_OS_STR),
            semaphore: open_shm_sync_sem(),
        }
    }
}

impl SharedMemoryReader {
    #[instrument(skip_all, fields(shm_path = %SHARED_MEMORY_FILE_PATH_OS_STR.to_string_lossy()))]
    pub fn start_reader<F>(&mut self, mut handler: F)
    where
        F: FnMut(&ArchivedRefractEvent),
    {
        if RUNNING.swap(true, Ordering::SeqCst) {
            panic!("SharedMemoryReader::start_reader() called more than once!");
        }

        info!("shared-memory-reader started");

        loop {
            if let Err(err) = self.read_once(&mut handler) {
                error!(error = ?err, "read_once failed");

                std::thread::sleep(std::time::Duration::from_millis(50));
            }
        }
    }

    fn read_once<F>(&mut self, handler: &mut F) -> Result<(), SharedMemoryError>
    where
        F: FnMut(&ArchivedRefractEvent),
    {
        self.semaphore
            .sem_ref()
            .wait()
            .map_err(|_| SemSyncError::Wait)?;

        let data =
            unsafe { self.synchronizer.read::<RefractEvent>(false) }.map_err(SharedMemoryError::Read)?;

        let archived_data = &*data;

        handler(archived_data);

        Ok(())
    }
}
