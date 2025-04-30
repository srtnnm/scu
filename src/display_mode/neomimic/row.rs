use super::display::RowSenderT;

use std::sync::atomic::{AtomicUsize, Ordering};

// A crutch code to fix logo under shell command line when printed info is lower than logo height
pub(super) static NUMBER_OF_ROWS_PRINTED: AtomicUsize = AtomicUsize::new(0);
pub(super) fn plus_n_lines(n: usize) {
    NUMBER_OF_ROWS_PRINTED.store(
        NUMBER_OF_ROWS_PRINTED.load(Ordering::Relaxed) + n,
        Ordering::Relaxed,
    );
}
fn plus_one_line() {
    plus_n_lines(1)
}

pub(crate) struct DataRow;

impl DataRow {
    pub fn info(name: &str, value: &str, sender: &RowSenderT) {
        sender.send_row(name.to_string() + ": " + value);
        plus_one_line();
    }
    pub fn nameless(value: &str, sender: &RowSenderT) {
        sender.send_row(value.to_string());
        plus_one_line();
    }
}
