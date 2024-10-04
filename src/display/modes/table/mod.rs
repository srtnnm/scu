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

use crate::{config, info};

pub(crate) fn run(info: &info::SystemInformation, config: &config::Config, simplify_output: bool) {
    print::print(info, config, simplify_output);
}
