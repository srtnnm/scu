#![cfg(target_os = "linux")]

use crate::data::table::Table;

use libscu::{hardware::disk::Disk, types::Memory as MemorySize};

#[derive(Default)]
pub struct Disks {
    pub disks: Vec<Disk>,
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

impl Disks {
    pub fn to_print(&self) -> Table {
        let mut result = Table::new("Drives");

        for disk in self.disks.clone() {
            result.add(
                &disk.model.unwrap_or("unknown model".to_string()),
                format!("{} [{:?}]", size_to_string(disk.size), disk.technology).as_str(),
            )
        }

        result
    }
}
