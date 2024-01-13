mod about;
mod config;
mod data;
mod info;
mod utils;

use std::env;

use libscu::utils::libc::{isatty, STDOUT_FILENO};

fn main() {
    let args = Vec::from_iter(env::args());
    let args: Vec<&str> = args.iter().map(|s|s.as_str()).collect();
    let cfg = config::Config::new();

    if args.contains(&"-v") || args.contains(&"--version") {
        about::print_version();

        std::process::exit(0);
    } else if args.contains(&"-h") || args.contains(&"--help") {
        about::print_help();

        std::process::exit(0);
    } else {
        let simplify_output = (unsafe { isatty(STDOUT_FILENO) == 0 }
            || args.contains(&"--simplify"))
            && !args.contains(&"--ignore-pipe");

        info::print_info(cfg, args.contains(&"--whale"), simplify_output, args.contains(&"--force-versions"));
    }
}