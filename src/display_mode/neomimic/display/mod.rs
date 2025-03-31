mod hardware;
mod misc;
pub use misc::*;
mod software;

use super::{color_blocks, logo::print_logo, modules::run_module};

use crate::{
    config::simplify,
    display_mode::neomimic::logo::{TUX_HEIGHT, TUX_WIDTH},
    modules::Module,
};

use std::sync::atomic::AtomicUsize;

pub(super) static LAST_ROW_LENGTH: AtomicUsize = AtomicUsize::new(0);

// TODO: move to non-hardcoded config
const FULL_CONFIG: [Module; 24] = [
    Module::Header,
    Module::Separator,
    Module::Username,
    Module::Hostname,
    Module::Arch,
    Module::OS,
    Module::Device,
    Module::Kernel,
    Module::Uptime,
    Module::Init,
    Module::Packages,
    Module::Shell,
    Module::DE,
    Module::WM,
    Module::DisplayServer,
    Module::Brightness,
    Module::Terminal,
    Module::CPU,
    Module::GPU,
    Module::Memory,
    Module::RootFS,
    Module::Disks,
    Module::Battery,
    Module::Locale,
];

static CURSOR_MOVER: std::sync::OnceLock<&'static str> = std::sync::OnceLock::new();

pub fn cursor_mover() {
    print!("{}", CURSOR_MOVER.get_or_init(|| ""));
}

pub fn display() {
    CURSOR_MOVER
        .set(if !simplify() {
            Box::leak(format!("\x1b[{}C", TUX_WIDTH + 4).into_boxed_str())
        } else {
            "".into()
        })
        .expect("attempted to set already initialized cursor mover");

    if !simplify() {
        // Display logo
        print_logo();
        // Return cursor to start
        println!("\x1b[{}A\x1b[9999999D", TUX_HEIGHT + 1);
    }

    for module in FULL_CONFIG {
        if let Some(len) = run_module(&module) {
            if len > 0 {
                LAST_ROW_LENGTH.store(len, std::sync::atomic::Ordering::Relaxed);
            }
        }
    }

    // Display color blocks
    if !simplify() {
        color_blocks::print();
    }
}

pub trait Display: crate::modules::Detection {
    fn run(&self) -> std::io::Result<usize> {
        Self::display(self.fetch()?)
    }
    fn display(data: Self::Result) -> std::io::Result<usize>;
}
