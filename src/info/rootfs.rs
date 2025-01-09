#![cfg(any(target_os = "linux", target_os = "android"))] // REMOVE THIS TOO

use libscu::software::mounts;

pub(crate) fn get_rootfs_fstype() -> Option<String> {
    // ADD DEBUGGING FUNCTIONAL
    mounts::get_mountpoint_fstype("/")
        .ok()
        .and_then(mounts::fstype_to_string)
        .map(String::from)
}
