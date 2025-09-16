use super::ensure_file_permissions_for_front_backend_communication;
use errno::errno;
use once_cell::sync::Lazy;
use sem_safe::named::{OpenFlags, Semaphore};
use std::ffi::CString;

const SEMAPHORE_PATH_STR: &str = "/dev/shm/sem.refract-sem";
static SEMAPHORE_NAME_C_STRING: Lazy<CString> =
    Lazy::new(|| CString::new("/refract-sem").expect("Failed to name semaphore"));

pub fn open_shm_sync_sem() -> Semaphore {
    if std::fs::File::open(SEMAPHORE_PATH_STR).is_err() {
        panic!(
            "Semaphore doesn't exist at SEMAPHORE_PATH_STR, make sure you run the backend first"
        );
    }

    ensure_file_permissions_for_front_backend_communication(
        "SemSync|>File Permissions Check",
        vec![SEMAPHORE_PATH_STR],
    );

    create_or_open_shm_sync_sem()
}

pub fn create_or_open_shm_sync_sem() -> Semaphore {
    let result = Semaphore::open(
        &SEMAPHORE_NAME_C_STRING,
        OpenFlags::Create {
            exclusive: false,
            value: 0,
            mode: 0o660,
        },
    );

    let sem = match result {
        Ok(sem) => sem,
        Err(_) => {
            let errno = errno().to_string();

            match errno.as_str() {
                "EACCES" => {
                    panic!("Not enough permissions to open the semaphore at {SEMAPHORE_PATH_STR}")
                }
                _ => panic!("Unhandled sem_open() errno"),
            }
        }
    };

    ensure_file_permissions_for_front_backend_communication(
        "SemSync|>File Permissions Check",
        vec![SEMAPHORE_PATH_STR],
    );

    sem
}
