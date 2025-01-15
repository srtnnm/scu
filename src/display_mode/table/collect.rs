use crate::{args::Args, config, data::table, info};

pub(super) fn collect_tables(
    info: &info::SystemInformation,
    config: &config::Config,
    args: &Args,
) -> Vec<table::Table> {
    let mut result: Vec<table::Table> = Vec::new();

    for table in &config.order {
        if let Some(table) = match *table {
            config::Table::BATTERY => super::battery::to_table(info),
            config::Table::DISKS => super::disks::to_table(info),
            config::Table::GRAPHICS => super::graphics::to_table(info, args.simplify),
            config::Table::MEMORY => super::memory::to_table(info, args.simplify),
            config::Table::PACKAGES => super::packages::to_table(info),
            config::Table::PROCESSOR => {
                super::processor::to_table(info, args.simplify, args.multicpu)
            }
            config::Table::SYSTEM => super::system::to_table(info, args.simplify),
            _ => None,
        } {
            result.push(table);
        }
    }

    result
}
