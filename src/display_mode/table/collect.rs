use crate::{config, data::table};

pub(super) fn collect_tables(config: &config::Config) -> Vec<table::Table> {
    let mut result: Vec<table::Table> = Vec::new();

    for table in &config.order {
        if let Some(table) = match *table {
            config::Table::BATTERY => super::battery::to_table(),
            config::Table::DISKS => super::disks::to_table(),
            config::Table::GRAPHICS => super::graphics::to_table(),
            config::Table::MEMORY => super::memory::to_table(),
            config::Table::PACKAGES => super::packages::to_table(),
            config::Table::PROCESSOR => super::processor::to_table(),
            config::Table::SYSTEM => super::system::to_table(),
            _ => None,
        } {
            result.push(table);
        }
    }

    result
}
