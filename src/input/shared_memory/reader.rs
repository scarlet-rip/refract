use super::{ArchivedRefractEvent, RefractEvent, SEMAPHORE, SHARED_MEMORY_FILE_PATH_OS_STR};
use super::{SemSyncError, SharedMemoryError};
use mmap_sync::synchronizer::Synchronizer;
use std::sync::atomic::{AtomicBool, Ordering};

static RUNNING: AtomicBool = AtomicBool::new(false);

pub struct SharedMemoryReader {}

impl SharedMemoryReader {
    pub fn start_reader<F>(mut handler: F)
    where
        F: FnMut(&ArchivedRefractEvent),
    {
        if RUNNING.swap(true, Ordering::SeqCst) {
            panic!("SharedMemoryReader::start_reader() called more than once!");
        }

        let mut synchronizer = Synchronizer::new(&SHARED_MEMORY_FILE_PATH_OS_STR);

        loop {
            if let Err(error) = Self::read_once(&mut synchronizer, &mut handler) {
                eprintln!("SharedMemoryReader: {error}");
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
        SEMAPHORE
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
