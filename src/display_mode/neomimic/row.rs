use super::display::cursor_mover;

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
    pub fn info(name: &str, value: &str) -> usize {
        cursor_mover();
        println!("{name}: {value}");
        plus_one_line();
        name.len().saturating_add(value.len()).saturating_add(2)
    }
    pub fn nameless(value: &str) -> usize {
        cursor_mover();
        println!("{value}");
        plus_one_line();
        value.len()
    }
    pub fn separator(r#char: char) -> usize {
        cursor_mover();
        let len = super::display::LAST_ROW_LENGTH.load(Ordering::Relaxed);
        println!("{}", r#char.to_string().repeat(len));
        plus_one_line();
        len
    }
}
