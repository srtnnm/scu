use super::modules::{Header, Module};
use super::row::DataRow;

use std::sync::atomic::AtomicUsize;

pub(super) static LAST_ROW_LENGTH: AtomicUsize = AtomicUsize::new(0);

pub fn display(info: &crate::info::SystemInformation) {
    let mut rows: Vec<DataRow> = Vec::new();

    rows.push(Header::get(info).unwrap());
    rows.push(DataRow::separator('-'));

    for row in rows {
        let row = row.to_string();
        println!("{}", row);
        LAST_ROW_LENGTH.store(row.chars().count(), std::sync::atomic::Ordering::Relaxed);
    }
}
