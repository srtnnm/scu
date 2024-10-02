#![cfg(target_os = "linux")]

use crate::info::r#struct::Disks;

use libscu::hardware::disk;

pub fn collect() -> Disks {
    let mut result = Disks::default();

    if let Ok(disks) = disk::fetch_all() {
        result.disks = disks;
    }

    result
}
