#![cfg(target_os="linux")]

use crate::data::table::*;

use libscu::hardware::drives;
use libscu::utils::converter;

fn drive_size_to_string(size: converter::MemorySize) -> String {
    if size.gb == 0_f64 {
        format!("{:.1}MiB",size.mb as f64)
    } else if size.gb < 1024_f64 {
        format!("{:.1}GiB", size.gb as f64)
    } else { // So size.gb is more then 1024 (TiB)
        format!("{:.1}TiB", size.gb as f64 / 1024_f64)
    }
}

pub fn collect() -> Table {
    let mut result = Table::new("Drives");

    if let Some(drives) = drives::fetch_all() {
        for drive in drives {
            result.add(
                drive.model.as_str(),
                format!(
                    "{} [{:?}]",
                    drive_size_to_string(drive.size),
                    drive.technology
                )
                .as_str(),
            );
        }
    }

    result
}
