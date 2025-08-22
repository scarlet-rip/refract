use super::{
    ArchivedRefractEvent, RefractEvent, LISTENER_STARTED, SEMAPHORE, SHARED_MEMORY_FILE_PATH_OS_STR,
};
use mmap_sync::synchronizer::Synchronizer;
use std::sync::atomic::Ordering;

pub struct SharedMemoryReader {}

impl SharedMemoryReader {
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
