pub mod neomimic;
pub mod table;

use crate::config::{self, neomimic};

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
        Mode::Table => table::run(config::TABLE_CONFIG.get_or_init(|| Default::default())),
        Mode::NeoMimic => neomimic::display(
            config::NEOMIMIC_CONFIG
                .get_or_init(|| Default::default())
                .clone(),
        ),
    }
}
