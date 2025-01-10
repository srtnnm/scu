use super::modules::{Header, Host, Kernel, Module, OS};
use super::row::DataRow;

use std::sync::atomic::AtomicUsize;

pub(super) static LAST_ROW_LENGTH: AtomicUsize = AtomicUsize::new(0);

pub fn display(info: &crate::info::SystemInformation) {
    let mut rows: Vec<DataRow> = Vec::new();

    rows.push(Header::get(info).unwrap());
    rows.push(DataRow::separator('-'));
    rows.push(OS::get(info).unwrap());
    rows.push(Host::get(info).unwrap());
    rows.push(Kernel::get(info).unwrap());

    for row in rows {
        let row = row.to_string();
        println!("{}", row);
        LAST_ROW_LENGTH.store(row.chars().count(), std::sync::atomic::Ordering::Relaxed);
    }
}
