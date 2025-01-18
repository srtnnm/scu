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

use crate::{args::Args, config, modules};

pub(crate) fn run(config: &config::Config, args: Args) {
    print::print(config, &args);
}
