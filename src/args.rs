use crate::about;

use clap::Parser;
use libscu::util::platform::unix::libc::isatty;

#[derive(Parser, Debug)]
#[command(version, about, long_about = Some(about::DESCRIPTION))]
pub(super) struct Args {
    /// Outputs information in a much simpler form, forced by default when output is piped
    #[arg(long, default_value_t = false)]
    pub simplify: bool,

    /// Outputs information in regular form, even if it's piped (disables --simplify)
    #[arg(long, default_value_t = false)]
    pub ignore_pipe: bool,

    /// Enables version fetching for WMs (it was disabled by default due to bad performance on some WMs)
    #[arg(long, default_value_t = false)]
    pub force_versions: bool,

    /// Show raw models without processing
    #[arg(long, default_value_t = false)]
    pub raw_models: bool,
}

pub(super) fn arg_parse() -> Args {
    let mut parsed_args = Args::parse();
    parsed_args.simplify =
        !parsed_args.ignore_pipe && (unsafe { isatty(0) == 0 } || parsed_args.simplify);
    parsed_args
}
