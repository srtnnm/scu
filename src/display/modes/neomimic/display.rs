use super::color_blocks;
use super::logo::print_logo;
use super::modules::{
    Header, Host, Kernel, Memory, Module, Packages, Shell, Terminal, Uptime, CPU, DE, GPU, OS, WM,
};
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
    rows.push(Uptime::get(info).unwrap());
    rows.push(Packages::get(info).unwrap());
    rows.push(Shell::get(info).unwrap());
    rows.push(DE::get(info).unwrap());
    rows.push(WM::get(info).unwrap());
    rows.push(Terminal::get(info).unwrap());
    rows.push(CPU::get(info).unwrap());
    rows.push(GPU::get(info).unwrap());
    rows.push(Memory::get(info).unwrap());

    print_logo();

    for row in rows {
        let row = row.to_string();
        println!("{}", row);
        LAST_ROW_LENGTH.store(row.chars().count(), std::sync::atomic::Ordering::Relaxed);
    }

    color_blocks::print();
}
