mod about;
mod args;
mod config;
mod data;
mod display_mode;
mod modules;
mod util;

use std::panic;
pub fn rebuild_panic_handler() {
    let orig_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        orig_hook(panic_info);
        std::process::exit(-1);
    }));
}

fn main() {
    let args = args::Args::arg_parse();
    args.apply_to_config();
    rebuild_panic_handler();

    display_mode::run();
}
