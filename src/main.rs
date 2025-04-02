use std::path::Path;

mod about;
mod args;
mod config;
mod data;
mod display_mode;
mod modules;
mod util;

fn main() {
    let args = args::Args::arg_parse();

    config::Config::parse(Path::new("./config/default.json")).unwrap();

    // let config = config::Config::load();

    display_mode::run();
}
