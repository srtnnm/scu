mod neomimic;
mod table;

use crate::{args::Args, config};

#[derive(Default)]
pub enum Mode {
    #[default]
    Table,
    NeoMimic,
}

pub fn run(config: &config::Config, args: Args) {
    let mode = if !args.neomimic {
        Mode::default()
    } else {
        Mode::NeoMimic
    };

    match mode {
        Mode::Table => table::run(config, args),
        Mode::NeoMimic => neomimic::display(&args),
    }
}
