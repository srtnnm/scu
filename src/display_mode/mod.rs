mod neomimic;
mod table;

use neomimic::config::NeomimicConfig;

use crate::config::{self, neomimic};

#[derive(Default)]
pub enum Mode {
    #[default]
    Table,
    NeoMimic,
}

pub fn run(config: &config::Config) {
    let mode = if !neomimic() {
        Mode::default()
    } else {
        Mode::NeoMimic
    };

    match mode {
        Mode::Table => table::run(config),
        Mode::NeoMimic => neomimic::display(NeomimicConfig::default()),
    }
}
