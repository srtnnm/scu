mod neomimic;
mod table;

use crate::{args::Args, config, modules::SystemInformation};

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
        Mode::NeoMimic => neomimic::display(&args),
    }
}
