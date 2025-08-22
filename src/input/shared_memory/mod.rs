mod reader;
mod writer;

pub use self::{reader::SharedMemoryReader, writer::SharedMemoryWriter};

use bytecheck::CheckBytes;
use rkyv::{Archive, Deserialize, Serialize};
use sem_safe::named::{OpenFlags, Semaphore};
use std::{
    ffi::{CString, OsString},
    fs::{set_permissions, Permissions},
    os::unix::fs::PermissionsExt,
    sync::atomic::AtomicBool,
};

const SHARED_MEMORY_FILE_PATH: &str = "/dev/shm/refract-sm";
static LISTENER_STARTED: AtomicBool = AtomicBool::new(false);
const SEMAPHORE_PATH_STR: &str = "/dev/shm/sem.refract-sem";

lazy_static::lazy_static! {
    pub static ref SEMAPHORE_NAME_C_STRING: CString =  CString::new("/refract-sem").expect("Failed to name semaphore");
    pub static ref SHARED_MEMORY_FILE_PATH_OS_STR: OsString = OsString::from(SHARED_MEMORY_FILE_PATH.to_string());
    pub static ref SEMAPHORE: Semaphore = initalize_read_sync_semaphore();
}

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

fn initalize_read_sync_semaphore() -> Semaphore {
    if std::fs::File::open(SEMAPHORE_PATH_STR).is_ok() {
        Semaphore::open(
            &SEMAPHORE_NAME_C_STRING,
            OpenFlags::Create {
                exclusive: false,
                value: 0,
                mode: 0o660,
            },
        )
        .expect("Failed to open semaphore")
    } else {
        let semaphore = Semaphore::open(
            &SEMAPHORE_NAME_C_STRING,
            OpenFlags::Create {
                exclusive: false,
                value: 0,
                mode: 0o660,
            },
        )
        .expect("Failed to open semaphore");

        set_permissions(SEMAPHORE_PATH_STR, Permissions::from_mode(0o660))
            .expect("Failed to set semaphore file permissions");

        semaphore
    }
}
