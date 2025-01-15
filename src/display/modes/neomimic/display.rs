use super::{
    color_blocks,
    logo::print_logo,
    modules::{run_module, Module},
};

use crate::display::modes::neomimic::logo::{TUX_HEIGHT, TUX_WIDTH};

use std::sync::atomic::AtomicUsize;

pub(super) static LAST_ROW_LENGTH: AtomicUsize = AtomicUsize::new(0);

// TODO: move to non-hardcoded config
const CONFIG: [Module; 17] = [
    Module::Header,
    Module::Separator,
    Module::OS,
    Module::Host,
    Module::Kernel,
    Module::Uptime,
    Module::Init,
    Module::Packages,
    Module::Shell,
    Module::DE,
    Module::WM,
    Module::Terminal,
    Module::CPU,
    Module::GPU,
    Module::Memory,
    Module::Locale,
    Module::Battery,
];

static CURSOR_MOVER: std::sync::OnceLock<&'static str> = std::sync::OnceLock::new();

pub fn cursor_mover() {
    print!("{}", CURSOR_MOVER.get_or_init(|| ""));
}

pub fn display(info: &crate::info::SystemInformation, args: &crate::args::Args) {
    CURSOR_MOVER
        .set(if !args.simplify {
            Box::leak(format!("\x1b[{}C", TUX_WIDTH + 4).into_boxed_str())
        } else {
            "".into()
        })
        .expect("attempted to set already initialized cursor mover");

    if !args.simplify {
        // Display logo
        print_logo();
        // Return cursor to start
        println!("\x1b[{}A\x1b[9999999D", TUX_HEIGHT + 1);
    }

    for module in CONFIG {
        if let Some(len) = run_module(&module, info) {
            if len > 0 {
                LAST_ROW_LENGTH.store(len, std::sync::atomic::Ordering::Relaxed);
            }
        }
    }

    // Display color blocks
    if !args.simplify {
        color_blocks::print();
    }
}
