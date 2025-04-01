use std::path::Path;

mod about;
mod args;
mod config;
mod data;
mod display_mode;
mod modules;
mod util;

fn main() {
    let args = args::arg_parse();
    config::set(config::ConfigData::RawModels, args.raw_models);
    config::set(config::ConfigData::Simplify, args.simplify);
    config::set(config::ConfigData::Multicpu, args.multicpu);
    config::set(config::ConfigData::Neomimic, args.neomimic);
    config::set(config::ConfigData::ForceVersions, args.force_versions);

    config::Config::parse(Path::new("./config/default.json")).unwrap();

    // let config = config::Config::load();

    display_mode::run();
}
