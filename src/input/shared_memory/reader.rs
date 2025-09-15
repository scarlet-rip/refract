use super::{
    ArchivedRefractEvent, RefractEvent, SHARED_MEMORY_FILE_PATH_OS_STR,
    SHARED_MEMORY_SYNC_SEMAPHORE,
};
use super::{SemSyncError, SharedMemoryError};
use mmap_sync::synchronizer::Synchronizer;
use std::sync::atomic::{AtomicBool, Ordering};
use tracing::{error, info, instrument};

static RUNNING: AtomicBool = AtomicBool::new(false);

pub struct SharedMemoryReader {}

impl SharedMemoryReader {
    #[instrument(skip_all, fields(shm_path = %SHARED_MEMORY_FILE_PATH_OS_STR.to_string_lossy()))]
    pub fn start_reader<F>(mut handler: F)
    where
        F: FnMut(&ArchivedRefractEvent),
    {
        if RUNNING.swap(true, Ordering::SeqCst) {
            panic!("SharedMemoryReader::start_reader() called more than once!");
        }

        let mut synchronizer = Synchronizer::new(&SHARED_MEMORY_FILE_PATH_OS_STR);

        info!("shared-memory-reader started");

        loop {
            if let Err(err) = Self::read_once(&mut synchronizer, &mut handler) {
                error!(error = ?err, "read_once failed");

                std::thread::sleep(std::time::Duration::from_millis(50));
            }
        }
    }

    fn read_once<F>(
        synchronizer: &mut Synchronizer,
        handler: &mut F,
    ) -> Result<(), SharedMemoryError>
    where
        F: FnMut(&ArchivedRefractEvent),
    {
        SHARED_MEMORY_SYNC_SEMAPHORE
            .sem_ref()
            .wait()
            .map_err(|_| SharedMemoryError::SemSync(SemSyncError::Wait))?;

        let data =
            unsafe { synchronizer.read::<RefractEvent>(false) }.map_err(SharedMemoryError::Read)?;

        let archived_data = &*data;

        handler(archived_data);

        Ok(())
    }
}
