mod about;
mod args;
mod config;
mod data;
mod display;
mod info;
mod util;

fn main() {
    let args = args::arg_parse();
    data::set_raw_models(args.raw_models);

    let config = config::Config::load();

    let mut info = info::SystemInformation::new();
    info.fetch(&config, &args);

    let mode = if !args.neomimic {
        display::Mode::default()
    } else {
        display::Mode::NeoMimic
    };
    display::run(mode, &info, &config, args);
}
