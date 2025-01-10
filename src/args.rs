use crate::about;

use libscu::software::terminal::output_is_piped;

#[derive(Default)]
pub(super) struct Args {
    // Outputs information in a much simpler form, forced by default when output is piped
    pub simplify: bool,

    // Outputs information in regular form, even if it's piped (disables --simplify)
    pub ignore_pipe: bool,

    // Enables version fetching for WMs (it was disabled by default due to bad performance on some WMs)
    pub force_versions: bool,

    // Show raw models without processing
    pub raw_models: bool,

    // Show multiple cpus instead of single cpu (UNSTABLE!)
    pub multicpu: bool,

    // Mimic the legendary neofetch (forces --raw-models)
    pub neomimic: bool,
}

fn version() {
    println!("{} v{}", about::BIN_NAME, about::VERSION);
    println!("libscu v{}", libscu::VERSION);

    std::process::exit(0);
}

const ARGS_WITH_DESCRIPTION: [(&str,&str); 8] = [
    ("--simplify","Outputs information in a much simpler form, forced by default when output is piped."),
    ("--ignore-pipe", "Outputs information in regular form, even if it's piped. (disables --simplify)"),
    ("--force-versions", "Enables version fetching for WMs. (it was disabled by default due to bad performance on some WMs)"),
    ("--raw-models", "Show raw models without processing."),
    ("--multicpu", "Show multiple cpus instead of single cpu. (UNSTABLE!)"),
    ("--neomimic", "Mimic the legendary neofetch (forces --raw-models)"),
    ("--version", "Print version and exit."),
    ("--help", "Print this page and exit."),
];

fn help() {
    println!("{}", about::DESCRIPTION);
    println!("Homepage: {}", about::HOMEPAGE);
    println!("Licensed under {} license.", about::LICENSE);

    if !ARGS_WITH_DESCRIPTION.is_empty() {
        println!("\nOptions:");
        let max_option_name = ARGS_WITH_DESCRIPTION
            .iter()
            .map(|(option, _)| option.len())
            .max()
            .unwrap_or_default();
        for (option, description) in ARGS_WITH_DESCRIPTION {
            println!(
                "    {option}{spacing}  {description}",
                spacing = " ".repeat(max_option_name.saturating_sub(option.len()))
            )
        }
    }

    std::process::exit(0);
}

pub(super) fn arg_parse() -> Args {
    let env_args = std::env::args().collect::<Vec<String>>();
    let mut args = Args::default();

    if env_args.contains(&"-v".to_string()) || env_args.contains(&"--version".to_string()) {
        version();
    } else if env_args.contains(&"-h".to_string()) || env_args.contains(&"--help".to_string()) {
        help();
    }

    args.force_versions = env_args.contains(&"--force-versions".to_string());
    args.simplify = env_args.contains(&"--simplify".to_string());
    args.ignore_pipe = env_args.contains(&"--ignore-pipe".to_string());
    args.raw_models = env_args.contains(&"--raw-models".to_string());
    args.multicpu = env_args.contains(&"--multicpu".to_string());
    args.neomimic = env_args.contains(&"--neomimic".to_string());
    if args.neomimic {
        args.raw_models = true;
    }

    args.simplify = !args.ignore_pipe && (output_is_piped() || args.simplify);
    args
}
