use crate::about;

use libscu::software::terminal::output_is_piped;
use simpleargs::Arg;

#[derive(Default)]
pub(super) struct Args {
    // Name or path to the config
    pub config: Option<String>,

    // Enables version fetching for WMs (it was disabled by default due to bad performance on some WMs)
    pub force_versions: bool,

    // Outputs information in regular form, even if it's piped (disables --simplify)
    pub ignore_pipe: bool,

    // Show multiple cpus instead of single cpu (UNSTABLE!)
    pub multicpu: bool,

    // Mimic the legendary neofetch
    pub neomimic: bool,

    // Disable colors in output
    pub no_colors: bool,

    // Disable logo in neomimic
    pub no_logo: bool,

    // Show raw models without processing
    pub raw_models: bool,

    // Outputs information in a much simpler form, forced by default when output is piped
    pub simplify: bool,
}

fn version() {
    println!("{} v{}", about::BIN_NAME, about::VERSION);
    println!("libscu v{}", libscu::VERSION);

    std::process::exit(0);
}

const ARGS_WITH_DESCRIPTION: [(&str,&str); 11] = [
    ("--config", "Name or path to the config"),
    ("--force-versions", "Enables version fetching for WMs. (it was disabled by default due to bad performance on some WMs)"),
    ("--ignore-pipe", "Outputs information in regular form, even if it's piped. (disables --simplify)"),
    ("--multicpu", "Show multiple cpus instead of single cpu. (UNSTABLE!)"),
    ("--neomimic", "Mimic the legendary neofetch"),
    ("--no-colors","Disable colors in output"),
    ("--no-logo","Disable logo in neomimic"),
    ("--raw-models", "Show raw models without processing."),
    ("--simplify","Outputs information in a much simpler form, forced by default when output is piped."),
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
    let mut env_args = simpleargs::Args::from(std::env::args());
    let mut args = Args::default();

    loop {
        match env_args.next() {
            Arg::Positional(_) => {}
            Arg::Named(arg) => {
                let _ = arg.parse(|name, value| {
                    match name {
                        "v" | "version" => version(),
                        "h" | "help" => help(),
                        "force-versions" => args.force_versions = true,
                        "simplify" => args.simplify = true,
                        "ignore-pipe" => args.ignore_pipe = true,
                        "raw-models" => args.raw_models = true,
                        "multicpu" => args.multicpu = true,
                        "neomimic" => args.neomimic = true,
                        "config" => match value.as_str() {
                            Ok(config) => args.config = Some(config.to_string()),
                            Err(error) => {
                                logs::warning!("--config is a parameter, but: {error:?}");
                            }
                        },
                        _ => {}
                    };
                    Ok(())
                });
            }
            Arg::End => break,
            Arg::Error(_) => {}
        }
    }

    args.simplify = !args.ignore_pipe && (output_is_piped() || args.simplify);
    args
}
