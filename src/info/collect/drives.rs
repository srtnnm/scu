#![cfg(target_os = "linux")]

use crate::info::r#struct::Drives;

use libscu::hardware::drives;

pub fn collect() -> Drives {
    let mut result = Drives::default();

    if let Some(drives) = drives::fetch_all() {
        result.drives = drives;
    }

    result
}
