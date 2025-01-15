mod about;
mod args;
mod config;
mod data;
mod display_mode;
mod info;
mod util;

fn main() {
    let args = args::arg_parse();
    data::set_raw_models(args.raw_models);

    let config = config::Config::load();

    let mut info = info::SystemInformation::new();
    info.fetch(&config, &args);

    let mode = if !args.neomimic {
        display_mode::Mode::default()
    } else {
        display_mode::Mode::NeoMimic
    };
    display_mode::run(mode, &info, &config, args);
}
