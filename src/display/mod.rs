mod modes;

use crate::{config, info};

#[derive(Default)]
pub enum Mode {
    JSON,
    #[default]
    Table,
    TUI,
}

pub fn run(
    mode: Mode,
    info: &info::SystemInformation,
    config: &config::Config,
    simplify_output: bool,
) {
    match mode {
        Mode::Table => modes::table::run(info, config, simplify_output),
        _ => {}
    }
}
