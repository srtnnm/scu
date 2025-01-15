pub mod neomimic;
pub mod table;

use crate::{args::Args, config, info::SystemInformation};

#[derive(Default)]
pub enum Mode {
    #[default]
    Table,
    NeoMimic,
}

pub fn run(info: &SystemInformation, config: &config::Config, args: Args) {
    let mode = if !args.neomimic {
        Mode::default()
    } else {
        Mode::NeoMimic
    };

    match mode {
        Mode::Table => table::run(info, config, args),
        Mode::NeoMimic => neomimic::display(info, &args),
    }
}
