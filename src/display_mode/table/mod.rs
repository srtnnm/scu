mod collect;
pub mod config;
mod gen_table;
mod print;

// tables
mod battery;
mod disks;
mod graphics;
mod memory;
mod packages;
mod processor;
mod system;

pub(crate) fn run(config: &config::TableConfig) {
    print::print(config);
}
