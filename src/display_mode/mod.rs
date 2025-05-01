pub mod neomimic;
pub mod table;

use crate::config::{self, neomimic};

use std::sync::{mpsc, Arc};

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

#[derive(Clone)]
pub struct DisplaySender<Index, T2> {
    index: Index,
    sender: Arc<mpsc::Sender<(Index, T2)>>,
}
impl<Index: Copy, T2> DisplaySender<Index, T2> {
    pub fn new(index: Index, sender: Arc<mpsc::Sender<(Index, T2)>>) -> Self {
        Self { index, sender }
    }
    pub fn send(&self, data: T2) {
        let _ = self.sender.send((self.index, data));
    }
}
