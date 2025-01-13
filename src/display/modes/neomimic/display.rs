use super::{
    color_blocks,
    logo::print_logo,
    modules::{run_module, Module},
    row::DataRow,
};

use crate::display::modes::neomimic::logo::{TUX_HEIGHT, TUX_WIDTH};

use std::sync::atomic::AtomicUsize;

pub(super) static LAST_ROW_LENGTH: AtomicUsize = AtomicUsize::new(0);

// TODO: move to non-hardcoded config
const CONFIG: [Module; 14] = [
    Module::Header,
    Module::Separator,
    Module::OS,
    Module::Host,
    Module::Kernel,
    Module::Uptime,
    Module::Packages,
    Module::Shell,
    Module::DE,
    Module::WM,
    Module::Terminal,
    Module::CPU,
    Module::GPU,
    Module::Memory,
];

pub fn display(info: &crate::info::SystemInformation, args: &crate::args::Args) {
    let mut rows: Vec<DataRow> = Vec::new();

    for module in CONFIG {
        if let Some(row) = run_module(&module, info) {
            rows.push(row);
        }
    }

    let cursor_mover = if !args.simplify {
        format!("\x1b[{}C", TUX_WIDTH + 4)
    } else {
        "".into()
    };

    if !args.simplify {
        // Display logo
        print_logo();
        // Return cursor to start
        println!("\x1b[{}A\x1b[9999999D", TUX_HEIGHT + 1);
    }

    // Display data
    for row in rows {
        let row = row.to_string();
        println!("{cursor_mover}{}", row);
        LAST_ROW_LENGTH.store(row.chars().count(), std::sync::atomic::Ordering::Relaxed);
    }

    // Display color blocks
    if !args.simplify {
        color_blocks::print();
    }
}
