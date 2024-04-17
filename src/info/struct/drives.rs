#![cfg(target_os = "linux")]

use crate::data::table::Table;

use libscu::{hardware::drives::Drive, utils::converter::MemorySize};

#[derive(Default)]
pub struct Drives {
    pub drives: Vec<Drive>,
}

fn size_to_string(size: MemorySize) -> String {
    if size.gb == 0_f64 {
        format!("{:.1}MiB", size.mb as f64)
    } else if size.gb < 1024_f64 {
        format!("{:.1}GiB", size.gb as f64)
    } else {
        // So size.gb is more then 1024GiB (TiB)
        format!("{:.1}TiB", size.gb as f64 / 1024_f64)
    }
}

impl Drives {
    pub fn to_print(&self) -> Table {
        let mut result = Table::new("Drives");

        for drive in self.drives.clone() {
            result.add(
                &drive.model,
                format!("{} [{:?}]", size_to_string(drive.size), drive.technology).as_str(),
            )
        }

        result
    }
}
