mod hardware;
mod misc;
pub use misc::*;
mod software;

use super::{color_blocks, config::NeomimicConfig, modules::run_module};

use crate::{
    config::{no_colors, no_logo, simplify},
    display_mode::neomimic::logo::{logo_height, logo_width},
};

use std::sync::atomic::AtomicUsize;

pub(super) static LAST_ROW_LENGTH: AtomicUsize = AtomicUsize::new(0);

static CURSOR_MOVER: std::sync::OnceLock<&'static str> = std::sync::OnceLock::new();

pub fn cursor_mover() {
    print!("{}", CURSOR_MOVER.get_or_init(|| ""));
}

pub fn display(config: &NeomimicConfig) {
    if !simplify() {
        // Display logo
        config.logo.print();
        // Return cursor to start
        println!("\x1b[{}A\x1b[9999999D", logo_height());
    }

    let logo_width = logo_width();
    if !no_logo() {
        CURSOR_MOVER
            .set(if !simplify() {
                Box::leak(
                    format!("\x1b[{}C", if logo_width == 0 { 0 } else { logo_width + 4 })
                        .into_boxed_str(),
                )
            } else {
                "".into()
            })
            .expect("attempted to set already initialized cursor mover");
    }

    for module in config.modules.iter() {
        if let Some(len) = run_module(&module) {
            if len > 0 {
                LAST_ROW_LENGTH.store(len, std::sync::atomic::Ordering::Relaxed);
            }
        }
    }

    // Display color blocks
    if !no_colors() {
        color_blocks::print();
    }
}

pub trait Display: crate::modules::Detection {
    fn run(&self) -> std::io::Result<usize> {
        Self::display(self.fetch()?)
    }
    fn display(data: Self::Result) -> std::io::Result<usize>;
}
