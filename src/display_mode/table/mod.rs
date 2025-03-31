mod collect;
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

use crate::config;

pub(crate) fn run(config: &config::Config) {
    print::print(config);
}
