mod config;
mod data;
mod info;
mod utils;

use std::env;

use scu::utils::libc::{isatty, STDOUT_FILENO};

fn main() {
    let args: Vec<String> = env::args().collect();
    let cfg = config::Config::new();

    let simplify_output =
        (unsafe { isatty(STDOUT_FILENO) == 0 } || args.contains(&"--simplify".to_string())) && !args.contains(&"--ignore-pipe".to_string());

    info::print_info(cfg, args.contains(&"--whale".to_string()), simplify_output);
}
