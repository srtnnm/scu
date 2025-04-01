mod neomimic;
mod table;

use neomimic::config::NeomimicConfig;
use table::config::TableConfig;

use crate::config::neomimic;

#[derive(Default)]
pub enum Mode {
    #[default]
    Table,
    NeoMimic,
}

pub fn run() {
    let mode = if !neomimic() {
        Mode::default()
    } else {
        Mode::NeoMimic
    };

    match mode {
        Mode::Table => table::run(TableConfig::default()),
        Mode::NeoMimic => neomimic::display(NeomimicConfig::default()),
    }
}
