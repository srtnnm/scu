mod modes;

use crate::{args::Args, config, info};

#[derive(Default)]
pub enum Mode {
    #[default]
    Table,
}

pub fn run(mode: Mode, info: &info::SystemInformation, config: &config::Config, args: Args) {
    match mode {
        Mode::Table => modes::table::run(info, config, args),
    }
}
