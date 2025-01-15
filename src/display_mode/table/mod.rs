/*
    graphics
*/

mod collect;
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

pub(crate) fn run(info: &modules::SystemInformation, config: &config::Config, args: Args) {
    print::print(info, config, &args);
}
