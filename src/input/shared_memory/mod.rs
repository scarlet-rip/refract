mod reader;
mod writer;

pub use self::{reader::SharedMemoryReader, writer::SharedMemoryWriter};

use bytecheck::CheckBytes;
use file_owner::PathExt;
use mmap_sync::synchronizer::SynchronizerError;
use rkyv::{Archive, Deserialize, Serialize};
use sem_safe::named::{OpenFlags, Semaphore};
use std::ffi::{CString, OsString};
use std::fs;
use std::os::unix::fs::PermissionsExt;

const SHARED_MEMORY_FILE_PATH: &str = "/dev/shm/refract-sm";
const SEMAPHORE_PATH_STR: &str = "/dev/shm/sem.refract-sem";

lazy_static::lazy_static! {
    pub static ref SEMAPHORE_NAME_C_STRING: CString =  CString::new("/refract-sem").expect("Failed to name semaphore");
    pub static ref SHARED_MEMORY_FILE_PATH_OS_STR: OsString = OsString::from(SHARED_MEMORY_FILE_PATH.to_string());
    pub static ref SEMAPHORE: Semaphore = initalize_read_sync_semaphore();
}

use miette::Diagnostic;

#[derive(Debug, thiserror::Error, Diagnostic)]
pub enum SemSyncError {
    #[error("wait() failed on Semaphore")]
    #[diagnostic(severity(Error))]
    Wait,

    #[error("post() failed on Semaphore")]
    #[diagnostic(severity(Error))]
    Post,
}

#[derive(Debug, thiserror::Error, Diagnostic)]
pub enum SharedMemoryError {
    #[error(transparent)]
    #[diagnostic(severity(Error))]
    Write(SynchronizerError),

    #[error(transparent)]
    #[diagnostic(severity(Error))]
    Read(SynchronizerError),

    #[error(transparent)]
    #[diagnostic(transparent)]
    SemSync(#[from] SemSyncError),
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

        setup_file_permissions_for_front_backend(
            "SemSync|>File Permissions Check",
            vec![SEMAPHORE_PATH_STR],
        );

        semaphore
    }
}

pub fn setup_file_permissions_for_front_backend(panic_identifier: &str, file_paths: Vec<&str>) {
    const DESIRED_MODE: u32 = 0o660;
    const DESIRED_GROUP: &str = "refract";
    const DESIRED_OWNER: &str = "refract";

    for path in file_paths {
        set_file_mode_if_different(path, DESIRED_MODE, panic_identifier);
        set_file_owner_if_different(path, DESIRED_OWNER, panic_identifier);
        set_file_group_if_different(path, DESIRED_GROUP, panic_identifier);
    }
}

fn set_file_mode_if_different(path: &str, desired_mode: u32, panic_identifier: &str) {
    let metadata = fs::metadata(path)
        .unwrap_or_else(|_| panic!("{panic_identifier} -> Failed to get the metadata of [{path}]"));

    let current_permissions = metadata.permissions().mode() & 0o777; // Plain permissions only

    if current_permissions != desired_mode {
        let mut permissions = metadata.permissions();

        permissions.set_mode(desired_mode);

        let result = fs::set_permissions(path, permissions);

        if let Err(error) = result {
            panic!("{panic_identifier} -> Failed to change file permissions to [{desired_mode}] from [{current_permissions}] at [{path}], ERROR: {error:#?}");
        }
    }
}

fn set_file_owner_if_different(path: &str, desired_owner: &str, panic_identifier: &str) {
    if path
        .owner()
        .ok()
        .and_then(|o| o.name().ok().and_then(|name| name))
        .map_or(true, |name| name != desired_owner)
    {
        path
            .set_owner(desired_owner)
            .unwrap_or_else(|_| panic!("[{panic_identifier}] -> Failed to change file owner, make sure [{path}] is owned by {desired_owner}"));
    }
}

fn set_file_group_if_different(path: &str, desired_group: &str, panic_identifier: &str) {
    if path
        .group()
        .ok()
        .and_then(|o| o.name().ok().and_then(|name| name))
        .map_or(true, |name| name != desired_group)
    {
        path
            .set_group(desired_group)
            .unwrap_or_else(|_| panic!("[{panic_identifier}] -> Failed to change file group, make sure [{path}] is owned by {desired_group} group"));
    }
}
