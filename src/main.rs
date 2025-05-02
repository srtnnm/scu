mod about;
mod args;
mod config;
mod data;
mod display_mode;
mod modules;
mod util;

fn main() {
    let args = args::Args::arg_parse();
    args.apply_to_config();

    display_mode::run();
}
