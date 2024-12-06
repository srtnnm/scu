/*
    graphics
*/

mod print;
mod collect;

// tables
mod disks;
mod battery;
mod memory;
mod processor;
mod system;
mod packages;
mod graphics;

use crate::{args::Args, config, info};

pub(crate) fn run(info: &info::SystemInformation, config: &config::Config, args: Args) {
    print::print(info, config, &args);
}
