use crate::{config, data::table, info};

pub(super) fn collect_tables(
    info: &info::SystemInformation,
    config: &config::Config,
    simplify_output: bool,
) -> Vec<table::Table> {
    let mut result: Vec<table::Table> = Vec::new();

    for table in &config.order {
        if let Some(table) = match *table {
            #[cfg(target_os = "linux")]
            config::Table::BATTERY => super::battery::to_table(info),
            #[cfg(target_os = "linux")]
            config::Table::DISKS => super::disks::to_table(info),
            config::Table::GRAPHICS => super::graphics::to_table(info, simplify_output),
            config::Table::MEMORY => super::memory::to_table(info, simplify_output),
            config::Table::PACKAGES => super::packages::to_table(info),
            config::Table::PROCESSOR => super::processor::to_table(info, simplify_output),
            config::Table::SYSTEM => super::system::to_table(info, simplify_output),
            _ => None,
        } {
            result.push(table);
        }
    }

    result
}
