pub mod neomimic;
pub mod table;

use crate::{args::Args, config, info::SystemInformation};

#[derive(Default)]
pub enum Mode {
    #[default]
    Table,
    NeoMimic,
}

pub fn run(mode: Mode, info: &SystemInformation, config: &config::Config, args: Args) {
    match mode {
        Mode::Table => table::run(info, config, args),
        Mode::NeoMimic => neomimic::display(info, &args),
    }
}
