mod reader;
mod sync;
mod writer;

pub use self::{reader::SharedMemoryReader, sync::SemSyncError, writer::SharedMemoryWriter};

use bytecheck::CheckBytes;
use file_owner::PathExt;
use miette::Diagnostic;
use mmap_sync::synchronizer::SynchronizerError;
use once_cell::sync::Lazy;
use rkyv::{Archive, Deserialize, Serialize};
use std::{ffi::OsString, fs, os::unix::fs::PermissionsExt, path::Path};

const SHARED_MEMORY_FILE_PATH: &str = "/dev/shm/refract-sm";
static SHARED_MEMORY_FILE_PATH_OS_STR: Lazy<OsString> =
    Lazy::new(|| OsString::from(SHARED_MEMORY_FILE_PATH.to_string()));

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

fn ensure_file_permissions_for_front_backend_communication<I, P>(
    panic_identifier: &str,
    file_paths: I,
) where
    I: IntoIterator<Item = P>,
    P: AsRef<std::path::Path>,
{
    const DESIRED_MODE: u32 = 0o660;
    const DESIRED_GROUP: &str = "refract";
    const DESIRED_OWNER: &str = "refract";

    for path in file_paths {
        let path = path.as_ref();

        if !fs::exists(path).expect("How did fs::exists fail...") {
            let _ = fs::File::create_new(path);
        }

        set_file_mode_if_different(path, DESIRED_MODE, panic_identifier);
        set_file_owner_if_different(path, DESIRED_OWNER, panic_identifier);
        set_file_group_if_different(path, DESIRED_GROUP, panic_identifier);
    }
}

fn set_file_mode_if_different(path: &Path, desired_mode: u32, panic_identifier: &str) {
    let metadata = fs::metadata(path).unwrap_or_else(|_| {
        panic!(
            "{panic_identifier} -> Failed to get the metadata of [{}]",
            path.display()
        )
    });

    let current_permissions = metadata.permissions().mode() & 0o777; // Plain permissions only

    if current_permissions != desired_mode {
        let mut permissions = metadata.permissions();

        permissions.set_mode(desired_mode);

        let result = fs::set_permissions(path, permissions);

        if let Err(error) = result {
            panic!("{} -> Failed to change file permissions to [{}] from [{current_permissions}] at [{}], ERROR: {:#?}", panic_identifier, desired_mode, path.display(), error);
        }
    }
}

fn set_file_owner_if_different(path: &Path, desired_owner: &str, panic_identifier: &str) {
    if path
        .owner()
        .ok()
        .and_then(|o| o.name().ok().and_then(|name| name))
        .map_or(true, |name| name != desired_owner)
    {
        path.set_owner(desired_owner).unwrap_or_else(|_| {
            panic!(
                "[{}] -> Failed to change file owner, make sure [{}] is owned by {}",
                panic_identifier,
                path.display(),
                desired_owner
            )
        });
    }
}

fn set_file_group_if_different(path: &Path, desired_group: &str, panic_identifier: &str) {
    if path
        .group()
        .ok()
        .and_then(|o| o.name().ok().and_then(|name| name))
        .map_or(true, |name| name != desired_group)
    {
        path.set_group(desired_group).unwrap_or_else(|_| {
            panic!(
                "[{}] -> Failed to change file group, make sure [{}] is owned by {} group",
                panic_identifier,
                path.display(),
                desired_group
            )
        });
    }
}
