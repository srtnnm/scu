#![cfg(target_os = "linux")]
use crate::{data::table::Table, info};

use libscu::types::Memory;

fn size_to_string(size: &Memory) -> String {
    if size.gb == 0_f64 {
        format!("{:.1}MiB", size.mb as f64)
    } else if size.gb < 1024_f64 {
        format!("{:.1}GiB", size.gb as f64)
    } else {
        // So size.gb is more then 1024GiB (TiB)
        format!("{:.1}TiB", size.gb as f64 / 1024_f64)
    }
}

pub(super) fn to_table(info: &info::SystemInformation) -> Option<Table> {
    if info.disks.is_empty() {
        return None;
    }
    let mut result = Table::new("Disks");

    for disk in info.disks.iter() {
        result.add(
            &disk.model.clone().unwrap_or("unknown model".to_string()),
            format!("{} [{:?}]", size_to_string(&disk.size), disk.technology).as_str(),
        )
    }

    Some(result)
}
