use crate::about;

use libscu::util::platform::unix::libc::isatty;

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
}

fn version() {
    println!("{} v{}", about::BIN_NAME, about::VERSION);
    println!("libscu v{}", libscu::VERSION);

    std::process::exit(0);
}

const ARGS_WITH_DESCRIPTION: [(&str,&str);6] = [
    ("--simplify","Outputs information in a much simpler form, forced by default when output is piped"),
    ("--ignore-pipe", "Outputs information in regular form, even if it's piped (disables --simplify)"),
    ("--force-versions", "Enables version fetching for WMs (it was disabled by default due to bad performance on some WMs)"),
    ("--raw-models", "Show raw models without processing"),
    ("--version", "Print version and exit."),
    ("--help", "Print this page and exit"),
];

fn help() {
    println!("{}", about::DESCRIPTION);

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

    args.simplify = !args.ignore_pipe && (unsafe { isatty(0) == 0 } || args.simplify);
    args
}