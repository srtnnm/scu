mod about;
mod args;
mod config;
mod data;
mod display_mode;
mod modules;
mod util;

fn main() {
    let args = args::arg_parse();
    data::set_raw_models(args.raw_models);

    let config = config::Config::load();

    display_mode::run(&config, args);
}
