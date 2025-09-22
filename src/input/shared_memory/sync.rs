use super::ensure_file_permissions_for_front_backend_communication;
use miette::Diagnostic;
use once_cell::sync::Lazy;
use sem_safe::named::{OpenFlags, Semaphore};
use std::ffi::CString;
use tracing::{error, instrument};

#[derive(Debug, thiserror::Error, Diagnostic)]
pub enum SemSyncError {
    #[error("wait() failed on Semaphore")]
    #[diagnostic(severity(Error))]
    Wait,

    #[error("post() failed on Semaphore")]
    #[diagnostic(severity(Error))]
    Post,
}

const SEMAPHORE_PATH_STR: &str = "/dev/shm/sem.refract-sem";
static SEMAPHORE_NAME_C_STRING: Lazy<CString> =
    Lazy::new(|| CString::new("/refract-sem").expect("Failed to name semaphore"));

#[instrument]
pub fn open_shm_sync_sem() -> Semaphore {
    if std::fs::exists(SEMAPHORE_PATH_STR).is_err() {
        error!(
            "Semaphore doesn't exist at {SEMAPHORE_PATH_STR}, make sure you run the backend first"
        );

        std::process::exit(1);
    }

    ensure_file_permissions_for_front_backend_communication(
        "SemSync|>File Permissions Check",
        vec![SEMAPHORE_PATH_STR],
    );

    create_or_open_shm_sync_sem()
}

#[instrument]
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
            use std::io::{Error, ErrorKind};

            let error = Error::last_os_error();

            match error.kind() {
                ErrorKind::PermissionDenied => {
                    error!("Not enough permissions to open the semaphore at {SEMAPHORE_PATH_STR}");

                    std::process::exit(1);
                }

                _ => panic!("Unhandled sem_open() error: {error:#?}"),
            }
        }
    };

    ensure_file_permissions_for_front_backend_communication(
        "SemSync|>File Permissions Check",
        vec![SEMAPHORE_PATH_STR],
    );

    sem
}
