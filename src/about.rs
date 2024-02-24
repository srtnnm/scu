pub const ORG_NAME: &str = "Omnitix";
pub const SOFTWARE_NAME: &str = "scu";
pub const DESCRIPTION: &str =
    "%SOFTWARE_NAME% is a command line system info fetch utility is aimed at informativeness";
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

const FLAGS: [(&str, &str); 5] = [
    ("--simplify", "Outputs information in a much simpler form, forced by default when output is piped"),
    ("--ignore-pipe", "Outputs information in regular form, even if it's piped (disables --simplify)"),
    ("--force-versions", "Enables version fetching for WMs (it was disabled by default due to bad performance on some WMs)"),
    ("-v --version", "Print version"),
    ("-h --help", "Print this page"),
];

pub fn print_version() {
    println!("{SOFTWARE_NAME} - version {VERSION}");
}

pub fn print_help() {
    print!("[{ORG_NAME}]/");
    print_version();

    println!(
        "  {}\n",
        DESCRIPTION.replace("%SOFTWARE_NAME%", SOFTWARE_NAME)
    );

    /* show cli flags */
    if !FLAGS.is_empty() {
        println!("Flags:");
        let max_flag_len = FLAGS.iter().map(|f| f.0.chars().count()).max().unwrap_or(0);
        for flag in FLAGS {
            println!(
                "  {}{}- {}",
                flag.0,
                " ".repeat(max_flag_len - flag.0.chars().count() + 2),
                flag.1
            );
        }
    }

    std::process::exit(0)
}
