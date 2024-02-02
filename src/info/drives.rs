use crate::data::table::*;

use libscu::hardware::drives;
use libscu::utils::converter;

fn drive_size_to_string(size: converter::MemorySize) -> String {
    let mut _size: f64 = 0_f64;
    let mut suffix = "";
    if size.gb == 0_f64 {
        _size = size.mb as f64;
        suffix = "MiB";
    } else if size.gb < 1024_f64 {
        _size = size.gb as f64;
        suffix = "GiB";
    } else if size.gb > 1024_f64 {
        _size = size.gb as f64 / 1024_f64;
        suffix = "TiB";
    }

    format!("{:.1}{}", _size, suffix)
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
